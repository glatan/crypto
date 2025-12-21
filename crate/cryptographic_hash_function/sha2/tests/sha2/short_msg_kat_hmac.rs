use std::path::Path;

use dev_util::short_msg_kat::ShortMsgKat;
use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512Trunc224, Sha512Trunc256};

#[test]
fn short_msg_kat_hmac_224() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("sha2")
        .join("ShortMsgKAT_HMAC_224.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Sha224::new();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_hmac_256() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("sha2")
        .join("ShortMsgKAT_HMAC_256.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Sha256::new();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_hmac_384() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("sha2")
        .join("ShortMsgKAT_HMAC_384.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Sha384::new();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_hmac_512() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("sha2")
        .join("ShortMsgKAT_HMAC_512.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Sha512::new();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_hmac_512_trunc_224() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("sha2")
        .join("ShortMsgKAT_HMAC_512_trunc_224.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Sha512Trunc224::new();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_hmac_512_trunc_256() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("sha2")
        .join("ShortMsgKAT_HMAC_512_trunc_256.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Sha512Trunc256::new();
    kat.assert(&mut hasher);
}
