FROM rust:1.87-slim-bookworm

WORKDIR /var/www/html

RUN \
    apt-get update -y ; \
    apt-get install  -y --no-install-recommends \
        libssl-dev \
        make \
        perl \
        pkg-config

RUN \
    rustup component add rustfmt

RUN \
    cargo install dioxus-cli --version 0.6.3

#CMD ["dx", "serve", "--addr", "0.0.0.0"]
