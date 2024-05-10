FROM rust:1.78.0-alpine3.19 as build
WORKDIR /app
ADD https://github.com/renatoexpert/aga_tables/archive/main.zip /tmp/main.zip
RUN unzip /tmp/main.zip -d /tmp/aga_tables	&& \
	cp -r /tmp/aga_tables/* /app/aga_tables	&& \
	mkdir -p /var/rustgas			&& \
	ln -s /app/aga_tables /var/rustgas
RUN cargo init
COPY Cargo.toml Cargo.lock .
RUN cargo build --release			&& \
	rm target/release/deps/rustgas*
COPY src src
RUN cargo build --release
CMD cargo run

FROM scratch as release
WORKDIR /var/rustgas
COPY --from=build aga_tables .
WORKDIR /release
COPY --from=build /app/target/release .
CMD ["/release/rustgas"]

