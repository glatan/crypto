use dev_util::impl_short_msg_kat_hmac_from_file;

use edonr::EdonR224;
use hmac::Hmac;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC_224.txt", EdonR224::default());
