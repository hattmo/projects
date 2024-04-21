#!/bin/bash

qemu-nbd -c /dev/nbd0 -f qcow2 disk.img
partprobe /dev/nbd0
sleep 1
mount /dev/nbd0p1 ./disk
cp ./target/x86_64-unknown-uefi/debug/uefi_test.efi ./disk/EFI/BOOT/BOOTx64.EFI
umount ./disk
qemu-nbd -d /dev/nbd0 > /dev/null
