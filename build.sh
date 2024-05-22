#!/bin/zsh

echo

cargo build --release

echo

rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/orz_os -O binary target/riscv64gc-unknown-none-elf/release/orz_os.bin

qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ./bin/rustsbi-qemu.bin \
    -device loader,file=./target/riscv64gc-unknown-none-elf/release/orz_os.bin,addr=0x80200000
