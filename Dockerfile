
FROM rust:1.61 as builder

RUN apt update
RUN apt install -y libpq-dev
RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR ./app
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./files ./files
COPY ./migrations ./migrations
RUN cargo build --release
RUN cargo install --path .
ENTRYPOINT [ "btc_tx" ]

# FROM alpine:3.16
# WORKDIR /app
# COPY ./files ./files
# COPY ./migrations ./migrations
# COPY --from=builder /app/target/release/btc_tx .
# CMD [ "./btc_tx" ]
