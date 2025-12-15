use dev_util::impl_short_msg_kat_hmac_from_file;

use blake::Blake384;
use hmac::Hmac;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC_384.txt", Blake384::default());
