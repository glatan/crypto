# Crypto

`#![no_std]`で暗号を実装していくプロジェクト

## 実装しているハッシュ関数

* BLAKE-{28, 32, 48, 64, 224, 256, 384, 512}
* BLAKE2s
* BLAKE2b
* EDON-R{224, 256, 384, 512}
* Keccak-f{200, 400, 800, 1600}
* Keccak-{224, 256, 384, 512}
* MD2
* MD4
* MD5
* RIPEMD-{128, 160, 256, 320}
* SHA-0
* SHA-1
* SHA-{224, 256, 384, 512, 512/224, 512/256}
* SHA3-{224, 256, 384, 512}
* SHAKE128, SHAKE256

## Example

* WebAssemblyターゲットでビルドしたデモが[ここ](https://ghash.glatan.vercel.app/)にあります。
* PSP(PlayStation Portable)ターゲットのデモは[ここ](https://gitlab.com/glatan/ghash-psp)。

## Features

### default

以下の機能が利用可能

- BLAKE2
- SHA-2
- SHA-3

### all

全ての機能を有効化する。

## License

This project is licensed under GPL-3.0-only ([LICENSE](./LICENSE) or https://www.gnu.org/licenses/gpl-3.0-standalone.html)
