# Use a Rust base image
FROM rust:latest as builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.lock Cargo.toml ./

# Copy the source code
COPY . .

# Build the project
RUN cargo build  --target x86_64-unknown-linux-musl

# Create the final Docker image
FROM debian:buster-slim

# Install PostgreSQL client
RUN apt-get update && apt-get install -y postgresql-client

# Set the working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/debug/gomarket-items .

# Set the environment variables
ENV DATABASE_URL=postgresql://postgres:1234@127.0.0.1:5432/postgres

# Expose any necessary ports (e.g., for web applications)
EXPOSE 8000

# Run the binary
CMD ["./gomarket-items"]