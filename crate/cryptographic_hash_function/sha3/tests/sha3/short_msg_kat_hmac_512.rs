use dev_util::impl_short_msg_kat_hmac_from_file;

use hmac::Hmac;
use sha3::Sha3_512;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC_512.txt", Sha3_512::default());
