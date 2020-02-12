FROM rust:1.41.0 AS production-pseudo

RUN apt update && apt install -y --no-install-recommends make less && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*

# --

FROM production-pseudo AS development

RUN cargo install cargo-edit

RUN apt update && apt install -y --no-install-recommends \
      git \
      rust-lldb \
      gdb \
      python3 \
      && \
    rustup toolchain add nightly && \
    rustup component add \
      rustfmt  \
      rust-analysis \
      rust-src \
      rls \
      && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*