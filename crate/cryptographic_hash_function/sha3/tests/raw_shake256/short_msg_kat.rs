use std::path::Path;

use dev_util::short_msg_kat::ShortMsgKat;
use sha3::RawShake256;

#[test]
fn short_msg_kat_128() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("raw_shake256")
        .join("ShortMsgKAT_128.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = RawShake256::new(16);
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_256() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("raw_shake256")
        .join("ShortMsgKAT_256.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = RawShake256::new(32);
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_512() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("raw_shake256")
        .join("ShortMsgKAT_512.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = RawShake256::new(64);
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_1024() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("raw_shake256")
        .join("ShortMsgKAT_1024.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = RawShake256::new(128);
    kat.assert(&mut hasher);
}
