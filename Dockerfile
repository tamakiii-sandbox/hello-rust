FROM rust:1.41.0 AS production-pseudo

RUN apt update && apt install -y --no-install-recommends make less && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*

# --

FROM production-pseudo AS development

RUN apt update && apt install -y --no-install-recommends \
      git \
      rust-lldb \
      gdb \
      libclang-dev \
      && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-edit cargo-inspect cargo-rls-install && \
    cargo install --no-default-features --features ci-autoclean cargo-cache

# https://rust-lang.github.io/rustup-components-history/
RUN rustup toolchain add nightly-2020-02-14 && \
    rustup default nightly-2020-02-14 && \
    rustup component add \
      rustfmt  \
      rust-analysis \
      rust-src \
      rls \
      && \
    cargo cache --autoclean
