#!/usr/bin/env bash
set -e

# Default paths, but you can override by setting an env variable
OVMF_CODE_FILE="${OVMF_CODE_FILE:-/usr/share/OVMF/OVMF_CODE_4M.fd}"
OVMF_VARS_FILE="${OVMF_VARS_FILE:-/usr/share/OVMF/OVMF_VARS_4M.fd}"

# Copy OVMF files to current directory only if they don't exist
[ ! -f ./OVMF_CODE_4M.fd ] && cp "$OVMF_CODE_FILE" ./OVMF_CODE_4M.fd
[ ! -f ./OVMF_VARS_4M.fd ] && cp "$OVMF_VARS_FILE" ./OVMF_VARS_4M.fd

# Create UEFI boot path
mkdir -p esp/efi/boot

# Compile the UEFI example in release mode
cargo +nightly build -p uefi_example --target=x86_64-unknown-uefi --release

# Copy the compiled EFI file
cp target/x86_64-unknown-uefi/release/uefi_example.efi esp/efi/boot/bootx64.efi

# Start QEMU
qemu-system-x86_64 \
    -enable-kvm \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE_4M.fd \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS_4M.fd \
    -drive format=raw,file=fat:rw:esp
