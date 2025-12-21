use std::path::Path;

use blake::{Blake28, Blake32, Blake48, Blake64};
use dev_util::short_msg_kat::ShortMsgKatHmac;

#[test]
fn short_msg_kat_hmac_224() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("blake")
        .join("round2")
        .join("ShortMsgKAT_HMAC_224.txt");
    let kat = ShortMsgKatHmac::from(file);
    let mut hasher = Blake28::default();
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
    let mut hasher = Blake32::default();
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
    let mut hasher = Blake48::default();
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
    let mut hasher = Blake64::default();
    kat.assert(&mut hasher);
}
