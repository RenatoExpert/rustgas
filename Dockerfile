FROM rust:1.78.0-alpine3.19
RUN apk add git
WORKDIR /app
COPY Cargo.toml Cargo.lock .
RUN cargo install
COPY .gitmodules .
RUN git submodule init				&& \
	mkdir -p /var/rustgas			&& \
	ln -s /app/aga_tables /var/rustgas	&& \
	git submodule update
COPY src .
RUN cargo build
CMD cargo run

