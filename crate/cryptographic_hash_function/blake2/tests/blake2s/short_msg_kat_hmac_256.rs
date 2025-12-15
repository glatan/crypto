use dev_util::impl_short_msg_kat_hmac_from_file;

use blake2::Blake2s;
use hmac::Hmac;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC_256.txt", Blake2s::default());
