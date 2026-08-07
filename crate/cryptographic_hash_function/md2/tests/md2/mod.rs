mod short_msg_kat_hmac;

use dev_util::impl_short_msg_kat_all;

impl_short_msg_kat_all!(md2::Md2, Md2::default(), "tests/md2/ShortMsgKAT.txt");
