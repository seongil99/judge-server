FROM ubuntu:latest

WORKDIR /usr/local/bin/online_judge

RUN ["apt-get", "update"]
RUN ["apt-get",  "install", "-y",  "gcc"]
RUN ["apt-get",  "install", "-y",  "libseccomp-dev"]
RUN ["apt-get",  "install", "-y",  "curl"]
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN ["rustc", "--version"]

COPY . .

RUN ["cargo", "build", "--release"]
RUN rm -rf target/deps/online_judge*

CMD ["./target/release/online_judge"]
