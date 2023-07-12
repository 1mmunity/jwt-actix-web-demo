FROM rust:latest AS builder
ARG SQLX_OFFLINE=true 

COPY . .
COPY .env.production .env
RUN cargo build --release

FROM debian:bullseye
RUN apt-get update && apt-get -y upgrade
COPY --from=builder ./target/release/userauth ./target/release/userauth
EXPOSE 3001
CMD ["./target/release/userauth"]
