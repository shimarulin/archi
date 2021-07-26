# Archi

> Just another simple and minimal Arch Linux installer

### Principles

- Use the original Arch Linux installation image
- Use minimal but sufficient setup
- Use the most automated installation and configuration process

### Features

- UEFI and BIOS compatible disk layout and bootloader (used GRUB2): you can use the disk with the installed system
  in another computer with UEFI or BIOS without any changes in most use cases
- Detect user timezone automatically
- Btrfs subvolumes for root and home directories
- Add new user with `sudo` for granting administrator privileges to a user
- Login for `root` user is disabled by default
- Used NetworkManager for manage connections

### Installation and usage

You should boot from Arch Linux installation image, download installer binary, set executable permission, and run it:

```shell
# Download https://github.com/shimarulin/archi/releases/latest/download/archi
# with Curl
curl -L https://git.io/JGRV3 --output archi
# or Wget
wget https://git.io/JGRV3 -O archi
# Mark the file as executable
chmod +x archi
# Run
./archi
```

### Known issues

At this moment timezone select (from [dialoguer](https://github.com/mitsuhiko/dialoguer)) isn't very convenient to use. Press arrow up and down on keyboard to select detected timezone.

### Notes for installed system

If you move the disc after installation to another computer with UEFI, do not forget to generate a new UEFI menu item (`/dev/sda` - your disk with installed Arch Linux):

```shell
efibootmgr -c -d /dev/sda -p 2 -L "Arch Linux" -l "\EFI\BOOT\BOOTX64.EFI"
```

## Development

### Git hooks

Run in project root to set Git hooks directory:

```shell
git config core.hooksPath .hooks
```

### Development dependencies

To extend Cargo functionality I'm use https://github.com/killercup/cargo-edit. To compile it, you need to install the openssl development package.

For example, `libssl-dev` on Ubuntu:

```shell
sudo apt install libssl-dev && cargo install cargo-edit
```

or `openssl-devel` on Fedora.

Install development tools:

```shell
cargo install cargo-edit cocogitto cargo-bump
```

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

#### Use Docker to build with musl

See more on https://github.com/clux/muslrust

### How to test on Virtualbox

#### Setup SSH connection

1. Add default VirtualBox Network: VirtualBox > Settings > Network > Add (you will get vboxnet0)
2. Setup VM Network: VM's Settings > Network > Adapter 2 > Host Only (vboxnet0)
3. Run VM:
    2.1. Setup password: `passwd`
    2.2. Show IP: `ip addr show` (192.168.43.91, for example)
4. If the SSH service in the guest OS is not running, start it (`systemctl start sshd`)
5. Connect from host: `ssh -o 'IdentitiesOnly=yes' root@192.168.43.91`

#### Send file from host to guest via SSH

```shell
scp -o 'IdentitiesOnly=yes' ./archi root@192.168.43.91:/root/
```
