#FROM debian:bullseye-slim
FROM debian:stable-slim

RUN apt-get update && \
    apt-get -y dselect-upgrade && \
    apt-get install -y build-essential procps curl git && \
    apt-get clean && rm -rf /var/lib/apt/lists/* && \
    echo "=== Downloading SHA512SUMS" && \
    curl --proto '=https' --tlsv1.2 -sSf https://musl.cc/SHA512SUMS | grep -E '(aarch64-linux-musl-cross.tgz|x86_64-linux-musl-cross.tgz)' > /tmp/SHA512SUMS && \
    echo "=== Downloading aarch64-linux-musl-cross.tgz" && \
    curl --proto '=https' --tlsv1.2 -sSf -o /tmp/aarch64-linux-musl-cross.tgz https://musl.cc/aarch64-linux-musl-cross.tgz && \
    echo "=== Downloading x86_64-linux-musl-cross.tgz" && \
    curl --proto '=https' --tlsv1.2 -sSf -o /tmp/x86_64-linux-musl-cross.tgz https://musl.cc/x86_64-linux-musl-cross.tgz && \
    echo "=== Check SHA512SUMS" && \
    cd /tmp && sha512sum -c SHA512SUMS && \
    mkdir /opt/musl-toolchains && \
    { { [ -f /tmp/aarch64-linux-musl-cross.tgz ] && tar zxf /tmp/aarch64-linux-musl-cross.tgz -C /opt/musl-toolchains; } || true; } && \
    { { [ -f /tmp/x86_64-linux-musl-cross.tgz ] && tar zxf /tmp/x86_64-linux-musl-cross.tgz -C /opt/musl-toolchains; } || true; } && \
    cd /opt/musl-toolchains/aarch64-linux-musl-cross/bin && ln -sfr aarch64-linux-musl-cc cc && \
    cd /opt/musl-toolchains/x86_64-linux-musl-cross/bin && ln -sfr x86_64-linux-musl-cc cc && cd && \
    echo "=== Download and install Rust" && \
    curl --proto '=https' --tlsv1.2 -sSf -o /tmp/rustup-init.sh https://sh.rustup.rs && \
    chmod 700 /tmp/rustup-init.sh && \
    /tmp/rustup-init.sh -q -y -t aarch64-unknown-linux-musl x86_64-unknown-linux-musl && \
    rm -f /tmp/*.gz /tmp/*.sh /tmp/SHA512SUMS

ENV PATH "/root/.cargo/bin:${PATH}"

COPY ./cargo-config /root/.cargo/config

CMD [ "bash" ]