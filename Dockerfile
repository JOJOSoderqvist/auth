# Stage 1: Builder
FROM rust:latest AS builder

WORKDIR /app

# Copy your Rust project files
COPY . .

# Build your Rust application (adjust as needed for your specific project)
RUN cargo build --release

# Stage 2: Runtime
FROM debian:stable-slim

WORKDIR /app

# Copy the compiled executable from the builder stage
COPY --from=builder /app/target/release/auth .

EXPOSE 3000

# Set the entrypoint for your application
CMD ["./auth"]