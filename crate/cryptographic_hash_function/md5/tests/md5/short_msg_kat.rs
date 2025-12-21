use std::path::Path;

use dev_util::short_msg_kat::ShortMsgKat;
use md5::Md5;

#[test]
fn short_msg_kat() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("md5")
        .join("ShortMsgKAT.txt");
    let kat = ShortMsgKat::from(file);
    let mut hasher = Md5::default();
    kat.assert(&mut hasher);
}
