use std::path::Path;

use blake2::Blake2s;
use dev_util::short_msg_kat::ShortMsgKat;

#[test]
fn short_msg_kat_128() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("blake2s")
        .join("ShortMsgKAT_128.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Blake2s::new(16);
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_160() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("blake2s")
        .join("ShortMsgKAT_160.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Blake2s::new(20);
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_224() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("blake2s")
        .join("ShortMsgKAT_224.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Blake2s::new(28);
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_256() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("blake2s")
        .join("ShortMsgKAT_256.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Blake2s::new(32);
    kat.assert(&mut hasher);
}
