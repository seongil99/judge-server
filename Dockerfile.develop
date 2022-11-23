FROM rust:alpine

RUN apk add --no-cache libseccomp-dev
RUN apk add --no-cache libseccomp-static
RUN apk add --no-cache libseccomp
RUN apk add --no-cache vim
RUN apk add --no-cache git
RUN apk add --no-cache build-base

RUN mkdir /app
WORKDIR /app

COPY . .
RUN rustup component add rustfmt

CMD ["/bin/sh"]