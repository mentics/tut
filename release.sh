#!/bin/bash
# rustup target add x86_64-unknown-linux-musl
# gcloud auth login --no-browser
# gcloud config set project journey-e1c94
cargo build --release --target x86_64-unknown-linux-musl
upx --best --lzma target/x86_64-unknown-linux-musl/release/tut
# upx --ultra-brute target/x86_64-unknown-linux-musl/release/tut
docker build -t us-west1-docker.pkg.dev/journey-e1c94/mentics/tut:latest .
docker push us-west1-docker.pkg.dev/journey-e1c94/mentics/tut:latest

# TODO: consider https://github.com/slimtoolkit/slim ?


# docker pull clux/muslrust:stable
# # the initial slash is necessary when run in windows bash shell
# docker run -v /${PWD}/..:/volume --rm -t clux/muslrust:stable bash -c "cd tut; cargo build --release"
