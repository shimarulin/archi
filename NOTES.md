### Installation process

#### Questions

- initramfs: mkinitcpio, dracut, booster?

---

- https://ctlos.github.io/wiki/btrfs/btrfs-part1/

```shell
pacstrap /mnt base btrfs-progs grub linux linux-firmware nano
genfstab -U /mnt >> /mnt/etc/fstab
arch-chroot /mnt
hostnamectl set-hostname shimarulin-arch-vm
ln -sf /usr/share/zoneinfo/Europe/Moscow /etc/localtime
hwclock --systohc
#passwd
[root@archiso /]# touch /boot/grub/grub.cfg
[root@archiso /]# touch /boot/grub/config.cfg
[root@archiso /]# nano /boot/grub/grub.cfg
[root@archiso /]# nano /boot/grub/config.cfg
[root@archiso /]# nano /boot/grub/grub.cfg
[root@archiso /]# nano /boot/grub/config.cfg
[root@archiso /]# nano /etc/pacman.conf
```

```shell
#  GNU nano 5.6.1 /boot/grub/grub.cfg                                                                                                                                                                                          Modified
. $prefix/config.cfg
```

```shell
#  GNU nano 5.6.1 /boot/grub/config.cfg  
set timeout=5
menuentry "Arch Linux" {
  insmod btrfs
  linux /@/boot/vmlinuz-linux root=LABEL=System ro rootflags=subvol=@
  initrd /@/boot/initramfs-linux.img
}
```

Прямая блокировка grub.cfg
Чтобы защитить файл от любых изменений, присвойте ему атрибут immutable.
```shell
chattr +i /boot/grub/grub.cfg
```
Если основная конфигурация уже вынесена в другой файл, блокировку grub.cfg достаточно установить однажды и больше не снимать.

Блокировка защитит файл от перезаписи скриптами установки пакетов. Чтобы избежать конфликта с файлом из пакета, добавьте его имя в строку NoUpgrade в /etc/pacman.conf:

```shell
#NoUpgrade   =
NoUpgrade = boot/grub/grub.cfg
```

```shell
arch-chroot /mnt grub-mkconfig -o /boot/grub/grub.cfg
#    Generating grub configuration file ...
#    Found linux image: /boot/vmlinuz-linux
#    Found initrd image: /boot/initramfs-linux.img
#    Found fallback initrd image(s) in /boot: initramfs-linux-fallback.img
#    done
```

#### Installation articles

- [UEFI Boot exec with Dracut on Arch Linux](http://3.123.67.145/)

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
