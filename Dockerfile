FROM rust:1.78.0-alpine3.19
ADD https://github.com/renatoexpert/aga_tables /src
WORKDIR /app
COPY . .
RUN cargo build
CMD cargo run

