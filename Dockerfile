FROM rust:1.41.0-alpine3.11 AS production-pseudo

RUN apk add --no-cache make bash && \
    sed -i -e 's|/bin/ash|/bin/bash|' /etc/passwd

# --

FROM production-pseudo AS development

ENV PAGER=less
RUN apk add --no-cache \
      git \
      bash-doc \
      bash-completion \
      man \
      man-pages \
      coreutils-doc \
      cargo-doc \
      rust-doc \
      && \
    rustup component add rustfmt

RUN mkdir -p ~/.local/share/bash-completion/completions && \
    rustup completions bash >> ~/.local/share/bash-completion/completions/rustup && \
    # rustup toolchain add nightly && \
    rustup component add rust-analysis && \
    rustup component add rust-src && \
    rustup component add rls
