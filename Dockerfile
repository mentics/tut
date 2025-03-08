FROM scratch
LABEL org.opencontainers.image.source="https://github.com/mentics/tut"

COPY target/x86_64-unknown-linux-musl/release/tut /

CMD ["/tut"]