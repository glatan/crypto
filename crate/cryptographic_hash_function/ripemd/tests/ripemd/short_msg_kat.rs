use std::path::Path;

use dev_util::short_msg_kat::ShortMsgKat;
use ripemd::{Ripemd, Ripemd128, Ripemd160, Ripemd256, Ripemd320};

#[test]
fn short_msg_kat() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("ripemd")
        .join("ShortMsgKAT.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Ripemd::new();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_128() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("ripemd")
        .join("ShortMsgKAT_128.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Ripemd128::new();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_160() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("ripemd")
        .join("ShortMsgKAT_160.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Ripemd160::new();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_256() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("ripemd")
        .join("ShortMsgKAT_256.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Ripemd256::new();
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_320() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("ripemd")
        .join("ShortMsgKAT_320.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Ripemd320::new();
    kat.assert(&mut hasher);
}
