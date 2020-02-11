FROM rust:1.41.0 AS production-pseudo

RUN apt update && apt install -y --no-install-recommends make less && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*

# RUN apk add --no-cache --virtual .builddeps \
#       musl-dev \
#       openssl-dev \
#       && \
#     apk add --no-cache \
#       make \
#       bash \
#       git \
#       && \
#     rustup toolchain add nightly && \
#     rustup component add \
#       rustfmt  \
#       rust-analysis \
#       rust-src \
#       rls \
#       && \
#     sed -i -e 's|/bin/ash|/bin/bash|' /etc/passwd

# --

FROM production-pseudo AS development

# ENV RUSTFLAGS -C target-feature=-crt-static

# RUN cargo install cargo-edit

# RUN apk add --no-cache curl bind-tools mysql-client
# RUN apk add --no-cache --repository http://dl-cdn.alpinelinux.org/alpine/v3.5/main python3~=3.5

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

# RUN echo 0 > /proc/sys/kernel/yama/ptrace_scope
# --privileged
