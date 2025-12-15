use dev_util::impl_short_msg_kat_hmac_from_file;

use hmac::Hmac;
use keccak::Keccak384;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC_384.txt", Keccak384::default());
