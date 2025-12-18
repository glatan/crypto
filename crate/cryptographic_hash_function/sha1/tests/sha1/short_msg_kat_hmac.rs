use std::path::Path;

use dev_util::short_msg_kat::ShortMsgKatHmac;
use sha1::Sha1;

#[test]
fn short_msg_kat_hmac() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("sha1")
        .join("ShortMsgKAT_HMAC.txt");
    let kat = ShortMsgKatHmac::from(file);
    let mut hasher = Sha1::default();
    kat.assert(&mut hasher);
}
