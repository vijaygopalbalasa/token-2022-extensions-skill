//! Program processor.
//!
//! Implements the SPL transfer-hook interface (`Execute`,
//! `InitializeExtraAccountMetaList`, `UpdateExtraAccountMetaList`) and gates every
//! transfer on a destination-owner allowlist. The interface plumbing mirrors the
//! canonical SPL transfer-hook example; the allowlist check and the admin
//! instructions are this program's custom logic.

use {
    crate::{
        instruction::AllowlistInstruction,
        state::{WhiteList, WHITELIST_LEN, WHITELIST_SEED},
    },
    solana_account_info::{next_account_info, AccountInfo},
    solana_cpi::invoke_signed,
    solana_msg::msg,
    solana_program_error::{ProgramError, ProgramResult},
    solana_pubkey::Pubkey,
    solana_system_interface::instruction as system_instruction,
    spl_tlv_account_resolution::{account::ExtraAccountMeta, state::ExtraAccountMetaList},
    spl_token_2022_interface::{
        extension::{
            transfer_hook::TransferHookAccount, BaseStateWithExtensions, StateWithExtensions,
        },
        state::{Account, Mint},
    },
    spl_transfer_hook_interface::{
        collect_extra_account_metas_signer_seeds,
        error::TransferHookError,
        get_extra_account_metas_address, get_extra_account_metas_address_and_bump_seed,
        instruction::{ExecuteInstruction, TransferHookInstruction},
    },
};

fn check_token_account_is_transferring(account_info: &AccountInfo) -> Result<(), ProgramError> {
    let account_data = account_info.try_borrow_data()?;
    let token_account = StateWithExtensions::<Account>::unpack(&account_data)?;
    let extension = token_account.get_extension::<TransferHookAccount>()?;
    if bool::from(extension.transferring) {
        Ok(())
    } else {
        Err(TransferHookError::ProgramCalledOutsideOfTransfer.into())
    }
}

/// Read the owner authority of a token account.
fn token_account_owner(account_info: &AccountInfo) -> Result<Pubkey, ProgramError> {
    let data = account_info.try_borrow_data()?;
    let account = StateWithExtensions::<Account>::unpack(&data)?;
    Ok(account.base.owner)
}

/// Processes an `Execute` instruction: the transfer-hook callback.
pub fn process_execute(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let source_account_info = next_account_info(account_info_iter)?;
    let _mint_info = next_account_info(account_info_iter)?;
    let destination_account_info = next_account_info(account_info_iter)?;
    let _authority_info = next_account_info(account_info_iter)?;
    let extra_account_metas_info = next_account_info(account_info_iter)?;
    // Our single declared extra account: the allowlist PDA.
    let white_list_info = next_account_info(account_info_iter)?;

    // 1) Only run during an actual Token-2022 transfer.
    check_token_account_is_transferring(source_account_info)?;
    check_token_account_is_transferring(destination_account_info)?;

    // 2) Validate that every passed account matches the declared ExtraAccountMetaList
    //    (this also pins white_list_info to the resolved PDA).
    let mint_info = _mint_info;
    let expected_validation_address = get_extra_account_metas_address(mint_info.key, program_id);
    if expected_validation_address != *extra_account_metas_info.key {
        return Err(ProgramError::InvalidSeeds);
    }
    let data = extra_account_metas_info.try_borrow_data()?;
    ExtraAccountMetaList::check_account_infos::<ExecuteInstruction>(
        accounts,
        &TransferHookInstruction::Execute { amount }.pack(),
        program_id,
        &data,
    )?;

    // 3) Enforce the allowlist: the destination owner must be allowlisted.
    if *white_list_info.owner != *program_id {
        return Err(ProgramError::IllegalOwner);
    }
    let dest_owner = token_account_owner(destination_account_info)?;
    let wl_data = white_list_info.try_borrow_data()?;
    if !WhiteList::contains(&wl_data, &dest_owner)? {
        msg!("Transfer denied: destination owner not on allowlist");
        return Err(ProgramError::Custom(1));
    }

    Ok(())
}

