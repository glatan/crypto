use dev_util::impl_short_msg_kat_hmac_from_file;

use hmac::Hmac;
use sha0::Sha0;

impl_short_msg_kat_hmac_from_file!("ShortMsgKAT_HMAC.txt", Sha0::default());
