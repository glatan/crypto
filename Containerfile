FROM docker.io/library/rust:1.80.0-slim-bookworm

WORKDIR /workdir

ENV CARGO_HOME='/cargo' \
    PATH="${PATH}:/cargo/bin" \
    STABLE_RUST_VERSION='1.80.0' \
    WASMTIME_VERSION='23.0.1' \
    TARGETS='\
        aarch64-unknown-linux-gnu \
        aarch64-unknown-linux-musl \
        arm-unknown-linux-gnueabi \
        arm-unknown-linux-gnueabihf \
        arm-unknown-linux-musleabi \
        arm-unknown-linux-musleabihf \
        armv5te-unknown-linux-gnueabi \
        armv5te-unknown-linux-musleabi \
        armv7-unknown-linux-gnueabi \
        armv7-unknown-linux-gnueabihf \
        armv7-unknown-linux-musleabi \
        armv7-unknown-linux-musleabihf \
        i586-unknown-linux-gnu \
        i586-unknown-linux-musl \
        i686-unknown-linux-gnu \
        i686-unknown-linux-musl \
        powerpc-unknown-linux-gnu \
        powerpc64-unknown-linux-gnu \
        powerpc64le-unknown-linux-gnu \
        riscv64gc-unknown-linux-gnu \
        s390x-unknown-linux-gnu \
        sparc64-unknown-linux-gnu \
        thumbv7neon-unknown-linux-gnueabihf \
        x86_64-unknown-linux-gnu \
        x86_64-unknown-linux-musl \
        wasm32-wasi \
        wasm32-wasip1'

RUN \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    --mount=type=cache,target=/var/cache/apt,sharing=locked \
    apt-get -Uy install \
        curl \
        qemu-user \
        xz-utils \
        gcc-aarch64-linux-gnu \
        gcc-i686-linux-gnu \
        gcc \
        gcc-arm-linux-gnueabi \
        gcc-arm-linux-gnueabihf \
        gcc-powerpc-linux-gnu \
        gcc-powerpc64-linux-gnu \
        gcc-powerpc64le-linux-gnu \
        gcc-riscv64-linux-gnu \
        gcc-s390x-linux-gnu \
        gcc-sparc64-linux-gnu \
        # Tier 3
        gcc-mips-linux-gnu \
        gcc-mips64-linux-gnuabi64 \
        gcc-mipsisa32r6-linux-gnu \
        gcc-mipsisa64r6-linux-gnuabi64 \
        gcc-mipsel-linux-gnu \
        gcc-mips64el-linux-gnuabi64 \
        gcc-mipsisa32r6el-linux-gnu \
        gcc-mipsisa64r6el-linux-gnuabi64 

RUN \
    curl -L "https://github.com/bytecodealliance/wasmtime/releases/download/v${WASMTIME_VERSION}/wasmtime-v${WASMTIME_VERSION}-x86_64-linux.tar.xz" > wasmtime-v${WASMTIME_VERSION}-x86_64-linux.tar.xz && \
    tar -xvf wasmtime-v${WASMTIME_VERSION}-x86_64-linux.tar.xz && \
    mv wasmtime-v${WASMTIME_VERSION}-x86_64-linux/wasmtime /usr/local/bin/

RUN \
    curl https://sh.rustup.rs -sSf > install.sh && \
    sh install.sh -y --default-toolchain ${STABLE_RUST_VERSION} --profile minimal && \
    rustup target add ${TARGETS} && \
    rustup default nightly && \
    rustup target add ${TARGETS}

RUN \
    rustup component add rust-src
