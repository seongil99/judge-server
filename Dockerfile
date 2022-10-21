FROM rust:alpine AS builder

WORKDIR /online_judge

RUN cargo init .
COPY ./Cargo* ./
RUN cargo build --release && \
  rm target/release/deps/online_judge*

COPY . .
RUN cargo build --release

FROM ubuntu:latest

WORKDIR /usr/local/bin

COPY --from=builder /online_judge/target/release/online_judge .

CMD ["./online_judge"]