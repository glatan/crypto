use std::{env, path::Path};

use blake::{Blake224, Blake256, Blake384, Blake512};
use dev_util::short_msg_kat::ShortMsgKat;

#[test]
fn short_msg_kat_224() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("blake")
        .join("round3")
        .join("ShortMsgKAT_224.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Blake224::default();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_256() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("blake")
        .join("round3")
        .join("ShortMsgKAT_256.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Blake256::default();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_384() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("blake")
        .join("round3")
        .join("ShortMsgKAT_384.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Blake384::default();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_512() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("blake")
        .join("round3")
        .join("ShortMsgKAT_512.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Blake512::default();
    kat.assert(&mut hasher);
}
