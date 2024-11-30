# Target

## 動作確認環境

### \*-linux-\*

- OS: Debian bookworn
- Host CPU Arch: x86_64

### *-pc-windows-msvc

- OS: Windows Server 2022 
- Host CPU Arch: x86_64

https://docs.gitlab.com/ee/ci/runners/hosted_runners/windows.html

### i686-pc-windows-gnu

*-pc-windows-msvc + MSYS2(MINGW32)

### x86_64-pc-windows-gnu

*-pc-windows-msvc + MSYS2(MINGW64)

### i686-pc-windows-gnullvm

*-pc-windows-msvc + MSYS2(CLANG32)

### x86_64-pc-windows-gnullvm

*-pc-windows-msvc + MSYS2(CLANG64)

### wasm32-*

- OS: Debian bookworn
- Host CPU Arch: x86_64
- Runtime: Wasmtime

## .cargo/config.toml

### Tier 1

|Target|Linker|Runner + Option|Note|
|-|-|-|-|
|aarch64-unknown-linux-gnu|aarch64-linux-gnu-gcc|qemu-aarch64 -L /usr/aarch64-linux-gnu/||
|i686-pc-windows-gnu|i686-w64-mingw32-gcc|||
|i686-pc-windows-msvc||||
|i686-unknown-linux-gnu|i686-linux-gnu-gcc|qemu-i386 -L /usr/i686-linux-gnu/||
|x86_64-pc-windows-gnu|x86_64-w64-mingw32-gcc|||
|x86_64-pc-windows-msvc||||
|x86_64-unknown-linux-gnu||||

### Tier 2 with Host Tools

|Target|Linker|Runner + Option|Note|
|-|-|-|-|
|aarch64-unknown-linux-musl|aarch64-linux-gnu-gcc|qemu-aarch64 -L /usr/aarch64-linux-gnu/||
|arm-unknown-linux-gnueabi|arm-linux-gnueabi-gcc|qemu-arm -L /usr/arm-linux-gnueabi/||
|arm-unknown-linux-gnueabihf|arm-linux-gnueabihf-gcc|qemu-arm -L /usr/arm-linux-gnueabihf/||
|armv7-unknown-linux-gnueabihf|arm-linux-gnueabihf-gcc|qemu-arm -L /usr/arm-linux-gnueabihf/||
|powerpc-unknown-linux-gnu|powerpc-linux-gnu-gcc|qemu-ppc -L /usr/powerpc-linux-gnu/||
|powerpc64-unknown-linux-gnu|powerpc64-linux-gnu-gcc|qemu-ppc64 -L /usr/powerpc64-linux-gnu/||
|powerpc64le-unknown-linux-gnu|powerpc64le-linux-gnu-gcc|qemu-ppc64le -L /usr/powerpc64le-linux-gnu/||
|riscv64gc-unknown-linux-gnu|riscv64-linux-gnu-gcc|qemu-riscv64 -L /usr/riscv64-linux-gnu/||
|s390x-unknown-linux-gnu|s390x-linux-gnu-gcc|qemu-s390x -L /usr/s390x-linux-gnu/||
|x86_64-unknown-linux-musl||||

### Tier 2 without Host Tools

|Target|Linker|Runner + Option|Note|
|-|-|-|-|
|aarch64-pc-windows-gnullvm||lld||
|arm-unknown-linux-musleabi|arm-linux-gnueabi-gcc|qemu-arm -L /usr/arm-linux-gnueabi/||
|arm-unknown-linux-musleabihf|arm-linux-gnueabihf-gcc|qemu-arm -L /usr/arm-linux-gnueabihf/||
|armv5te-unknown-linux-gnueabi|arm-linux-gnueabi-gcc|qemu-arm -L /usr/arm-linux-gnueabi/||
|armv5te-unknown-linux-musleabi|arm-linux-gnueabi-gcc|qemu-arm -L /usr/arm-linux-gnueabi/||
|armv7-unknown-linux-gnueabi|arm-linux-gnueabi-gcc|qemu-arm -L /usr/arm-linux-gnueabi/||
|armv7-unknown-linux-musleabi|arm-linux-gnueabi-gcc|qemu-arm -L /usr/arm-linux-gnueabi/||
|armv7-unknown-linux-musleabihf|arm-linux-gnueabihf-gcc|qemu-arm -L /usr/arm-linux-gnueabihf/||
|i586-unknown-linux-gnu|i686-linux-gnu-gcc|qemu-i386 -L /usr/i686-linux-gnu/||
|i586-unknown-linux-musl|i686-linux-gnu-gcc|qemu-i386 -L /usr/i686-linux-gnu/||
|i686-pc-windows-gnullvm||lld||
|i686-unknown-linux-musl|i686-linux-gnu-gcc|qemu-i386 -L /usr/i686-linux-gnu/||
|sparc64-unknown-linux-gnu|sparc64-linux-gnu-gcc|qemu-sparc64 -L /usr/sparc64-linux-gnu/||
|thumbv7neon-unknown-linux-gnueabihf|arm-linux-gnueabihf-gcc|qemu-arm -L /usr/arm-linux-gnueabihf/||
|wasm32-wasi||wasmtime||
|wasm32-wasip1||wasmtime||
|x86_64-pc-windows-gnullvm||lld||

