use dev_util::impl_short_msg_kat_hmac_from_file;

use hmac::Hmac;
use sha2::Sha512;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC_512.txt", Sha512::default());
