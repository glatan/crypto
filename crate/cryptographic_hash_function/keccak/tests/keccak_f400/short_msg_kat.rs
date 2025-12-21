use std::path::Path;

use dev_util::short_msg_kat::ShortMsgKat;
use keccak::KeccakF400;

#[test]
fn short_msg_kat_512_r144_c256() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("keccak_f400")
        .join("ShortMsgKAT_512_r144_c256.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = KeccakF400::new(144, 256, 64);
    kat.assert(&mut hasher);
}

#[test]
fn short_msg_kat_512_r240_c160() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("keccak_f400")
        .join("ShortMsgKAT_512_r240_c160.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = KeccakF400::new(240, 160, 64);
    kat.assert(&mut hasher);
}
