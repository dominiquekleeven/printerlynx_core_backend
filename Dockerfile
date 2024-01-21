FROM alpine:latest as builder

# Install Rust
RUN apk --no-cache add rust cargo openssl-dev pkgconf

WORKDIR /app
RUN cargo init

COPY ./src /app/src
COPY Cargo.toml Cargo.lock /app/

RUN cargo build --release

FROM alpine:latest

RUN apk --no-cache add openssl

EXPOSE 3000

# Copy the binary from the builder image to the final image
COPY --from=builder /app/target/release/printerlynx_core_backend /app/printerlynx_core_backend

WORKDIR /app

CMD ["./printerlynx_core_backend"]