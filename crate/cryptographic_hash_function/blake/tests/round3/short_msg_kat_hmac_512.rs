use dev_util::impl_short_msg_kat_hmac_from_file;

use blake::Blake512;
use hmac::Hmac;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC_512.txt", Blake512::default());
