FROM rust:1.41.0-alpine3.11 AS production-pseudo

RUN apk add --no-cache make

# --

FROM production-pseudo AS development

ENV PAGER=less
RUN apk add --no-cache \
      git \
      bash \
      man \
      man-pages \
      coreutils-doc \
      cargo-doc \
      rust-doc \
      && \
    rustup component add rustfmt

RUN mkdir -p ~/.local/share/bash-completion/completions && \
    rustup completions bash >> ~/.local/share/bash-completion/completions/rustup
