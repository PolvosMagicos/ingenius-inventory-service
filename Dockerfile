# Use Rust official image with musl support
FROM rust:1.84.1-alpine as builder

# Install required dependencies for building
RUN apk add --no-cache musl-dev pkgconf perl make

# Set working directory inside the container
WORKDIR /app

# Copy Cargo files for dependency caching
COPY Cargo.toml Cargo.lock ./
COPY entity/Cargo.toml entity/

# Create dummy main.rs for dependency caching
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    mkdir -p entity/src && \
    echo "pub fn dummy() {}" > entity/src/lib.rs

# Build dependencies
RUN cargo build --release --target x86_64-unknown-linux-musl

# Remove the dummy files
RUN rm -f src/main.rs entity/src/lib.rs

# Copy the actual source code
COPY . .

# Verify files present
RUN ls -la /app

# Build the application
RUN cargo build --release --target x86_64-unknown-linux-musl && \
    ls -la target/x86_64-unknown-linux-musl/release/ingenius-inventory-service

# ---- Final Stage ----
FROM alpine:latest

# Install necessary runtime dependencies and debugging tools
RUN apk add --no-cache \
    libgcc \
    ca-certificates \
    postgresql-client \
    strace \
    bash \
    iputils \
    bind-tools \
    curl

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/ingenius-inventory-service /app/ingenius-inventory-service

# Not necesary env files
# COPY --from=builder /app/.env* ./ 

# Make sure the binary is executable
RUN chmod +x /app/ingenius-inventory-service

# Expose the application port
EXPOSE 8080

# Set default environment variables
ENV RUST_BACKTRACE=1
ENV RUST_LOG=debug

# Modified entrypoint for debugging
COPY entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

CMD ["/app/entrypoint.sh"]
