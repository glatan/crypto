use dev_util::impl_short_msg_kat_hmac_from_file;

use hmac::Hmac;
use sha2::Sha224;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC_224.txt", Sha224::default());
