use std::path::Path;
use std::{fs, panic};

use byte::{bytes_to_lowerhex, hex_to_bytes};
use cryptographic_hash_function::{BlockSize, CryptographicHashFunction};
use hmac::Hmac;

struct ShortMsgKatEntry {
    msg: String,
    md: String,
}

impl ShortMsgKatEntry {
    fn assert_by_hash_to_lowerhex(&self, hasher: &mut impl CryptographicHashFunction) {
        assert_eq!(self.md, hasher.hash_to_lowerhex(&hex_to_bytes(&self.msg)));
    }
    fn assert_chunked_update<Haser>(&self, hasher: &mut Haser, chunk_size: usize)
    where
        Haser: CryptographicHashFunction,
    {
        let message = &hex_to_bytes(&self.msg);
        let blocks = message.chunks(chunk_size);
        for block in blocks {
            hasher.update(block);
        }
        let digest = hasher.finalize();
        assert_eq!(self.md, bytes_to_lowerhex(digest.as_slice()));
    }
}

pub struct ShortMsgKat {
    entries: Vec<ShortMsgKatEntry>,
}

impl ShortMsgKat {
    /// hash_to_lowerhexでまとめて処理したあとにassertする
    pub fn assert_by_hash_to_lowerhex<Hasher>(&self, hasher: &mut Hasher)
    where
        Hasher: CryptographicHashFunction + Clone,
    {
        for entry in &self.entries {
            entry.assert_by_hash_to_lowerhex(&mut hasher.clone());
        }
    }
    /// ブロックサイズちょうどのchunkにしてupdateを繰り返した後、finalizeしてassertする
    pub fn assert_chunked_by_block_size<Hasher>(&self, hasher: &mut Hasher)
    where
        Hasher: CryptographicHashFunction + BlockSize + Clone,
    {
        for entry in &self.entries {
            entry.assert_chunked_update(&mut hasher.clone(), hasher.byte_block_len());
        }
    }
    /// ブロックサイズ+1のchunkにしてupdateを繰り返した後、finalizeしてassertする
    pub fn assert_chunked_by_block_size_plus_1<Hasher>(&self, hasher: &mut Hasher)
    where
        Hasher: CryptographicHashFunction + BlockSize + Clone,
    {
        for entry in &self.entries {
            entry.assert_chunked_update(&mut hasher.clone(), hasher.byte_block_len() + 1);
        }
    }
    /// ブロックサイズ-1のchunkにしてupdateを繰り返した後、finalizeしてassertする
    pub fn assert_chunked_by_block_size_minus_1<Hasher>(&self, hasher: &mut Hasher)
    where
        Hasher: CryptographicHashFunction + BlockSize + Clone,
    {
        for entry in &self.entries {
            entry.assert_chunked_update(&mut hasher.clone(), hasher.byte_block_len() - 1);
        }
    }
}

impl<T: AsRef<Path>> From<T> for ShortMsgKat {
    fn from(value: T) -> Self {
        let txt = fs::read_to_string(value).expect("Failed to read file");
        let mut kat = Self {
            entries: Vec::new(),
        };

        let mut lines = txt.lines();
        while let Some(line) = lines.next() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line.starts_with("Len") {
                // advance to find Msg and MD lines for this Len block
                let len: usize = if let Some(idx) = line.find('=') {
                    line[idx + 1..].trim().parse().unwrap()
                } else {
                    panic!("Invalid Len line");
                };
                let mut msg = "";
                let mut md = "";
                for line in lines.by_ref() {
                    let line = line.trim();
                    match line {
                        line if line.is_empty() || line.starts_with('#') => continue,
                        line if line.starts_with("Msg") => {
                            if let Some(idx) = line.find('=') {
                                msg = line[idx + 1..].trim();
                            }
                            continue;
                        }
                        line if line.starts_with("MD") => {
                            if let Some(idx) = line.find('=') {
                                md = line[idx + 1..].trim();
                            }
                            assert_eq!(len, msg.len() * 4, "{msg}");
                            break;
                        }
                        _ => continue,
                    }
                }
                kat.entries.push(ShortMsgKatEntry {
                    msg: msg.to_string(),
                    md: md.to_string(),
                });
            }
        }

