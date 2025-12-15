use dev_util::impl_short_msg_kat_hmac_from_file;

use hmac::Hmac;
use ripemd::Ripemd128;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC_128.txt", Ripemd128::default());
