FROM gladmo/rust:latest as gitstar_bin
RUN mkdir -p /app/gitstar
ADD . /app/gitstar
WORKDIR /app/gitstar
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest
RUN mkdir -p /app/gitstar
COPY --from=gitstar_bin /app/gitstar/target/release/main /app/gitstar/gitstar
WORKDIR /app/gitstar
ENV RUST_LOG info
CMD ["./gitstar"]