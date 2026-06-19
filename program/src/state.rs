//! On-chain state: a fixed-capacity allowlist of permitted destination owners.
//!
//! Layout (353 bytes, fixed capacity = 10 entries):
//!   [0..32]    authority   (Pubkey allowed to add entries)
//!   [32]       len         (u8, number of active entries)
//!   [33..353]  entries     ([Pubkey; 10])
//!
//! A fixed capacity keeps the account size constant, so there is no reallocation
//! and no borsh/serde dependency — just deterministic byte offsets via `arrayref`.

use {
    arrayref::{array_mut_ref, array_ref, mut_array_refs},
    solana_program_error::ProgramError,
    solana_pubkey::Pubkey,
};

/// Maximum number of allowlisted owners this reference program stores.
pub const MAX_ENTRIES: usize = 10;
/// Total serialized size of the [`WhiteList`] account.
pub const WHITELIST_LEN: usize = 32 + 1 + (MAX_ENTRIES * 32); // 353

/// PDA seed prefix for the allowlist account.
pub const WHITELIST_SEED: &[u8] = b"white-list";

/// Helpers for reading/writing the raw allowlist account data in place.
pub struct WhiteList;

impl WhiteList {
    /// Initialize a freshly allocated account: set the authority, len = 0.
    pub fn init(data: &mut [u8], authority: &Pubkey) -> Result<(), ProgramError> {
        if data.len() != WHITELIST_LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        let dst = array_mut_ref![data, 0, WHITELIST_LEN];
        let (auth, len, entries) = mut_array_refs![dst, 32, 1, MAX_ENTRIES * 32];
        auth.copy_from_slice(authority.as_ref());
        len[0] = 0;
        for e in entries.iter_mut() {
            *e = 0;
        }
        Ok(())
    }

    /// Read the stored authority.
    pub fn authority(data: &[u8]) -> Result<Pubkey, ProgramError> {
        if data.len() != WHITELIST_LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        let auth = array_ref![data, 0, 32];
        Ok(Pubkey::from(*auth))
    }

    /// Return true if `key` is present in the allowlist.
    pub fn contains(data: &[u8], key: &Pubkey) -> Result<bool, ProgramError> {
        if data.len() != WHITELIST_LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        let len = data[32] as usize;
        let want = key.as_ref();
        for i in 0..len.min(MAX_ENTRIES) {
            let start = 33 + i * 32;
            if &data[start..start + 32] == want {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Append `key` to the allowlist. Errors if full or already present.
    pub fn push(data: &mut [u8], key: &Pubkey) -> Result<(), ProgramError> {
        if Self::contains(data, key)? {
            return Ok(());
        }
        let len = data[32] as usize;
        if len >= MAX_ENTRIES {
            return Err(ProgramError::InvalidArgument);
        }
        let start = 33 + len * 32;
        data[start..start + 32].copy_from_slice(key.as_ref());
        data[32] = (len + 1) as u8;
        Ok(())
    }
}
