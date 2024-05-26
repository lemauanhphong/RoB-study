FROM rust:latest

RUN cargo install wasm-pack

WORKDIR /app
COPY ./server ./server
COPY ./client ./client

WORKDIR /app/client
RUN wasm-pack build -t web

WORKDIR /app/server
RUN cargo build --release

CMD ./target/release/server
