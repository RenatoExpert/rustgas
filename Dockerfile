FROM rust:1.78.0-alpine3.19
RUN apk add git
WORKDIR /app
RUN cargo init
COPY Cargo.toml Cargo.lock .
RUN cargo build --release			&& \
	rm target/release/deps/rustgas*
COPY .gitmodules .
RUN git submodule init				&& \
	mkdir -p /var/rustgas			&& \
	ln -s /app/aga_tables /var/rustgas	&& \
	git submodule update
COPY src src
RUN cargo build --release
CMD cargo run

