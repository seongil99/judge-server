FROM rust:latest as builder
WORKDIR /usr/local/bin/online_judge
RUN ["cargo", "init"]
COPY ./Cargo.* .
COPY ./src ./src
COPY ./test_code ./test_code
RUN apt-get update && apt-get install -y gcc && apt-get install -y libseccomp-dev && cargo build --release

FROM ubuntu:latest
WORKDIR /usr/local/bin/online_judge
COPY --from=builder /usr/local/bin/online_judge .
RUN apt-get update && apt-get install -y gcc
CMD ["./target/release/online_judge"]

