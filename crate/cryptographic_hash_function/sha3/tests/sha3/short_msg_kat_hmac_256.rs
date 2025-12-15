use dev_util::impl_short_msg_kat_hmac_from_file;

use hmac::Hmac;
use sha3::Sha3_256;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC_256.txt", Sha3_256::default());
