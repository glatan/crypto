use std::path::Path;

use dev_util::short_msg_kat::ShortMsgKat;
use edonr::{EdonR224, EdonR256, EdonR384, EdonR512};

#[test]
fn short_msg_kat_224() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("edonr")
        .join("ShortMsgKAT_224.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = EdonR224::new();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_256() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("edonr")
        .join("ShortMsgKAT_256.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = EdonR256::new();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_384() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("edonr")
        .join("ShortMsgKAT_384.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = EdonR384::new();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_512() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("edonr")
        .join("ShortMsgKAT_512.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = EdonR512::new();
    kat.assert(&mut hasher);
}
