use dev_util::impl_short_msg_kat_hmac_from_file;

use hmac::Hmac;
use md2::Md2;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC.txt", Md2::default());
