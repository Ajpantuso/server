# SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
#
# SPDX-License-Identifier: Apache-2.0

FROM rust:1.74 as builder

# Initialize cargo project
RUN USER=root cargo new --bin server
WORKDIR /server

# Add std library implementation for x86_64-unknown-linux-musl
RUN rustup target add x86_64-unknown-linux-musl

# Download and compile dependencies
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-musl
RUN rm src/*.rs

# Compile main binary
COPY ./src ./src
COPY ./assets ./assets
RUN rm ./target/x86_64-unknown-linux-musl/release/deps/server*
RUN RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

WORKDIR /app

COPY --from=builder /server/target/x86_64-unknown-linux-musl/release/server .

USER 1001:1001

ENTRYPOINT ["/app/server", "serve"]
CMD ["--listen-addr", "0.0.0.0:8080"]
