//! Unit tests for the allowlist data logic and instruction (de)serialization.
//!
//! These run on the host with plain `cargo test` — no SBF, no validator. They
//! test the security-critical pieces directly: allowlist membership/capacity/
//! dedup, authority storage, and instruction pack/unpack round-trips.
//!
//! End-to-end transfer tests (a real Token-2022 transfer routed through the
//! deployed hook) live separately and are described in `program/README.md`.

use {
    solana_pubkey::Pubkey,
    transfer_hook_allowlist::{
        instruction::{AllowlistInstruction, TAG_ADD, TAG_INITIALIZE},
        state::{WhiteList, MAX_ENTRIES, WHITELIST_LEN},
    },
};

fn key(b: u8) -> Pubkey {
    Pubkey::from([b; 32])
}

#[test]
fn whitelist_init_sets_authority_and_empty() {
    let mut data = vec![0u8; WHITELIST_LEN];
    let auth = key(7);
    WhiteList::init(&mut data, &auth).unwrap();
    assert_eq!(WhiteList::authority(&data).unwrap(), auth);
    assert!(!WhiteList::contains(&data, &key(1)).unwrap());
}

#[test]
fn whitelist_push_and_contains() {
    let mut data = vec![0u8; WHITELIST_LEN];
    WhiteList::init(&mut data, &key(7)).unwrap();

    let allowed = key(42);
    assert!(!WhiteList::contains(&data, &allowed).unwrap());
    WhiteList::push(&mut data, &allowed).unwrap();
    assert!(WhiteList::contains(&data, &allowed).unwrap());
    // a different key is still not present
    assert!(!WhiteList::contains(&data, &key(43)).unwrap());
}

#[test]
fn whitelist_push_is_idempotent() {
    let mut data = vec![0u8; WHITELIST_LEN];
    WhiteList::init(&mut data, &key(7)).unwrap();
    let k = key(9);
    WhiteList::push(&mut data, &k).unwrap();
    WhiteList::push(&mut data, &k).unwrap(); // no-op, must not grow len
    assert_eq!(data[32], 1, "duplicate push must not increment len");
}

#[test]
fn whitelist_respects_capacity() {
    let mut data = vec![0u8; WHITELIST_LEN];
    WhiteList::init(&mut data, &key(7)).unwrap();
    for i in 0..MAX_ENTRIES as u8 {
        WhiteList::push(&mut data, &key(100 + i)).unwrap();
    }
    assert_eq!(data[32] as usize, MAX_ENTRIES);
    // one past capacity must error
    assert!(WhiteList::push(&mut data, &key(250)).is_err());
}

#[test]
fn whitelist_rejects_wrong_size() {
    let mut data = vec![0u8; WHITELIST_LEN - 1];
    assert!(WhiteList::init(&mut data, &key(1)).is_err());
    assert!(WhiteList::contains(&data, &key(1)).is_err());
}

#[test]
fn instruction_initialize_roundtrip() {
    let packed = AllowlistInstruction::InitializeWhiteList.pack();
    assert_eq!(packed, vec![TAG_INITIALIZE]);
    match AllowlistInstruction::unpack(&packed).unwrap() {
        AllowlistInstruction::InitializeWhiteList => {}
        _ => panic!("wrong variant"),
    }
}

#[test]
fn instruction_init_meta_roundtrip() {
    let packed = AllowlistInstruction::InitializeExtraAccountMetaList.pack();
    assert_eq!(packed, vec![2]);
    match AllowlistInstruction::unpack(&packed).unwrap() {
        AllowlistInstruction::InitializeExtraAccountMetaList => {}
        _ => panic!("wrong variant"),
    }
}

#[test]
fn instruction_add_roundtrip() {
    let addr = key(55);
    let packed = AllowlistInstruction::AddToWhiteList { address: addr }.pack();
    assert_eq!(packed[0], TAG_ADD);
    assert_eq!(packed.len(), 33);
    match AllowlistInstruction::unpack(&packed).unwrap() {
        AllowlistInstruction::AddToWhiteList { address } => assert_eq!(address, addr),
        _ => panic!("wrong variant"),
    }
}

#[test]
fn instruction_add_rejects_bad_length() {
    // tag for ADD but missing the 32-byte address
    assert!(AllowlistInstruction::unpack(&[TAG_ADD, 1, 2, 3]).is_err());
}

#[test]
fn instruction_rejects_unknown_tag_and_empty() {
    assert!(AllowlistInstruction::unpack(&[99]).is_err());
    assert!(AllowlistInstruction::unpack(&[]).is_err());
}
