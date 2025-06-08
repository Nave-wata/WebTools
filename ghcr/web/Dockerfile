FROM rust:1.87-slim-bookworm

WORKDIR /var/www/html

RUN \
    apt-get update -y && \
    apt-get install  -y --no-install-recommends \
        libssl-dev \
        make \
        perl \
        pkg-config && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

RUN \
    rustup component add rustfmt

RUN \
    cargo install dioxus-cli --version 0.6.3

COPY --from=node:22.16-slim /usr/local /usr/local

RUN npm install -g aws-cdk
