use dev_util::impl_short_msg_kat_hmac_from_file;

use hmac::Hmac;
use ripemd::Ripemd320;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC_320.txt", Ripemd320::default());
