# -------- Stage 1: Build --------
FROM rust:1.77 as builder

WORKDIR /usr/src/app

# Cache dependencies first
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -r src

# Copy the actual source code
COPY . .

# Build your project
RUN cargo build --release

# Optional: strip debug symbols for smaller size
RUN strip target/release/my_rust_api

# -------- Stage 2: Runtime --------
FROM debian:buster-slim

# Install minimal system dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/my_rust_api .

# Ensure the binary is executable
RUN chmod +x ./my_rust_api

# Expose the port your app listens on
EXPOSE 8080

# Launch the app
CMD ["./my_rust_api"]
