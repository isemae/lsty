# Base image
FROM rust:1.76.0 AS builder

# Set working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Create an empty src directory to cache dependencies
RUN mkdir src && echo "fn main() {println!(\"Dummy file\")}" > src/main.rs

# Build the dependencies
RUN cargo build --release
# Remove dummy file
RUN rm -f src/main.rs

# Copy the source code to the container
COPY . .

# Create a new stage for the final image
FROM debian:buster-slim

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/ .

# Expose any ports the app needs
EXPOSE 8000

# Command to run the executable
CMD ["./lsty"]
