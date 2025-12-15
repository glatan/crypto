use dev_util::impl_short_msg_kat_hmac_from_file;

use edonr::EdonR384;
use hmac::Hmac;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC_384.txt", EdonR384::default());
