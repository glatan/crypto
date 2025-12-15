use dev_util::impl_short_msg_kat_hmac_from_file;

use blake2::Blake2b;
use hmac::Hmac;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC_224.txt", Blake2b::new(28));
