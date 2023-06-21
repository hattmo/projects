#!/bin/bash
cat /home/arch/fdisk_script | sfdisk /dev/sda
mkfs.ext4 /dev/sda3
mkswap /dev/sda2
mkfs.fat -F 32 /dev/sda1
mount /dev/sda3 /mnt
mount --mkdir /dev/sda1 /mnt/boot
swapoff /dev/sda2

pacstrap -K /mnt base linux linux-firmware netplan
genfstab -U /mnt >> /mnt/etc/fstab