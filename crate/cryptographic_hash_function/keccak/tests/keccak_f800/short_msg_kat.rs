use std::path::Path;

use dev_util::short_msg_kat::ShortMsgKat;
use keccak::KeccakF800;

#[test]
fn short_msg_kat_512_r288_c512() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("keccak_f800")
        .join("ShortMsgKAT_512_r288_c512.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = KeccakF800::new(288, 512, 64);
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_512_r544_c256() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("keccak_f800")
        .join("ShortMsgKAT_512_r544_c256.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = KeccakF800::new(544, 256, 64);
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_512_r640_c160() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("keccak_f800")
        .join("ShortMsgKAT_512_r640_c160.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = KeccakF800::new(640, 160, 64);
    kat.assert(&mut hasher);
}
