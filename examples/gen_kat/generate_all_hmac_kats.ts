interface Target {
  options: string;
  outfile: string;
}

const TARGETS = [
  {
    options: "--hmac blake28",
    outfile:
      "../../crate/cryptographic_hash_function/blake/tests/round2/ShortMsgKAT_HMAC_224.txt",
  },
  {
    options: "--hmac blake32",
    outfile:
      "../../crate/cryptographic_hash_function/blake/tests/round2/ShortMsgKAT_HMAC_256.txt",
  },
  {
    options: "--hmac blake48",
    outfile:
      "../../crate/cryptographic_hash_function/blake/tests/round2/ShortMsgKAT_HMAC_384.txt",
  },
  {
    options: "--hmac blake64",
    outfile:
      "../../crate/cryptographic_hash_function/blake/tests/round2/ShortMsgKAT_HMAC_512.txt",
  },
  {
    options: "--hmac blake224",
    outfile:
      "../../crate/cryptographic_hash_function/blake/tests/round3/ShortMsgKAT_HMAC_224.txt",
  },
  {
    options: "--hmac blake256",
    outfile:
      "../../crate/cryptographic_hash_function/blake/tests/round3/ShortMsgKAT_HMAC_256.txt",
  },
  {
    options: "--hmac blake384",
    outfile:
      "../../crate/cryptographic_hash_function/blake/tests/round3/ShortMsgKAT_HMAC_384.txt",
  },
  {
    options: "--hmac blake512",
    outfile:
      "../../crate/cryptographic_hash_function/blake/tests/round3/ShortMsgKAT_HMAC_512.txt",
  },
  {
    options: "--hmac blake2b --len 28",
    outfile:
      "../../crate/cryptographic_hash_function/blake2/tests/blake2b/ShortMsgKAT_HMAC_224.txt",
  },
  {
    options: "--hmac blake2b --len 32",
    outfile:
      "../../crate/cryptographic_hash_function/blake2/tests/blake2b/ShortMsgKAT_HMAC_256.txt",
  },
  {
    options: "--hmac blake2b --len 48",
    outfile:
      "../../crate/cryptographic_hash_function/blake2/tests/blake2b/ShortMsgKAT_HMAC_384.txt",
  },
  {
    options: "--hmac blake2b --len 64",
    outfile:
      "../../crate/cryptographic_hash_function/blake2/tests/blake2b/ShortMsgKAT_HMAC_512.txt",
  },
  {
    options: "--hmac blake2s --len 16",
    outfile:
      "../../crate/cryptographic_hash_function/blake2/tests/blake2s/ShortMsgKAT_HMAC_128.txt",
  },
  {
    options: "--hmac blake2s --len 20",
    outfile:
      "../../crate/cryptographic_hash_function/blake2/tests/blake2s/ShortMsgKAT_HMAC_160.txt",
  },
  {
    options: "--hmac blake2s --len 28",
    outfile:
      "../../crate/cryptographic_hash_function/blake2/tests/blake2s/ShortMsgKAT_HMAC_224.txt",
  },
  {
    options: "--hmac blake2s --len 32",
    outfile:
      "../../crate/cryptographic_hash_function/blake2/tests/blake2s/ShortMsgKAT_HMAC_256.txt",
  },
  {
    options: "--hmac edonr224",
    outfile:
      "../../crate/cryptographic_hash_function/edonr/tests/edonr/ShortMsgKAT_HMAC_224.txt",
  },
  {
    options: "--hmac edonr256",
    outfile:
      "../../crate/cryptographic_hash_function/edonr/tests/edonr/ShortMsgKAT_HMAC_256.txt",
  },
  {
    options: "--hmac edonr384",
    outfile:
      "../../crate/cryptographic_hash_function/edonr/tests/edonr/ShortMsgKAT_HMAC_384.txt",
  },
  {
    options: "--hmac edonr512",
    outfile:
      "../../crate/cryptographic_hash_function/edonr/tests/edonr/ShortMsgKAT_HMAC_512.txt",
  },
  {
    options: "--hmac keccak224",
    outfile:
      "../../crate/cryptographic_hash_function/keccak/tests/keccak/ShortMsgKAT_HMAC_224.txt",
  },
  {
    options: "--hmac keccak256",
    outfile:
      "../../crate/cryptographic_hash_function/keccak/tests/keccak/ShortMsgKAT_HMAC_256.txt",
  },
  {
    options: "--hmac keccak384",
    outfile:
      "../../crate/cryptographic_hash_function/keccak/tests/keccak/ShortMsgKAT_HMAC_384.txt",
  },
  {
    options: "--hmac keccak512",
    outfile:
      "../../crate/cryptographic_hash_function/keccak/tests/keccak/ShortMsgKAT_HMAC_512.txt",
  },
  {
    options: "--hmac md2",
    outfile:
      "../../crate/cryptographic_hash_function/md2/tests/ShortMsgKAT_HMAC.txt",
  },
  {
    options: "--hmac md4",
    outfile:
      "../../crate/cryptographic_hash_function/md4/tests/ShortMsgKAT_HMAC.txt",
  },
  {
    options: "--hmac md5",
    outfile:
      "../../crate/cryptographic_hash_function/md5/tests/ShortMsgKAT_HMAC.txt",
  },
  {
    options: "--hmac ripemd",
    outfile:
      "../../crate/cryptographic_hash_function/ripemd/tests/ripemd/ShortMsgKAT_HMAC.txt",
  },
  {
    options: "--hmac ripemd128",
    outfile:
      "../../crate/cryptographic_hash_function/ripemd/tests/ripemd/ShortMsgKAT_HMAC_128.txt",
  },
  {
    options: "--hmac ripemd160",
    outfile:
      "../../crate/cryptographic_hash_function/ripemd/tests/ripemd/ShortMsgKAT_HMAC_160.txt",
  },
  {
    options: "--hmac ripemd256",
    outfile:
      "../../crate/cryptographic_hash_function/ripemd/tests/ripemd/ShortMsgKAT_HMAC_256.txt",
  },
  {
    options: "--hmac ripemd320",
    outfile:
      "../../crate/cryptographic_hash_function/ripemd/tests/ripemd/ShortMsgKAT_HMAC_320.txt",
  },
  {
    options: "--hmac sha0",
    outfile:
      "../../crate/cryptographic_hash_function/sha0/tests/ShortMsgKAT_HMAC.txt",
  },
  {
    options: "--hmac sha1",
    outfile:
      "../../crate/cryptographic_hash_function/sha1/tests/ShortMsgKAT_HMAC.txt",
  },
  {
    options: "--hmac sha224",
    outfile:
      "../../crate/cryptographic_hash_function/sha2/tests/sha2/ShortMsgKAT_HMAC_224.txt",
  },
  {
    options: "--hmac sha256",
    outfile:
      "../../crate/cryptographic_hash_function/sha2/tests/sha2/ShortMsgKAT_HMAC_256.txt",
  },
  {
    options: "--hmac sha384",
    outfile:
      "../../crate/cryptographic_hash_function/sha2/tests/sha2/ShortMsgKAT_HMAC_384.txt",
  },
  {
    options: "--hmac sha512",
    outfile:
      "../../crate/cryptographic_hash_function/sha2/tests/sha2/ShortMsgKAT_HMAC_512.txt",
  },
  {
    options: "--hmac sha3-224",
    outfile:
      "../../crate/cryptographic_hash_function/sha3/tests/sha3/ShortMsgKAT_HMAC_224.txt",
  },
  {
    options: "--hmac sha3-256",
    outfile:
      "../../crate/cryptographic_hash_function/sha3/tests/sha3/ShortMsgKAT_HMAC_256.txt",
  },
  {
    options: "--hmac sha3-384",
    outfile:
      "../../crate/cryptographic_hash_function/sha3/tests/sha3/ShortMsgKAT_HMAC_384.txt",
  },
  {
    options: "--hmac sha3-512",
    outfile:
      "../../crate/cryptographic_hash_function/sha3/tests/sha3/ShortMsgKAT_HMAC_512.txt",
  },
  {
    options: "--hmac shake128 --len 16",
    outfile:
      "../../crate/cryptographic_hash_function/sha3/tests/shake128/ShortMsgKAT_HMAC_128.txt",
  },
  {
    options: "--hmac shake128 --len 32",
    outfile:
      "../../crate/cryptographic_hash_function/sha3/tests/shake128/ShortMsgKAT_HMAC_256.txt",
  },
  {
    options: "--hmac shake128 --len 64",
    outfile:
      "../../crate/cryptographic_hash_function/sha3/tests/shake128/ShortMsgKAT_HMAC_512.txt",
  },
  {
    options: "--hmac shake128 --len 128",
    outfile:
      "../../crate/cryptographic_hash_function/sha3/tests/shake128/ShortMsgKAT_HMAC_1024.txt",
  },
  {
    options: "--hmac shake256 --len 16",
    outfile:
      "../../crate/cryptographic_hash_function/sha3/tests/shake256/ShortMsgKAT_HMAC_128.txt",
  },
  {
    options: "--hmac shake256 --len 32",
    outfile:
      "../../crate/cryptographic_hash_function/sha3/tests/shake256/ShortMsgKAT_HMAC_256.txt",
  },
  {
    options: "--hmac shake256 --len 64",
    outfile:
      "../../crate/cryptographic_hash_function/sha3/tests/shake256/ShortMsgKAT_HMAC_512.txt",
  },
  {
    options: "--hmac shake256 --len 128",
    outfile:
      "../../crate/cryptographic_hash_function/sha3/tests/shake256/ShortMsgKAT_HMAC_1024.txt",
  },
];

for (const { options, outfile } of TARGETS) {
  const command = new Deno.Command("cargo", {
    args: [
      "run",
      "--",
      ...options.split(" "),
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
