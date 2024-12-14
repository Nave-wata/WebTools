FROM rust:1.81.0-slim-bullseye

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
    cargo install dioxus-cli --version 0.6.0

CMD ["dx", "serve", "--addr", "0.0.0.0"]
