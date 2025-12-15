use dev_util::impl_short_msg_kat_hmac_from_file;

use blake::Blake28;
use hmac::Hmac;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC_224.txt", Blake28::default());
