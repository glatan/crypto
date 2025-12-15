use dev_util::impl_short_msg_kat_hmac_from_file;

use hmac::Hmac;
use sha3::Shake128;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC_256.txt", Shake128::new(32));
