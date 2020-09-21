# Just to run on docker image. Definitely have to fix migrations and persistence of DB for real use.

FROM rust:1.43 AS builder

WORKDIR /dockerbuild

COPY Cargo.toml Cargo.toml
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs

RUN cargo build --release

COPY ./src ./src
COPY ./templates ./templates
COPY ./migrations ./migrations

RUN cargo install diesel_cli --no-default-features --features sqlite
RUN diesel setup --database-url "./todo.db"
RUN diesel migration run --database-url "./todo.db"

RUN rm -f target/release/deps/todo*

RUN cargo build --release

FROM debian:stable

RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get -yq --no-install-recommends install sqlite3=3.* && \
	rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
COPY --from=builder /dockerbuild/target/release/todo /usr/local/bin/todo
RUN mkdir -p /var/db
COPY --from=builder /dockerbuild/todo.db /var/db/todo.db
WORKDIR /var/db/

CMD ["todo"]