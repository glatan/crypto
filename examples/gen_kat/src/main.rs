mod short_msg_kat;

use std::fmt::Write;

use clap::{Parser, Subcommand};

use crypto::cryptographic_hash_function::{
    Blake2b, Blake2s, Blake28, Blake32, Blake48, Blake64, Blake224, Blake256, Blake384, Blake512,
    EdonR224, EdonR256, EdonR384, EdonR512, Keccak224, Keccak256, Keccak384, Keccak512, Md2, Md4,
    Md5, Ripemd, Ripemd128, Ripemd160, Ripemd256, Ripemd320, Sha0, Sha1, Sha3_224, Sha3_256,
    Sha3_384, Sha3_512, Sha224, Sha256, Sha384, Sha512, Sha512Trunc224, Sha512Trunc256, Shake128,
    Shake256,
};

use short_msg_kat::{short_msg_kat, short_msg_kat_hmac};

fn bytes_to_lowerhex(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut hex, byte| {
        let _ = write!(hex, "{byte:02x}");
        hex
    })
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    algorithm: Algorithm,
    #[arg(long)]
    hmac: bool,
}

#[derive(Subcommand, Debug)]
#[clap(rename_all = "kebab-case")]
enum Algorithm {
    Blake28,
    Blake32,
    Blake48,
    Blake64,
    Blake224,
    Blake256,
    Blake384,
    Blake512,
    Blake2b {
        #[arg(long, default_value_t = 64)]
        len: usize,
    },
    Blake2s {
        #[arg(long, default_value_t = 32)]
        len: usize,
    },
    Edonr224,
    Edonr256,
    Edonr384,
    Edonr512,
    Keccak224,
    Keccak256,
    Keccak384,
    Keccak512,
    Md2,
    Md4,
    Md5,
    Ripemd,
    Ripemd128,
    Ripemd160,
    Ripemd256,
    Ripemd320,
    Sha0,
    Sha1,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Sha512Trunc224,
    Sha512Trunc256,
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    Shake128 {
        #[arg(long)]
        len: usize,
    },
    Shake256 {
        #[arg(long)]
        len: usize,
    },
}

trait AlgorithmName {
    const ALGORITHM_NAME: &'static str;
}

macro_rules! impl_algorithm_name {
    ($T:ty, $name:expr) => {
        impl AlgorithmName for $T {
            const ALGORITHM_NAME: &'static str = $name;
        }
    };
}

impl_algorithm_name!(Md2, "MD2");
impl_algorithm_name!(Md4, "MD4");
impl_algorithm_name!(Md5, "MD5");
impl_algorithm_name!(Ripemd, "RIPEMD(Original Version)");
impl_algorithm_name!(Ripemd128, "RIPEMD");
impl_algorithm_name!(Ripemd160, "RIPEMD");
impl_algorithm_name!(Ripemd256, "RIPEMD");
impl_algorithm_name!(Ripemd320, "RIPEMD");
impl_algorithm_name!(Sha0, "SHA-0");
impl_algorithm_name!(Sha1, "SHA-1");
impl_algorithm_name!(Sha224, "SHA-2");
impl_algorithm_name!(Sha256, "SHA-2");
impl_algorithm_name!(Sha384, "SHA-2");
impl_algorithm_name!(Sha512, "SHA-2");
impl_algorithm_name!(Sha512Trunc224, "SHA-2");
impl_algorithm_name!(Sha512Trunc256, "SHA-2");
impl_algorithm_name!(Sha3_224, "SHA-3");
impl_algorithm_name!(Sha3_256, "SHA-3");
impl_algorithm_name!(Sha3_384, "SHA-3");
impl_algorithm_name!(Sha3_512, "SHA-3");
impl_algorithm_name!(Shake128, "SHAKE");
impl_algorithm_name!(Shake256, "SHAKE");
impl_algorithm_name!(Blake28, "BLAKE");
impl_algorithm_name!(Blake32, "BLAKE");
impl_algorithm_name!(Blake48, "BLAKE");
impl_algorithm_name!(Blake64, "BLAKE");
impl_algorithm_name!(Blake224, "BLAKE");
impl_algorithm_name!(Blake256, "BLAKE");
impl_algorithm_name!(Blake384, "BLAKE");
impl_algorithm_name!(Blake512, "BLAKE");
impl_algorithm_name!(Blake2b, "BLAKE2");
impl_algorithm_name!(Blake2s, "BLAKE2");
impl_algorithm_name!(EdonR224, "Edon-R");
impl_algorithm_name!(EdonR256, "Edon-R");
impl_algorithm_name!(EdonR384, "Edon-R");
impl_algorithm_name!(EdonR512, "Edon-R");
impl_algorithm_name!(Keccak224, "Keccak");
impl_algorithm_name!(Keccak256, "Keccak");
impl_algorithm_name!(Keccak384, "Keccak");
impl_algorithm_name!(Keccak512, "Keccak");

fn main() {
    let args = Args::parse();

    match args.algorithm {
        Algorithm::Blake28 => {
            let mut hasher = Blake28::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Blake32 => {
            let mut hasher = Blake32::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Blake48 => {
            let mut hasher = Blake48::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Blake64 => {
            let mut hasher = Blake64::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Blake224 => {
            let mut hasher = Blake224::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Blake256 => {
            let mut hasher = Blake256::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Blake384 => {
            let mut hasher = Blake384::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Blake512 => {
            let mut hasher = Blake512::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Blake2b { len } => {
            let mut hasher = Blake2b::new(len);
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Blake2s { len } => {
            let mut hasher = Blake2s::new(len);
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Edonr224 => {
            let mut hasher = EdonR224::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Edonr256 => {
            let mut hasher = EdonR256::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Edonr384 => {
            let mut hasher = EdonR384::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Edonr512 => {
            let mut hasher = EdonR512::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Keccak224 => {
            let mut hasher = Keccak224::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Keccak256 => {
            let mut hasher = Keccak256::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Keccak384 => {
            let mut hasher = Keccak384::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Keccak512 => {
            let mut hasher = Keccak512::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Md2 => {
            let mut hasher = Md2::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Md4 => {
            let mut hasher = Md4::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Md5 => {
            let mut hasher = Md5::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Ripemd => {
            let mut hasher = Ripemd::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Ripemd128 => {
            let mut hasher = Ripemd128::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Ripemd160 => {
            let mut hasher = Ripemd160::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Ripemd256 => {
            let mut hasher = Ripemd256::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Ripemd320 => {
            let mut hasher = Ripemd320::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Sha0 => {
            let mut hasher = Sha0::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Sha1 => {
            let mut hasher = Sha1::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Sha224 => {
            let mut hasher = Sha224::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Sha256 => {
            let mut hasher = Sha256::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Sha384 => {
            let mut hasher = Sha384::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Sha512 => {
            let mut hasher = Sha512::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Sha512Trunc224 => {
            let mut hasher = Sha512Trunc224::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Sha512Trunc256 => {
            let mut hasher = Sha512Trunc256::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Sha3_224 => {
            let mut hasher = Sha3_224::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Sha3_256 => {
            let mut hasher = Sha3_256::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Sha3_384 => {
            let mut hasher = Sha3_384::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Sha3_512 => {
            let mut hasher = Sha3_512::default();
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Shake128 { len } => {
            let mut hasher = Shake128::new(len);
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
        Algorithm::Shake256 { len } => {
            let mut hasher = Shake256::new(len);
            if args.hmac {
                short_msg_kat_hmac(&mut hasher);
            } else {
                short_msg_kat(&mut hasher);
            }
        }
    };
}