        kat
    }
}

#[macro_export]
macro_rules! impl_short_msg_kat_all {
    ($impl_path:path, $hasher:expr, $short_msg_kat_path:expr) => {
        mod short_msg_kat {
            use std::path::Path;

            use dev_util::short_msg_kat::ShortMsgKat;
            use $impl_path;

            #[test]
            fn assert_by_hash_to_lowerhex() {
                let file = Path::new(env!("CARGO_MANIFEST_DIR")).join($short_msg_kat_path);
                let kat = ShortMsgKat::from(file);
                let mut hasher = $hasher;
                kat.assert_by_hash_to_lowerhex(&mut hasher);
            }

            #[test]
            fn assert_chunked_by_block_size() {
                let file = Path::new(env!("CARGO_MANIFEST_DIR")).join($short_msg_kat_path);
                let kat = ShortMsgKat::from(file);
                let mut hasher = $hasher;
                kat.assert_chunked_by_block_size(&mut hasher);
            }

            #[test]
            fn assert_chunked_by_block_size_plus_1() {
                let file = Path::new(env!("CARGO_MANIFEST_DIR")).join($short_msg_kat_path);
                let kat = ShortMsgKat::from(file);
                let mut hasher = $hasher;
                kat.assert_chunked_by_block_size_plus_1(&mut hasher);
            }

            #[test]
            fn assert_chunked_by_block_size_minus_1() {
                let file = Path::new(env!("CARGO_MANIFEST_DIR")).join($short_msg_kat_path);
                let kat = ShortMsgKat::from(file);
                let mut hasher = $hasher;
                kat.assert_chunked_by_block_size_minus_1(&mut hasher);
            }
        }
    };
}

struct ShortMsgKatHmacEntry {
    msg: String,
    md: String,
    key: String,
}

impl ShortMsgKatHmacEntry {
    fn assert(&self, hasher: &mut impl Hmac) {
        assert_eq!(
            self.md,
            hasher.hmac_hash_to_lower_hex(&hex_to_bytes(&self.msg), &hex_to_bytes(&self.key))
        );
    }
}

pub struct ShortMsgKatHmac {
    entries: Vec<ShortMsgKatHmacEntry>,
}

impl ShortMsgKatHmac {
    pub fn assert<Hasher>(&self, hasher: &mut Hasher)
    where
        Hasher: Hmac + Clone,
    {
        for entry in &self.entries {
            entry.assert(&mut hasher.clone());
        }
    }
}

impl<T: AsRef<Path>> From<T> for ShortMsgKatHmac {
    fn from(value: T) -> Self {
        let txt = fs::read_to_string(value).expect("Failed to read file");
        let mut kat = Self {
            entries: Vec::new(),
        };

        let mut lines = txt.lines();
        while let Some(line) = lines.next() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line.starts_with("Len") {
                // advance to find Msg and MD lines for this Len block
                let len: usize = if let Some(idx) = line.find('=') {
                    line[idx + 1..].trim().parse().unwrap()
                } else {
                    panic!("Invalid Len line");
                };
                let mut msg = "";
                let mut key = "";
                let mut md = "";
                for line in lines.by_ref() {
                    let line = line.trim();
                    match line {
                        line if line.is_empty() || line.starts_with('#') => continue,
                        line if line.starts_with("Msg") => {
                            if let Some(idx) = line.find('=') {
                                msg = line[idx + 1..].trim();
                            }
                            continue;
                        }
                        line if line.starts_with("Key") => {
                            if let Some(idx) = line.find('=') {
                                key = line[idx + 1..].trim();
                            }
                            assert_eq!(64 * 8, key.len() * 4, "{key}");
                            continue;
                        }
                        line if line.starts_with("MD") => {
                            if let Some(idx) = line.find('=') {
                                md = line[idx + 1..].trim();
                            }
                            assert_eq!(len, msg.len() * 4, "{msg}");
                            break;
                        }
                        _ => continue,
                    }
                }
                kat.entries.push(ShortMsgKatHmacEntry {
                    msg: msg.to_string(),
                    md: md.to_string(),
                    key: key.to_string(),
                });
            }
        }

        kat
    }
}
