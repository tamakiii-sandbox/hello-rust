FROM rust:1.41.0 AS production-pseudo

RUN apt update && apt install -y --no-install-recommends make less && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*

# --

FROM production-pseudo AS development

RUN cargo install cargo-edit cargo-inspect cargo-rls-install

RUN apt update && apt install -y --no-install-recommends \
      git \
      rust-lldb \
      gdb \
      libclang-dev \
      python3 \
      && \
    # https://rust-lang.github.io/rustup-components-history/
    rustup toolchain add nightly-2020-02-14 && \
    rustup default nightly-2020-02-14 && \
    rustup component add \
      rustfmt  \
      rust-analysis \
      rust-src \
      rls \
      && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*