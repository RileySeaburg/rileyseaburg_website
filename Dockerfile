# Use a newer base image for building
FROM debian:bullseye-slim as builder

# Install Rust
RUN apt-get update && \
    apt-get install -y curl && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Rust to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Set the current working directory inside the container
WORKDIR /usr/src/rileyseaburg_website

# Copy the current directory contents into the container
COPY . .

# Install required dependencies including libpq, OpenSSL, lld, and clang
RUN apt-get install -y libpq-dev openssl pkg-config lld clang && \
    rm -rf /var/lib/apt/lists/*

# Build the Rust project with verbose output
RUN cargo build --release -v

# Use the same newer base image for the final build
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libpq5 openssl && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage to the final image
COPY --from=builder /usr/src/rileyseaburg_website/target/release/rileyseaburg_website /usr/local/bin/

# Set the command to run your application
CMD ["rileyseaburg_website"]
