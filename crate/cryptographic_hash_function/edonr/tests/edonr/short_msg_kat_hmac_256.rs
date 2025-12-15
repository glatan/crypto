use dev_util::impl_short_msg_kat_hmac_from_file;

use edonr::EdonR256;
use hmac::Hmac;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC_256.txt", EdonR256::default());
