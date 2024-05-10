FROM rust:1.78.0-alpine3.19
RUN apk add git
WORKDIR /app
COPY . .
RUN git submodule init && ln -s aga_tables /src
RUN cargo build
CMD cargo run

