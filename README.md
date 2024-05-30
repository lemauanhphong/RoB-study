# RoB

Implementation of RøB: Ransomware over Modern Web Browsers

# Build

Build the client:

```sh
cd ./client/
wasm-pack build -t web
```

Build the server:

```sh
cd ./server/
cargo build --release
```

# Usage

Run the server.

```bash
./server/target/release/server
```
