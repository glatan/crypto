interface Target {
  target: string;
  options: string;
  outfile: string;
}

const TARGETS = [
  {
    options: "blake28",
    outfile:
      "../../crate/cryptographic_hash_function/blake/tests/round2/ShortMsgKAT_224.txt",
  },
  {
    options: "blake32",
    outfile:
      "../../crate/cryptographic_hash_function/blake/tests/round2/ShortMsgKAT_256.txt",
  },
  {
    options: "blake48",
    outfile:
      "../../crate/cryptographic_hash_function/blake/tests/round2/ShortMsgKAT_384.txt",
  },
  {
    options: "blake64",
    outfile:
      "../../crate/cryptographic_hash_function/blake/tests/round2/ShortMsgKAT_512.txt",
  },
  {
    options: "blake224",
    outfile:
      "../../crate/cryptographic_hash_function/blake/tests/round3/ShortMsgKAT_224.txt",
  },
  {
    options: "blake256",
    outfile:
      "../../crate/cryptographic_hash_function/blake/tests/round3/ShortMsgKAT_256.txt",
  },
  {
    options: "blake384",
    outfile:
      "../../crate/cryptographic_hash_function/blake/tests/round3/ShortMsgKAT_384.txt",
  },
  {
    options: "blake512",
    outfile:
      "../../crate/cryptographic_hash_function/blake/tests/round3/ShortMsgKAT_512.txt",
  },
  {
    options: "blake2b --len 28",
    outfile:
      "../../crate/cryptographic_hash_function/blake2/tests/blake2b/ShortMsgKAT_224.txt",
  },
  {
    options: "blake2b --len 32",
    outfile:
      "../../crate/cryptographic_hash_function/blake2/tests/blake2b/ShortMsgKAT_256.txt",
  },
  {
    options: "blake2b --len 48",
    outfile:
      "../../crate/cryptographic_hash_function/blake2/tests/blake2b/ShortMsgKAT_384.txt",
  },
  {
    options: "blake2b --len 64",
    outfile:
      "../../crate/cryptographic_hash_function/blake2/tests/blake2b/ShortMsgKAT_512.txt",
  },
  {
    options: "blake2s --len 16",
    outfile:
      "../../crate/cryptographic_hash_function/blake2/tests/blake2s/ShortMsgKAT_128.txt",
  },
  {
    options: "blake2s --len 20",
    outfile:
      "../../crate/cryptographic_hash_function/blake2/tests/blake2s/ShortMsgKAT_160.txt",
  },
  {
    options: "blake2s --len 28",
    outfile:
      "../../crate/cryptographic_hash_function/blake2/tests/blake2s/ShortMsgKAT_224.txt",
  },
  {
    options: "blake2s --len 32",
    outfile:
      "../../crate/cryptographic_hash_function/blake2/tests/blake2s/ShortMsgKAT_256.txt",
  },
  {
    options: "edonr224",
    outfile:
      "../../crate/cryptographic_hash_function/edonr/tests/edonr/ShortMsgKAT_224.txt",
  },
  {
    options: "edonr256",
    outfile:
      "../../crate/cryptographic_hash_function/edonr/tests/edonr/ShortMsgKAT_256.txt",
  },
  {
    options: "edonr384",
    outfile:
      "../../crate/cryptographic_hash_function/edonr/tests/edonr/ShortMsgKAT_384.txt",
  },
  {
    options: "edonr512",
    outfile:
      "../../crate/cryptographic_hash_function/edonr/tests/edonr/ShortMsgKAT_512.txt",
  },
  {
    options: "keccak224",
    outfile:
      "../../crate/cryptographic_hash_function/keccak/tests/keccak/ShortMsgKAT_224.txt",
  },
  {
    options: "keccak256",
    outfile:
      "../../crate/cryptographic_hash_function/keccak/tests/keccak/ShortMsgKAT_256.txt",
  },
  {
    options: "keccak384",
    outfile:
      "../../crate/cryptographic_hash_function/keccak/tests/keccak/ShortMsgKAT_384.txt",
  },
  {
    options: "keccak512",
    outfile:
      "../../crate/cryptographic_hash_function/keccak/tests/keccak/ShortMsgKAT_512.txt",
  },
  {
    options: "md2",
    outfile:
      "../../crate/cryptographic_hash_function/md2/tests/ShortMsgKAT.txt",
  },
  {
    options: "md4",
    outfile:
      "../../crate/cryptographic_hash_function/md4/tests/ShortMsgKAT.txt",
  },
  {
    options: "md5",
    outfile:
      "../../crate/cryptographic_hash_function/md5/tests/ShortMsgKAT.txt",
  },
  {
    options: "ripemd",
    outfile:
      "../../crate/cryptographic_hash_function/ripemd/tests/ripemd/ShortMsgKAT.txt",
  },
  {
    options: "ripemd128",
    outfile:
      "../../crate/cryptographic_hash_function/ripemd/tests/ripemd/ShortMsgKAT_128.txt",
  },
  {
    options: "ripemd160",
    outfile:
      "../../crate/cryptographic_hash_function/ripemd/tests/ripemd/ShortMsgKAT_160.txt",
  },
  {
    options: "ripemd256",
    outfile:
      "../../crate/cryptographic_hash_function/ripemd/tests/ripemd/ShortMsgKAT_256.txt",
  },
  {
    options: "ripemd320",
    outfile:
      "../../crate/cryptographic_hash_function/ripemd/tests/ripemd/ShortMsgKAT_320.txt",
  },
  {
    options: "sha0",
    outfile:
      "../../crate/cryptographic_hash_function/sha0/tests/ShortMsgKAT.txt",
  },
  {
    options: "sha1",
    outfile:
      "../../crate/cryptographic_hash_function/sha1/tests/ShortMsgKAT.txt",
  },
  {
    options: "sha224",
    outfile:
      "../../crate/cryptographic_hash_function/sha2/tests/sha2/ShortMsgKAT_224.txt",
  },
  {
    options: "sha256",
    outfile:
      "../../crate/cryptographic_hash_function/sha2/tests/sha2/ShortMsgKAT_256.txt",
  },
  {
    options: "sha384",
    outfile:
      "../../crate/cryptographic_hash_function/sha2/tests/sha2/ShortMsgKAT_384.txt",
  },
  {
    options: "sha512",
    outfile:
      "../../crate/cryptographic_hash_function/sha2/tests/sha2/ShortMsgKAT_512.txt",
  },
  {
    options: "sha512-trunc224",
    outfile:
      "../../crate/cryptographic_hash_function/sha2/tests/sha2/ShortMsgKAT_512_trunc_224.txt",
  },
  {
    options: "sha512-trunc256",
    outfile:
      "../../crate/cryptographic_hash_function/sha2/tests/sha2/ShortMsgKAT_512_trunc_256.txt",
  },
  {
    options: "sha3-224",
    outfile:
      "../../crate/cryptographic_hash_function/sha3/tests/sha3/ShortMsgKAT_224.txt",
  },
  {
    options: "sha3-256",
    outfile:
      "../../crate/cryptographic_hash_function/sha3/tests/sha3/ShortMsgKAT_256.txt",
  },
  {
    options: "sha3-384",
    outfile:
      "../../crate/cryptographic_hash_function/sha3/tests/sha3/ShortMsgKAT_384.txt",
  },
  {
    options: "sha3-512",
    outfile:
      "../../crate/cryptographic_hash_function/sha3/tests/sha3/ShortMsgKAT_512.txt",
  },
  {
    options: "shake128 --len 16",
    outfile:
      "../../crate/cryptographic_hash_function/sha3/tests/shake128/ShortMsgKAT_128.txt",
  },
  {
    options: "shake128 --len 32",
    outfile:
      "../../crate/cryptographic_hash_function/sha3/tests/shake128/ShortMsgKAT_256.txt",
  },
  {
    options: "shake128 --len 64",
    outfile:
      "../../crate/cryptographic_hash_function/sha3/tests/shake128/ShortMsgKAT_512.txt",
  },
  {
    options: "shake128 --len 128",
    outfile:
      "../../crate/cryptographic_hash_function/sha3/tests/shake128/ShortMsgKAT_1024.txt",
  },
  {
    options: "shake256 --len 16",
    outfile:
      "../../crate/cryptographic_hash_function/sha3/tests/shake256/ShortMsgKAT_128.txt",
  },
  {
    options: "shake256 --len 32",
    outfile:
      "../../crate/cryptographic_hash_function/sha3/tests/shake256/ShortMsgKAT_256.txt",
  },
  {
    options: "shake256 --len 64",
    outfile:
      "../../crate/cryptographic_hash_function/sha3/tests/shake256/ShortMsgKAT_512.txt",
  },
  {
    options: "shake256 --len 128",
    outfile:
      "../../crate/cryptographic_hash_function/sha3/tests/shake256/ShortMsgKAT_1024.txt",
  },
];

for (const { options, outfile } of TARGETS) {
  let args: string[] = [];
  if (Deno.args.includes("--enable-hmac")) {
    args.push("--hmac");
  }
  args = [...args, ...options.split(" ")];
  const command = new Deno.Command("cargo", {
    args: [
      "run",
      "--",
      ...args,
    ],
    stdin: "piped",
    stdout: "piped",
  });
  const child = command.spawn();
  child.stdout.pipeTo(
    Deno.openSync(outfile, { write: true, create: true }).writable,
  );
  child.stdin.close();
}