/// Processes `InitializeExtraAccountMetaList`: declares our one extra account
/// (the allowlist PDA, resolved from a literal seed).
pub fn process_initialize_extra_account_meta_list(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let extra_account_metas_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;
    let _system_program_info = next_account_info(account_info_iter)?;

    // Mint authority must sign.
    let mint_data = mint_info.try_borrow_data()?;
    let mint = StateWithExtensions::<Mint>::unpack(&mint_data)?;
    let mint_authority = mint
        .base
        .mint_authority
        .ok_or(TransferHookError::MintHasNoMintAuthority)?;
    drop(mint_data);
    if !authority_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if *authority_info.key != mint_authority {
        return Err(TransferHookError::IncorrectMintAuthority.into());
    }

    // Declare the single extra account: the allowlist PDA (literal seed).
    let white_list_meta =
        ExtraAccountMeta::new_with_seeds(&[Seed::Literal {
            bytes: WHITELIST_SEED.to_vec(),
        }], false, false)?;
    let extra_account_metas = [white_list_meta];

    // Validation PDA + create + write.
    let (expected_validation_address, bump_seed) =
        get_extra_account_metas_address_and_bump_seed(mint_info.key, program_id);
    if expected_validation_address != *extra_account_metas_info.key {
        return Err(ProgramError::InvalidSeeds);
    }
    let bump_seed = [bump_seed];
    let signer_seeds = collect_extra_account_metas_signer_seeds(mint_info.key, &bump_seed);
    let account_size = ExtraAccountMetaList::size_of(extra_account_metas.len())?;
    invoke_signed(
        &system_instruction::allocate(extra_account_metas_info.key, account_size as u64),
        core::slice::from_ref(extra_account_metas_info),
        &[&signer_seeds],
    )?;
    invoke_signed(
        &system_instruction::assign(extra_account_metas_info.key, program_id),
        core::slice::from_ref(extra_account_metas_info),
        &[&signer_seeds],
    )?;
    let mut data = extra_account_metas_info.try_borrow_mut_data()?;
    ExtraAccountMetaList::init::<ExecuteInstruction>(&mut data, &extra_account_metas)?;

    Ok(())
}

/// Processes `InitializeWhiteList` (custom): allocate + init the allowlist PDA.
/// The account must be pre-funded (rent-exempt) by the client.
pub fn process_initialize_white_list(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let white_list_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;
    // System program must be present so the allocate/assign CPIs can resolve it.
    let _system_program_info = next_account_info(account_info_iter)?;

    if !authority_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (expected, bump) = Pubkey::find_program_address(&[WHITELIST_SEED], program_id);
    if expected != *white_list_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    let signer_seeds: &[&[u8]] = &[WHITELIST_SEED, &[bump]];
    invoke_signed(
        &system_instruction::allocate(white_list_info.key, WHITELIST_LEN as u64),
        core::slice::from_ref(white_list_info),
        &[signer_seeds],
    )?;
    invoke_signed(
        &system_instruction::assign(white_list_info.key, program_id),
        core::slice::from_ref(white_list_info),
        &[signer_seeds],
    )?;

    let mut data = white_list_info.try_borrow_mut_data()?;
    WhiteList::init(&mut data, authority_info.key)?;
    Ok(())
}

/// Processes `AddToWhiteList` (custom): authority appends an owner.
pub fn process_add_to_white_list(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    address: &Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let white_list_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;

    if *white_list_info.owner != *program_id {
        return Err(ProgramError::IllegalOwner);
    }
    if !authority_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    let mut data = white_list_info.try_borrow_mut_data()?;
    let stored_authority = WhiteList::authority(&data)?;
    if stored_authority != *authority_info.key {
        return Err(ProgramError::IllegalOwner);
    }
    WhiteList::push(&mut data, address)?;
    Ok(())
}

/// Top-level dispatch: interface instructions first, then custom admin ones.
pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    if let Ok(instruction) = TransferHookInstruction::unpack(input) {
        return match instruction {
            TransferHookInstruction::Execute { amount } => {
                msg!("Instruction: Execute");
                process_execute(program_id, accounts, amount)
            }
            TransferHookInstruction::InitializeExtraAccountMetaList { .. } => {
                msg!("Instruction: InitializeExtraAccountMetaList");
                process_initialize_extra_account_meta_list(program_id, accounts)
            }
            TransferHookInstruction::UpdateExtraAccountMetaList { .. } => {
                msg!("Instruction: UpdateExtraAccountMetaList (unsupported)");
                Err(ProgramError::InvalidInstructionData)
            }
        };
    }

    match AllowlistInstruction::unpack(input)? {
        AllowlistInstruction::InitializeWhiteList => {
            msg!("Instruction: InitializeWhiteList");
            process_initialize_white_list(program_id, accounts)
        }
        AllowlistInstruction::AddToWhiteList { address } => {
            msg!("Instruction: AddToWhiteList");
            process_add_to_white_list(program_id, accounts, &address)
        }
        AllowlistInstruction::InitializeExtraAccountMetaList => {
            msg!("Instruction: InitializeExtraAccountMetaList (custom)");
            process_initialize_extra_account_meta_list(program_id, accounts)
        }
    }
}

// Re-export for the meta declaration above.
use spl_tlv_account_resolution::seeds::Seed;
