use dev_util::impl_short_msg_kat_hmac_from_file;

use hmac::Hmac;
use sha2::Sha256;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC_256.txt", Sha256::default());
