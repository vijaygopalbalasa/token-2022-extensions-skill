//! Custom (non-interface) instructions for the allowlist hook.
//!
//! The transfer-hook *interface* instructions (`Execute`,
//! `InitializeExtraAccountMetaList`, `UpdateExtraAccountMetaList`) are decoded by
//! `spl_transfer_hook_interface::instruction::TransferHookInstruction` and use
//! 8-byte SPL discriminators. Our admin instructions below use a single leading
//! tag byte and are only parsed when the interface decode fails — the two
//! namespaces cannot collide.

use {solana_program_error::ProgramError, solana_pubkey::Pubkey};

/// Admin instructions for managing the allowlist.
pub enum AllowlistInstruction {
    /// Create + initialize the allowlist PDA. The account must be pre-funded
    /// (rent-exempt) by the client before this call.
    ///
    /// Accounts:
    ///   0. `[writable]` white_list PDA  (seeds: ["white-list"])
    ///   1. `[signer]`   authority       (stored as the allowlist admin)
    InitializeWhiteList,

    /// Append an owner to the allowlist (authority only).
    ///
    /// Accounts:
    ///   0. `[writable]` white_list PDA
    ///   1. `[signer]`   authority
    AddToWhiteList { address: Pubkey },

    /// Create + initialize the `ExtraAccountMetaList` PDA for this mint.
    /// Equivalent to the interface `InitializeExtraAccountMetaList`, exposed as a
    /// simple custom instruction so clients don't need to encode the interface's
    /// `Vec<ExtraAccountMeta>` (this program declares its own metas).
    ///
    /// Accounts:
    ///   0. `[writable]` extra_account_meta_list PDA (seeds: ["extra-account-metas", mint])
    ///   1. `[]`         mint
    ///   2. `[signer]`   mint authority
    ///   3. `[]`         system program
    InitializeExtraAccountMetaList,
}

/// Tag bytes for the custom instructions.
pub const TAG_INITIALIZE: u8 = 0;
pub const TAG_ADD: u8 = 1;
pub const TAG_INIT_META: u8 = 2;

impl AllowlistInstruction {
    /// Decode a custom instruction from raw input.
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        match tag {
            TAG_INITIALIZE => Ok(Self::InitializeWhiteList),
            TAG_INIT_META => Ok(Self::InitializeExtraAccountMetaList),
            TAG_ADD => {
                if rest.len() != 32 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let mut buf = [0u8; 32];
                buf.copy_from_slice(rest);
                Ok(Self::AddToWhiteList {
                    address: Pubkey::from(buf),
                })
            }
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }

    /// Encode this instruction's data (used by clients/tests).
    pub fn pack(&self) -> Vec<u8> {
        match self {
            Self::InitializeWhiteList => vec![TAG_INITIALIZE],
            Self::InitializeExtraAccountMetaList => vec![TAG_INIT_META],
            Self::AddToWhiteList { address } => {
                let mut v = Vec::with_capacity(33);
                v.push(TAG_ADD);
                v.extend_from_slice(address.as_ref());
                v
            }
        }
    }
}
