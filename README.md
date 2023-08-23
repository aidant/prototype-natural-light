```
rustup update
rustup install nightly
rustup target add thumbv7em-none-eabi

# https://probe.rs/docs/getting-started/installation/
cargo install probe-rs --features cli,ftdi
cargo +nightly run
```
