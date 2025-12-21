use clap::{Parser, Subcommand};
use crypto::cryptographic_hash_function::{
    Blake2b, Blake2s, Blake28, Blake32, Blake48, Blake64, Blake224, Blake256, Blake384, Blake512,
    CryptographicHashFunction, EdonR224, EdonR256, EdonR384, EdonR512, Keccak224, Keccak256,
    Keccak384, Keccak512, Md2, Md4, Md5, Ripemd, Ripemd128, Ripemd160, Ripemd256, Ripemd320, Sha0,
    Sha1, Sha3_224, Sha3_256, Sha3_384, Sha3_512, Sha224, Sha256, Sha384, Sha512, Sha512Trunc224,
    Sha512Trunc256, Shake128, Shake256,
};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name = "hashsum")]
#[command(version = "0.1.0")]
#[command(about = "Print or check checksums")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(name = "blake28")]
    Blake28 {
        /// File to hash.
        file: String,
    },
    #[command(name = "blake32")]
    Blake32 {
        /// File to hash.
        file: String,
    },
    #[command(name = "blake48")]
    Blake48 {
        /// File to hash.
        file: String,
    },
    #[command(name = "blake64")]
    Blake64 {
        /// File to hash.
        file: String,
    },
    #[command(name = "blake224")]
    Blake224 {
        /// File to hash.
        file: String,
    },
    #[command(name = "blake256")]
    Blake256 {
        /// File to hash.
        file: String,
    },
    #[command(name = "blake384")]
    Blake384 {
        /// File to hash.
        file: String,
    },
    #[command(name = "blake512")]
    Blake512 {
        /// File to hash.
        file: String,
    },
    #[command(name = "blake2b")]
    Blake2b {
        /// File to hash.
        file: String,
    },
    #[command(name = "blake2s")]
    Blake2s {
        /// File to hash.
        file: String,
    },
    #[command(name = "edonr224")]
    EdonR224 {
        /// File to hash.
        file: String,
    },
    #[command(name = "edonr256")]
    EdonR256 {
        /// File to hash.
        file: String,
    },
    #[command(name = "edonr384")]
    EdonR384 {
        /// File to hash.
        file: String,
    },
    #[command(name = "edonr512")]
    EdonR512 {
        /// File to hash.
        file: String,
    },
    #[command(name = "keccak224")]
    Keccak224 {
        /// File to hash.
        file: String,
    },
    #[command(name = "keccak256")]
    Keccak256 {
        /// File to hash.
        file: String,
    },
    #[command(name = "keccak384")]
    Keccak384 {
        /// File to hash.
        file: String,
    },
    #[command(name = "keccak512")]
    Keccak512 {
        /// File to hash.
        file: String,
    },
    #[command(name = "md2")]
    Md2 {
        /// File to hash.
        file: String,
    },
    #[command(name = "md4")]
    Md4 {
        /// File to hash.
        file: String,
    },
    #[command(name = "md5")]
    Md5 {
        /// File to hash.
        file: String,
    },
    #[command(name = "ripemd")]
    Ripemd {
        /// File to hash.
        file: String,
    },
    #[command(name = "ripemd128")]
    Ripemd128 {
        /// File to hash.
        file: String,
    },
    #[command(name = "ripemd160")]
    Ripemd160 {
        /// File to hash.
        file: String,
    },
    #[command(name = "ripemd256")]
    Ripemd256 {
        /// File to hash.
        file: String,
    },
    #[command(name = "ripemd320")]
    Ripemd320 {
        /// File to hash.
        file: String,
    },
    #[command(name = "sha0")]
    Sha0 {
        /// File to hash.
        file: String,
    },
    #[command(name = "sha1")]
    Sha1 {
        /// File to hash.
        file: String,
    },
    #[command(name = "sha224")]
    Sha224 {
        /// File to hash.
        file: String,
    },
    #[command(name = "sha256")]
    Sha256 {
        /// File to hash.
        file: String,
    },
    #[command(name = "sha384")]
    Sha384 {
        /// File to hash.
        file: String,
    },
    #[command(name = "sha512")]
    Sha512 {
        /// File to hash.
        file: String,
    },
    #[command(name = "sha512/224")]
    Sha512Trunc224 {
        /// File to hash.
        file: String,
    },
    #[command(name = "sha512/256")]
    Sha512Trunc256 {
        /// File to hash.
        file: String,
    },
    #[command(name = "sha3-224")]
    Sha3_224 {
        /// File to hash.
        file: String,
    },
    #[command(name = "sha3-256")]
    Sha3_256 {
        /// File to hash.
        file: String,
    },
    #[command(name = "sha3-384")]
    Sha3_384 {
        /// File to hash.
        file: String,
    },
    #[command(name = "sha3-512")]
    Sha3_512 {
        /// File to hash.
        file: String,
    },
    #[command(name = "shake128")]
    Shake128 {
        /// File to hash.
        file: String,
        /// Length of the output in bytes.
        #[arg(long)]
        len: usize,
    },
    #[command(name = "shake256")]
    Shake256 {
        /// File to hash.
        file: String,
        /// Length of the output in bytes.
        #[arg(long)]
        len: usize,
    },
}

