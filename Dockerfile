FROM rust:alpine AS builder

WORKDIR /usr/src/online_judge

RUN apk add --no-cache libseccomp-dev
RUN apk add --no-cache musl-dev

RUN cargo init .
COPY Cargo* ./
RUN cargo build --release && \
  rm target/release/deps/online_judge*

COPY . .
RUN cargo build --release

FROM alpine:latest

WORKDIR /usr/local/bin

RUN apk add --no-cache build-base
RUN apk add --no-cache libseccomp-dev
RUN apk add --no-cache musl-dev

COPY --from=builder /usr/src/online_judge/target/release/online_judge .
COPY ./result ./result
COPY ./test_cases ./test_cases

CMD ["./online_judge"]