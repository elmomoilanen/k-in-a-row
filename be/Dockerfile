FROM rust:1.78 AS build

# Create a new empty project
RUN USER=root cargo new --bin be
WORKDIR /be
# Build the dependencies only (main.rs is empty placeholder)
COPY ./Cargo.lock .
COPY ./Cargo.toml .
COPY ./src/main.rs ./benches/bench.rs
COPY ./src/main.rs ./src/bin/server/
RUN cargo build --bin server --release \
    && rm -r src/bin
# Copy source code and install the server binary
COPY ./src ./src
RUN rm ./target/release/deps/server*
RUN cargo install --bin server --path .

FROM gcr.io/distroless/cc-debian12

COPY --from=build /be/target/release/server .
CMD ["./server"]
