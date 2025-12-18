use std::path::Path;

use dev_util::short_msg_kat::ShortMsgKat;
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};

#[test]
fn short_msg_kat_hmac_224() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("sha3")
        .join("ShortMsgKAT_HMAC_224.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Sha3_224::new();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_hmac_256() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("sha3")
        .join("ShortMsgKAT_HMAC_256.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Sha3_256::new();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_hmac_384() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("sha3")
        .join("ShortMsgKAT_HMAC_384.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Sha3_384::new();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_hmac_512() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("sha3")
        .join("ShortMsgKAT_HMAC_512.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Sha3_512::new();
    kat.assert(&mut hasher);
}
