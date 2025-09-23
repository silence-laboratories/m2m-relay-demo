# Build stage
FROM rust:1.90.0 as builder

WORKDIR /app

# Copy the Cargo files
COPY Cargo.toml Cargo.lock ./
COPY m2m-relay-svc/Cargo.toml ./m2m-relay-svc/

# Create a dummy main.rs to build dependencies
RUN mkdir -p m2m-relay-svc/src && \
    echo "fn main() {}" > m2m-relay-svc/src/main.rs

# Build dependencies only (to cache them)
RUN cargo build --release -p m2m-relay-svc

# Copy the actual source code
COPY m2m-relay-svc/src ./m2m-relay-svc/src

# Build the actual application
RUN touch m2m-relay-svc/src/main.rs && \
    cargo build --release -p m2m-relay-svc

# Runtime stage
FROM debian:bookworm-slim

# Install ca-certificates for HTTPS requests
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a user for running the application
RUN useradd -ms /bin/bash trio

WORKDIR /app

# Copy the binary from the build stage
COPY --from=builder /app/target/release/m2m-relay-svc ./m2m-relay-svc

# Change ownership of the binary to the trio user
RUN chown trio:trio ./m2m-relay-svc

# Switch to the trio user
USER trio

# Expose the default port
EXPOSE 8080

# Run the application
ENTRYPOINT ["./m2m-relay-svc"]
CMD ["--listen", "0.0.0.0:8080"]