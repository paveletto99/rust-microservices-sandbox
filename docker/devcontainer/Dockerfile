#FROM debian:bullseye-slim
FROM debian:stable-slim

RUN apt-get update && \
    apt-get -y dselect-upgrade && \
    apt-get install -y build-essential procps curl git && \
    apt-get clean && rm -rf /var/lib/apt/lists/* && \
    echo "=== Downloading aarch64-linux-musl-cross.tgz" && \
    curl --proto '=https' --tlsv1.2 -sSf -o /tmp/aarch64-linux-musl-cross.tgz https://more.musl.cc/10/x86_64-linux-musl/aarch64-linux-musl-cross.tgz && \
    echo "=== Downloading x86_64-linux-musl-cross.tgz" && \
    curl --proto '=https' --tlsv1.2 -sSf -o /tmp/x86_64-linux-musl-cross.tgz https://more.musl.cc/10/x86_64-linux-musl/x86_64-linux-musl-cross.tgz && \
    mkdir /opt/musl-toolchains && \
    { { [ -f /tmp/aarch64-linux-musl-cross.tgz ] && tar zxf /tmp/aarch64-linux-musl-cross.tgz -C /opt/musl-toolchains; } || true; } && \
    { { [ -f /tmp/x86_64-linux-musl-cross.tgz ] && tar zxf /tmp/x86_64-linux-musl-cross.tgz -C /opt/musl-toolchains; } || true; } && \
    rm -f /tmp/*.tgz && \
    cd /opt/musl-toolchains/aarch64-linux-musl-cross/bin && ln -sfr aarch64-linux-musl-cc cc && \
    cd /opt/musl-toolchains/x86_64-linux-musl-cross/bin && ln -sfr x86_64-linux-musl-cc cc && cd && \
    echo "=== Download and install Rust" && \
    curl --proto '=https' --tlsv1.2 -sSf -o /tmp/rustup-init.sh https://sh.rustup.rs && \
    chmod 700 /tmp/rustup-init.sh && \
    /tmp/rustup-init.sh -q -y -t aarch64-unknown-linux-musl x86_64-unknown-linux-musl && \
    rm -f /tmp/*.sh

ENV PATH "/root/.cargo/bin:${ARM64_BIN_PATH}:${AMD64_BIN_PATH}:${PATH}"

COPY ./cargo-config /root/.cargo/config

CMD [ "bash" ]