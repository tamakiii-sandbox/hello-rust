FROM rust:1.41.0-alpine3.11 AS production-pseudo

RUN apk add --no-cache --virtual .builddeps \
      musl-dev \
      openssl-dev \
      && \
    apk add --no-cache \
      make \
      bash \
      git \
      && \
    rustup toolchain add nightly && \
    rustup component add \
      rustfmt  \
      rust-analysis \
      rust-src \
      rls \
      && \
    sed -i -e 's|/bin/ash|/bin/bash|' /etc/passwd

# --

FROM production-pseudo AS development

ENV RUSTFLAGS -C target-feature=-crt-static

RUN cargo install cargo-edit

RUN apk add --no-cache curl bind-tools mysql-client
RUN apk add --no-cache --repository http://dl-cdn.alpinelinux.org/alpine/v3.5/main python3~=3.5
