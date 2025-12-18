use std::path::Path;

use dev_util::short_msg_kat::ShortMsgKat;
use keccak::KeccakF200;

#[test]
fn short_msg_kat_512_r40_c160() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("keccak_f200")
        .join("ShortMsgKAT_512_r40_c160.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = KeccakF200::new(40, 160, 64);
    kat.assert(&mut hasher);
}
