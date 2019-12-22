# Octopus Operating System


## How to build

### Prerequisites

1. Set `TARGET_ARCH` environment variable for the target architecture (can be
`x86_64` or `aarch64`).

2. Install GNU binary utilities:
    - Download the latest [source code](http://ftp.gnu.org/gnu/binutils/).
    - Run `./configure --target $TARGET_ARCH-elf`.
    - Run `make && sudo make install`.

3. Install Rust:

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain=nightly
    ```

4. Run `rustup component add rust-src`.

5. Run `cargo install cargo-xbuild`.

### Build

1. Cd to the project directory.

2. Run `cargo xbuild --target src/arch/$TARGET_ARCH/$TARGET_ARCH-elf.json --release`.

## Run

To boot the kernel image on QEMU emulator run: `src/arch/$TARGET_ARCH/run-qemu.sh`.

**Note**: For `x86_64` the kernel image complies with the multiboot specification and can be booted by GRUB. Kernel output will be passed through COM1-port.

## Blog

See my development [blog](https://octopus-os.blogspot.com/).
