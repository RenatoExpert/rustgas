FROM rust:1.78.0-alpine3.19
WORKDIR /app
COPY . .
RUN cargo build
CMD cargo run

