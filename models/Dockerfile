# Use the official Rust image as the base image
FROM rust:alpine

RUN rustup default nightly
ENV CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG=true
RUN mkdir ./target
ENV CARGO_TARGET_DIR=./target

ARG TOML_LOCATION

# Install necessary build dependencies
RUN apk update && apk add --no-cache build-base musl-dev
RUN apk add protobuf
RUN apk add protoc

# Set the working directory
WORKDIR /usr/models

# Set the correct permissions
RUN mkdir -p /tmp
RUN chmod -R 777 /tmp

# Set the correct permissions for the working directory
RUN chown -R 1:1 /usr/models  # Use specific user and group ID
RUN chmod -R 777 /usr/models

# Copy the Cargo.toml file
#COPY ${TOML_LOCATION} /usr/models/Cargo.toml

# Build the Rust project
RUN #cargo build

CMD ["sh"]