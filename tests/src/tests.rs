use iso_20022_dsig::{dsig::*, ed25519};
use iso_20022_head::head_001_001_03::*;

#[test]
fn test_header() {
    // Import ISO 20022 Modules we need.
    let mut signature = SignatureType::ed25519_builder();

    let mut key_info = KeyInfo::ed25519_builder();
    // key_info.

    signature.key_info(Some(key_info.build().expect("Failed to build key info")));

    let signature = signature.build().expect("Failed to build signature");

    //
    let mut header =
        BusinessApplicationHeaderV03Builder::<ed25519::Ed25519, ed25519::Ed25519>::default();
}
