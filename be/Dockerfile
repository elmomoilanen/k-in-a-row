FROM rust:1.66 as build

# Create a new empty project
RUN USER=root cargo new --bin be
WORKDIR /be
# Build the dependencies only
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src/main.rs ./src/bin/server/
RUN cargo build --release \
    && rm -r src/bin
# Copy source code and run build
COPY ./src ./src
RUN rm ./target/release/deps/server*
RUN cargo install --path .

FROM gcr.io/distroless/cc-debian11

COPY --from=build /be/target/release/server .
CMD ["./server"]