struct MessageFile {
    name: String,
    message: Vec<u8>,
}

impl MessageFile {
    fn from_path(path_str: &str) -> Self {
        let path = Path::new(path_str);
        let filename = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(path_str)
            .to_string();

        let message = match std::fs::read(path) {
            Ok(bytes) => bytes,
            Err(e) => panic!("Failed to read file '{}': {}", path_str, e),
        };

        Self {
            name: filename,
            message,
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let (hash_str, filename) = match cli.command {
        Commands::Blake28 { file } => {
            let m = MessageFile::from_path(&file);
            (Blake28::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Blake32 { file } => {
            let m = MessageFile::from_path(&file);
            (Blake32::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Blake48 { file } => {
            let m = MessageFile::from_path(&file);
            (Blake48::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Blake64 { file } => {
            let m = MessageFile::from_path(&file);
            (Blake64::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Blake224 { file } => {
            let m = MessageFile::from_path(&file);
            (Blake224::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Blake256 { file } => {
            let m = MessageFile::from_path(&file);
            (Blake256::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Blake384 { file } => {
            let m = MessageFile::from_path(&file);
            (Blake384::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Blake512 { file } => {
            let m = MessageFile::from_path(&file);
            (Blake512::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Blake2b { file } => {
            let m = MessageFile::from_path(&file);
            (Blake2b::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Blake2s { file } => {
            let m = MessageFile::from_path(&file);
            (Blake2s::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::EdonR224 { file } => {
            let m = MessageFile::from_path(&file);
            (EdonR224::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::EdonR256 { file } => {
            let m = MessageFile::from_path(&file);
            (EdonR256::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::EdonR384 { file } => {
            let m = MessageFile::from_path(&file);
            (EdonR384::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::EdonR512 { file } => {
            let m = MessageFile::from_path(&file);
            (EdonR512::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Keccak224 { file } => {
            let m = MessageFile::from_path(&file);
            (Keccak224::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Keccak256 { file } => {
            let m = MessageFile::from_path(&file);
            (Keccak256::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Keccak384 { file } => {
            let m = MessageFile::from_path(&file);
            (Keccak384::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Keccak512 { file } => {
            let m = MessageFile::from_path(&file);
            (Keccak512::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Md2 { file } => {
            let m = MessageFile::from_path(&file);
            (Md2::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Md4 { file } => {
            let m = MessageFile::from_path(&file);
            (Md4::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Md5 { file } => {
            let m = MessageFile::from_path(&file);
            (Md5::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Ripemd { file } => {
            let m = MessageFile::from_path(&file);
            (Ripemd::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Ripemd128 { file } => {
            let m = MessageFile::from_path(&file);
            (Ripemd128::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Ripemd160 { file } => {
            let m = MessageFile::from_path(&file);
            (Ripemd160::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Ripemd256 { file } => {
            let m = MessageFile::from_path(&file);
            (Ripemd256::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Ripemd320 { file } => {
            let m = MessageFile::from_path(&file);
            (Ripemd320::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Sha0 { file } => {
            let m = MessageFile::from_path(&file);
            (Sha0::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Sha1 { file } => {
            let m = MessageFile::from_path(&file);
            (Sha1::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Sha224 { file } => {
            let m = MessageFile::from_path(&file);
            (Sha224::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Sha256 { file } => {
            let m = MessageFile::from_path(&file);
            (Sha256::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Sha384 { file } => {
            let m = MessageFile::from_path(&file);
            (Sha384::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Sha512 { file } => {
            let m = MessageFile::from_path(&file);
            (Sha512::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Sha512Trunc224 { file } => {
            let m = MessageFile::from_path(&file);
            (
                Sha512Trunc224::default().hash_to_lowerhex(&m.message),
                m.name,
            )
        }
        Commands::Sha512Trunc256 { file } => {
            let m = MessageFile::from_path(&file);
            (
                Sha512Trunc256::default().hash_to_lowerhex(&m.message),
                m.name,
            )
        }
        Commands::Sha3_224 { file } => {
            let m = MessageFile::from_path(&file);
            (Sha3_224::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Sha3_256 { file } => {
            let m = MessageFile::from_path(&file);
            (Sha3_256::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Sha3_384 { file } => {
            let m = MessageFile::from_path(&file);
            (Sha3_384::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Sha3_512 { file } => {
            let m = MessageFile::from_path(&file);
            (Sha3_512::default().hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Shake128 { file, len } => {
            let m = MessageFile::from_path(&file);
            (Shake128::new(len).hash_to_lowerhex(&m.message), m.name)
        }
        Commands::Shake256 { file, len } => {
            let m = MessageFile::from_path(&file);
            (Shake256::new(len).hash_to_lowerhex(&m.message), m.name)
        }
    };

    println!("{hash_str} {filename}");
}
