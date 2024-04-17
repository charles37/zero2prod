FROM nixos/nix:latest

# Copy the flake files
COPY flake.nix flake.lock ./

# Build the Rust binary
RUN nix build --experimental-features 'nix-command flakes' 

# Create a new stage with a minimal base image
FROM debian:buster-slim

# Copy the Rust binary from the Nix store
COPY --from=0 /nix/store/*-zero2prod-0.1.0/bin/zero2prod /app/zero2prod

# Set the working directory
WORKDIR /app

# Set the entry point to the Rust binary
ENTRYPOINT ["/app/zero2prod"]
