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

# Create an .env file with a SECRET_KEY of at least 100 characters
RUN echo "SECRET_KEY=$(openssl rand -base64 100 | tr -d '\n' | tr -d ' ')" > .env

# Copy the current directory contents into the container
COPY . .

# Install required dependencies including libpq, OpenSSL, lld, and clang
RUN apt-get install -y libpq-dev openssl libssl-dev pkg-config lld clang && \
    rm -rf /var/lib/apt/lists/*

# Build the Rust project with verbose output
RUN cargo build --release -v

# Use the same newer base image for the final build
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libpq5 openssl && \
    rm -rf /var/lib/apt/lists/*

# Set the working directory for the second stage of the build
WORKDIR /app

# Copy the binary from the builder stage to the final image
COPY --from=builder /usr/src/rileyseaburg_website/target/release/rileyseaburg_website /app/rileyseaburg_website

# Copy the .env file from the builder stage to the final image
COPY --from=builder /usr/src/rileyseaburg_website/.env /app/.env
COPY --from=builder /usr/src/rileyseaburg_website/rustyroad.toml  /app/rustyroad.toml
COPY --from=builder /usr/src/rileyseaburg_website/static /app/static
COPY --from=builder /usr/src/rileyseaburg_website/templates /app/templates

# Set the command to run your application
CMD ["./rileyseaburg_website"]
