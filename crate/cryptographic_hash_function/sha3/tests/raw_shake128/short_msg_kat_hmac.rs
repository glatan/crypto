use std::path::Path;

use dev_util::short_msg_kat::ShortMsgKat;
use sha3::RawShake128;

#[test]
fn short_msg_kat_hmac_128() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("raw_shake128")
        .join("ShortMsgKAT_HMAC_128.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = RawShake128::new(16);
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_hmac_256() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("raw_shake128")
        .join("ShortMsgKAT_HMAC_256.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = RawShake128::new(32);
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_hmac_512() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("raw_shake128")
        .join("ShortMsgKAT_HMAC_512.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = RawShake128::new(64);
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_hmac_1024() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("raw_shake128")
        .join("ShortMsgKAT_HMAC_1024.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = RawShake128::new(128);
    kat.assert(&mut hasher);
}
