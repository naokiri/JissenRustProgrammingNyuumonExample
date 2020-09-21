FROM rust:1.43 AS builder

WORKDIR /dockerbuild

COPY Cargo.toml Cargo.toml
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs

RUN cargo build --release

COPY ./src ./src
COPY ./templates ./templates

RUN rm -f target/release/deps/todo*

RUN cargo build --release

FROM debian:stable-slim

COPY --from=builder /dockerbuild/target/release/todo /usr/local/bin/todo

CMD ["todo"]