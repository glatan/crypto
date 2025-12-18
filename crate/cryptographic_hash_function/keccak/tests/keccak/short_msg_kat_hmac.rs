use std::path::Path;

use dev_util::short_msg_kat::ShortMsgKat;
use keccak::{Keccak224, Keccak256, Keccak384, Keccak512};

#[test]
fn short_msg_kat_hmac_224() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("keccak")
        .join("ShortMsgKAT_HMAC_224.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Keccak224::new();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_hmac_256() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("keccak")
        .join("ShortMsgKAT_HMAC_256.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Keccak256::new();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_hmac_384() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("keccak")
        .join("ShortMsgKAT_HMAC_384.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Keccak384::new();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_hmac_512() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("keccak")
        .join("ShortMsgKAT_HMAC_512.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Keccak512::new();
    kat.assert(&mut hasher);
}
