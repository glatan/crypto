use dev_util::impl_short_msg_kat_hmac_from_file;

use hmac::Hmac;
use md5::Md5;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC.txt", Md5::default());
