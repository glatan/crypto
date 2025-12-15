use dev_util::impl_short_msg_kat_hmac_from_file;

use blake::Blake64;
use hmac::Hmac;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC_512.txt", Blake64::default());
