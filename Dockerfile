FROM gladmo/rust:latest as gitstar_bin
RUN curl -o openssl.tar.gz https://www.openssl.org/source/openssl-1.0.2n.tar.gz && \
	tar -zxvf openssl.tar.gz && cd openssl-1.0.2n && ./config --openssldir=/usr/local/openssl && \
	mkdir -p /app/gitstar
ADD . /app/gitstar
WORKDIR /app/gitstar
ENV OPENSSL_DIR /usr/local/openssl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest
RUN mkdir -p /app/gitstar
COPY --from=gitstar_bin /app/gitstar/target/release/main /app/gitstar/gitstar
WORKDIR /app/gitstar
ENV RUST_LOG info
CMD ["./gitstar"]