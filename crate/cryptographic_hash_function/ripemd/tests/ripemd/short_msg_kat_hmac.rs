use std::path::Path;

use dev_util::short_msg_kat::ShortMsgKat;
use ripemd::{Ripemd, Ripemd128, Ripemd160, Ripemd256, Ripemd320};

#[test]
fn short_msg_kat_hmac() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("ripemd")
        .join("ShortMsgKAT_HMAC.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Ripemd::new();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_hmac_128() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("ripemd")
        .join("ShortMsgKAT_HMAC_128.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Ripemd128::new();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_hmac_160() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("ripemd")
        .join("ShortMsgKAT_HMAC_160.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Ripemd160::new();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_hmac_256() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("ripemd")
        .join("ShortMsgKAT_HMAC_256.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Ripemd256::new();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_hmac_320() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("ripemd")
        .join("ShortMsgKAT_HMAC_320.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Ripemd320::new();
    kat.assert(&mut hasher);
}
