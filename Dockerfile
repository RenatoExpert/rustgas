FROM rust:1.78.0-alpine3.19
RUN apk add git
WORKDIR /app
COPY . .
RUN git submodule init
RUN mkdir -p /var/rustgas
RUN ln -s /app/aga_tables /var/rustgas
RUN git submodule update
RUN cargo build
CMD cargo run

