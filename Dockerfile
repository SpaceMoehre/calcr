# Stage 1: Build the Rust binary
FROM rust:1.82.0 as builder

# Set the working directory inside the container
WORKDIR /app

# Copy the project files into the container
COPY . .

# Build the release version of the binary
RUN cargo build --release

# Stage 2: Create a minimal image with the binary
FROM debian:buster-slim

# Set the working directory inside the container
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/calcr /usr/local/bin/calcr

# Set the entrypoint to the compiled binary
ENTRYPOINT ["/usr/local/bin/calcr"]