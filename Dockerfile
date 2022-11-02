FROM rust:alpine AS chef

WORKDIR /usr/src/online_judge

RUN set -eux; \
  apk add --no-cache musl-dev; \
  cargo install cargo-chef; \
  rm -rf $CARGO_HOME/registry

FROM chef as planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

RUN apk add --no-cache libseccomp-dev

COPY --from=planner /usr/src/online_judge/recipe.json .
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM alpine:latest

WORKDIR /usr/local/bin

RUN apk add --no-cache build-base

COPY --from=builder /usr/src/online_judge/target/release/online_judge .
COPY ./test_code ./test_code

CMD ["./online_judge"]
