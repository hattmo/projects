#!/bin/bash
echo -e "g\n" | fdisk /dev/sda
cat ./fdisk_script | sfdisk /dev/sda
mkfs.ext4 /dev/sda3
mkswap /dev/sda2
mkfs.fat -F 32 /dev/sda1
mount /dev/sda3 /mnt
mount --mkdir /dev/sda1 /mnt/boot
swapon /dev/sda2

pacstrap -K /mnt base linux linux-firmware git neovim openssh cloud-init intel-ucode
genfstab -U /mnt >> /mnt/etc/fstab

mv ./in_chroot.sh /mnt
mkdir -p /mnt/boot/loader/entries
mv ./loader.conf /mnt/boot/loader/loader.conf
mv ./arch.conf /mnt/boot/loader/entries/arch.conf
chmod +x /mnt/in_chroot.sh
arch-chroot /mnt ./in_chroot.sh
rm -f /mnt/in_chroot.sh