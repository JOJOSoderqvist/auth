# Stage 1: Builder
FROM rust:latest AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y protobuf-compiler

# Copy your Rust project files
COPY . .

# Build your Rust application (adjust as needed for your specific project)
RUN cargo build --release

# Stage 2: Runtime
FROM debian:stable-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled executable from the builder stage
COPY --from=builder /app/target/release/auth .

EXPOSE 8010
EXPOSE 8011

# Set the entrypoint for your application
CMD ["./auth"]