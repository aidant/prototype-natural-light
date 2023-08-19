```
rustup update
rustup target add thumbv7em-none-eabihf

# https://probe.rs/docs/getting-started/installation/
cargo install probe-rs --features cli,ftdi
cargo flash --release --chip STM32F411CEUx
```
