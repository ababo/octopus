# Octopus Operating System


## How to build

### Prerequisites

1. Set `TARGET_ARCH` environment variable for the target architecture (can be
`x86_64` or `aarch64`).

2. Install GNU binary utilities:
    - Download the latest [source code](http://ftp.gnu.org/gnu/binutils/).
    - Run `./configure --target $TARGET_ARCH-elf`.
    - Run `make && sudo make install`.

3. Install [Rust nightly](https://doc.rust-lang.org/1.5.0/book/nightly-rust.html).

4. Run `cargo install cargo-xbuild`.

### Build

1. Cd to the project directory.

2. Run `cargo xbuild --target $TARGET_ARCH-elf.json`.

## Run

The build produces multiboot-compatible kernel image which can be booted by GRUB.

You can also use QEMU emulator to test it:

```sh
qemu-system-$TARGET_ARCH -nographic -kernel target/$TARGET_ARCH-elf/release/octopus
```

## Blog

See my development [blog](https://octopus-os.blogspot.com/).
