use std::path::Path;

use dev_util::short_msg_kat::ShortMsgKatHmac;
use md4::Md4;

#[test]
fn short_msg_kat_hmac() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("md4")
        .join("ShortMsgKAT_HMAC.txt");
    let kat = ShortMsgKatHmac::from(file);
    let mut hasher = Md4::new();
    kat.assert(&mut hasher);
}
