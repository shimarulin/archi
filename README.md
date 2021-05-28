# Archi

> Just another simple and minimal Arch Linux installer

## Principles

- Use the original Arch Linux installation image
- Use minimal but sufficient setup
- Use the most automated installation and configuration process

## Features

- UEFI and BIOS compatible disk layout and bootloader (used GRUB2)
- Btrfs subvolumes for root and home directories
- Add new user with `sudo` for granting administrator privileges to a user
- Login for `root` user is disabled by default
- Used NetworkManager for manage connections

## Installation notes

If you move the disc after installation to another computer with UEFI, do not forget to generate a new UEFI menu item (`/dev/sda` - your disk with installed Arch Linux):

```shell
efibootmgr -c -d /dev/sda -p 2 -L "Arch Linux" -l "\EFI\BOOT\BOOTX64.EFI"
```

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

### How to test on Virtualbox

#### Setup SSH connection

1. Setup VM Network. Select Bridged Adapter
2. Run VM:
    2.1. Setup password: `passwd`
    2.2. Run SSH service: `systemctl start sshd`
    2.2. Show IP: `ip addr show` (192.168.43.91, for example)
3. Connect from host: `ssh -o 'IdentitiesOnly=yes' root@192.168.43.91`

#### Send file from host to guest via SSH

```shell
scp -o 'IdentitiesOnly=yes' ./archi root@192.168.43.91:/root/
```
