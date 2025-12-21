use std::path::Path;

use blake2::Blake2b;
use dev_util::short_msg_kat::ShortMsgKat;

#[test]
fn short_msg_kat_224() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("blake2b")
        .join("ShortMsgKAT_224.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Blake2b::new(28);
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_256() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("blake2b")
        .join("ShortMsgKAT_256.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Blake2b::new(32);
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_384() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("blake2b")
        .join("ShortMsgKAT_384.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Blake2b::new(48);
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_512() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("blake2b")
        .join("ShortMsgKAT_512.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Blake2b::new(64);
    kat.assert(&mut hasher);
}
