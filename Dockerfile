FROM rust:1.41.0-alpine3.11 AS production-pseudo

RUN apk add --no-cache make

# --

FROM production-pseudo AS development

ENV PAGER=less
RUN apk add --no-cache man man-pages coreutils-doc cargo-doc
