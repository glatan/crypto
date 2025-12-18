use std::path::Path;

use dev_util::short_msg_kat::ShortMsgKatHmac;
use md2::Md2;

#[test]
fn short_msg_kat_hmac() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("md2")
        .join("ShortMsgKAT_HMAC.txt");
    let kat = ShortMsgKatHmac::from(file);
    let mut hasher = Md2::new();
    kat.assert(&mut hasher);
}
