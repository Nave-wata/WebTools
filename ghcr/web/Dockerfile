FROM rust:1.87-slim-bookworm

WORKDIR /var/www/html

RUN \
    apt-get update -y && \
    apt-get install  -y --no-install-recommends \
        ca-certificates \
        curl \
        libssl-dev \
        make \
        perl \
        pkg-config \
        unzip &&  \
    \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    \
    curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip" &&  \
    unzip awscliv2.zip &&  \
    ./aws/install &&  \
    rm -rf aws awscliv2.zip

RUN \
    rustup component add rustfmt

RUN \
    cargo install dioxus-cli --version 0.6.3

COPY --from=node:22.16-slim /usr/local /usr/local