### Tier 3

|Target|Linker|Runner + Option|Note|
|-|-|-|-|
|armv4t-unknown-linux-gnueabi|arm-linux-gnueabi-gcc|qemu-arm -L /usr/arm-linux-gnueabi/||
|mips-unknown-linux-gnu|mips-linux-gnu-gcc|qemu-mips -L /usr/mips-linux-gnu/||
|mips-unknown-linux-musl|mips-linux-gnu-gcc|qemu-mips -L /usr/mips-linux-gnu/||
|mips64-unknown-linux-gnuabi64|mips64-linux-gnuabi64-gcc|qemu-mips64 -L /usr/mips64-linux-gnuabi64/||
|mips64-unknown-linux-muslabi64|mips64-linux-gnuabi64-gcc|qemu-mips64 -L /usr/mips64-linux-gnuabi64/||
|mips64el-unknown-linux-gnuabi64|mips64el-linux-gnuabi64-gcc|qemu-mips64el -L /usr/mips64el-linux-gnuabi64/||
|mips64el-unknown-linux-muslabi64|mips64el-linux-gnuabi64-gcc|qemu-mips64el -L /usr/mips64el-linux-gnuabi64/||
|mipsel-unknown-linux-gnu|mipsel-linux-gnu-gcc|qemu-mipsel -L /usr/mipsel-linux-gnu/||
|mipsel-unknown-linux-musl|mipsel-linux-gnu-gcc|qemu-mipsel -L /usr/mipsel-linux-gnu/||
|mipsisa32r6-unknown-linux-gnu|mipsisa32r6-linux-gnu-gcc|qemu-mips -L /usr/mipsisa32r6-linux-gnu/||
|mipsisa32r6el-unknown-linux-gnu|mipsisa32r6el-linux-gnu-gcc|qemu-mipsel -L /usr/mipsisa32r6el-linux-gnu/||
|mipsisa64r6-unknown-linux-gnuabi64|mipsisa64r6-linux-gnuabi64-gcc|qemu-mips64 -L /usr/mipsisa64r6-linux-gnuabi64/||
|mipsisa64r6el-unknown-linux-gnuabi64|mipsisa64r6el-linux-gnuabi64-gcc|qemu-mips64el -L /usr/mipsisa64r6el-linux-gnuabi64/||
|powerpc-unknown-linux-musl|powerpc-linux-gnu-gcc|qemu-ppc -L /usr/powerpc-linux-gnu/||
|powerpc64-unknown-linux-musl|powerpc64-linux-gnu-gcc|qemu-ppc64 -L /usr/powerpc64-linux-gnu/||
|powerpc64le-unknown-linux-musl|powerpc64le-linux-gnu-gcc|qemu-ppc64le -L /usr/powerpc64le-linux-gnu/||
|riscv64gc-unknown-linux-musl|riscv64-linux-gnu-gcc|qemu-riscv64 -L /usr/riscv64-linux-gnu/||
|s390x-unknown-linux-musl|s390x-linux-gnu-gcc|qemu-s390x -L /usr/s390x-linux-gnu/||
|thumbv7neon-unknown-linux-musleabihf|arm-linux-gnueabihf-gcc|qemu-arm -L /usr/arm-linux-gnueabihf/||
