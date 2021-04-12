### Lib

- https://github.com/crossterm-rs/crossterm

### available shell commands and CLI tools

#### Disk

- parted
- fdisk
- lsblk (get info about disks)

### Tools

- Lint - https://github.com/rust-lang/rust-clippy
- Format code - https://github.com/rust-lang/rustfmt

Manage dependencies

- https://github.com/killercup/cargo-edit
    -  Could not find directory of OpenSSL installation, and this `-sys` crate cannot
        proceed without this knowledge. If OpenSSL is installed and this crate had
        trouble finding it,  you can set the `OPENSSL_DIR` environment variable for the
        compilation process.
           
        Make sure you also have the development packages of openssl installed.
        For example, `libssl-dev` on Ubuntu or `openssl-devel` on Fedora.
        
        If you're in a situation where you think the directory *should* be found
        automatically, please open a bug at https://github.com/sfackler/rust-openssl
        and include information about your system as well as this message.

### Pre-commit

- https://github.com/rhysd/cargo-husky
- https://github.com/doublify/pre-commit-rust
- https://github.com/swellaby/rusty-hook
- https://github.com/SirWindfield/xtools
- https://github.com/paulollivier/git-hooks

See also

- https://pre-commit.com/
- https://www.reddit.com/r/rust/comments/acdcbf/best_practice_for_cargo_fmt_commit_hooks_tests_etc/
- https://rustc-dev-guide.rust-lang.org/building/suggested.html

###  conventional commit and semver specifications

- https://github.com/oknozor/cocogitto
