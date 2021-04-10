# Archi

> Just another experimental Arch installer

## Development

### Development dependencies

To extend Cargo functionality I'm use https://github.com/killercup/cargo-edit. To compile it, you need to install the openssl development package.

For example, `libssl-dev` on Ubuntu:

```shell
sudo apt install libssl-dev && cargo install cargo-edit
```

or `openssl-devel` on Fedora.

### Build

#### Optimizing the size of the executable file

- https://lifthrasiir.github.io/rustlog/why-is-a-rust-executable-large.html
- https://github.com/johnthagen/min-sized-rust

Cargo.toml:
```toml
# Release optimization
[profile.release]
opt-level = 'z'  # Optimize for size.
lto = true
panic = 'abort'
```

Shell command:
```shell
cargo build --release && strip target/release/archi
```