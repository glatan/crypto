use std::path::Path;

use dev_util::short_msg_kat::ShortMsgKat;
use md2::Md2;

#[test]
fn short_msg_kat() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("ShortMsgKAT.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Md2::default();
    kat.assert(&mut hasher);
}
