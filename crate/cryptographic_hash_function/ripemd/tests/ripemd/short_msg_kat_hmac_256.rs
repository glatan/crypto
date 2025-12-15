use dev_util::impl_short_msg_kat_hmac_from_file;

use hmac::Hmac;
use ripemd::Ripemd256;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC_256.txt", Ripemd256::default());
