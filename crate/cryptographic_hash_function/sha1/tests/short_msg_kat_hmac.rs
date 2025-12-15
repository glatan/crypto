use dev_util::impl_short_msg_kat_hmac_from_file;

use hmac::Hmac;
use sha1::Sha1;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC.txt", Sha1::default());
