use dev_util::impl_short_msg_kat_hmac_from_file;

use hmac::Hmac;
use ripemd::Ripemd160;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC_160.txt", Ripemd160::default());
