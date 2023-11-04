# Use an official Rust runtime as a parent image
FROM rust:latest as builder

# Set the current working directory inside the container
WORKDIR /usr/src/rileyseaburg_website

# Copy the current directory contents into the container
COPY . .

# Install required dependencies including libpq, OpenSSL, lld, and clang
RUN apt-get update && \
    apt-get install -y libpq-dev openssl pkg-config lld clang && \
    rm -rf /var/lib/apt/lists/*

# Create a dummy main.rs to build dependencies and cache them
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Build the Rust project
RUN cargo build --release

# Use a newer base image for the final build
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libpq5 openssl && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage to the final image
COPY --from=builder /usr/src/rileyseaburg_website/target/release/rileyseaburg_website /usr/local/bin/

# Set the command to run your application
CMD ["rileyseaburg_website"]
