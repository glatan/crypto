use std::path::Path;

use blake::{Blake224, Blake256, Blake384, Blake512};
use dev_util::short_msg_kat::ShortMsgKatHmac;

#[test]
fn short_msg_kat_hmac_224() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("blake")
        .join("round2")
        .join("ShortMsgKAT_HMAC_224.txt");
    let kat = ShortMsgKatHmac::from(file);
    let mut hasher = Blake224::default();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_hmac_256() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("blake")
        .join("round2")
        .join("ShortMsgKAT_HMAC_256.txt");
    let kat = ShortMsgKatHmac::from(file);
    let mut hasher = Blake256::default();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_hmac_384() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("blake")
        .join("round2")
        .join("ShortMsgKAT_HMAC_384.txt");
    let kat = ShortMsgKatHmac::from(file);
    let mut hasher = Blake384::default();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_hmac_512() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("blake")
        .join("round2")
        .join("ShortMsgKAT_HMAC_512.txt");
    let kat = ShortMsgKatHmac::from(file);
    let mut hasher = Blake512::default();
    kat.assert(&mut hasher);
}
