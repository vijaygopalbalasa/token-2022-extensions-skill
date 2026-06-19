//! Program entrypoint.

use {
    crate::processor,
    solana_account_info::AccountInfo,
    solana_program_entrypoint::{entrypoint, ProgramResult},
    solana_pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    processor::process(program_id, accounts, instruction_data)
}
