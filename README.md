# WIP 👷

![Screenshot](./myvtt.png)

## Build & Run

```bash
# Native
cargo run -r
# Web (requires basic-http-server `cargo install basic-http-server` )
cargo build -r --target wasm32-unknown-unknown && ~/.cargo/bin/basic-http-server .
```
