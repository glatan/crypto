use std::path::Path;

use dev_util::short_msg_kat::ShortMsgKat;
use md4::Md4;

#[test]
fn short_msg_kat() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("ShortMsgKAT.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Md4::default();
    kat.assert(&mut hasher);
}
