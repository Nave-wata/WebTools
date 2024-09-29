FROM rust:1.81.0-slim-bullseye

WORKDIR /var/www/html

RUN \
    apt-get update -y ; \
    apt-get install  -y --no-install-recommends \
        libssl-dev \
        make \
        perl

RUN \
    rustup component add rustfmt

RUN \
    cargo install dioxus-cli

CMD ["dx", "serve", "--hot-reload", "--platform", "web"]
