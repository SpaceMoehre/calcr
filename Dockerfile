# Stage 1: Build the Rust binary
FROM rust:1.82.0 as builder

# Set the working directory inside the container
WORKDIR /app

# Copy the project files into the container
COPY . .

# Build the release version of the binary
RUN rustup target add x86_64-unknown-linux-musl && cargo build --release --target=x86_64-unknown-linux-musl

# Stage 2: Create a minimal image with the binary
FROM debian:buster-slim

# Set the working directory inside the container
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/calcr /usr/local/bin/calcr
COPY --from=builder /app/templates /app/templates
ENV CARGO_MANIFEST_DIR=/app
# Set the entrypoint to the compiled binary
ENTRYPOINT ["/usr/local/bin/calcr"]