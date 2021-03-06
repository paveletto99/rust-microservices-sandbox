FROM scratch

COPY target/x86_64-unknown-linux-musl/release/RustMicroservicesSandbox /
COPY www /

EXPOSE 9000

ENTRYPOINT [ "/RustMicroservicesSandbox" ]