use dev_util::impl_short_msg_kat_hmac_from_file;

use hmac::Hmac;
use sha3::Shake256;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC_1024.txt", Shake256::new(128));
