#!/bin/bash
set -euxo pipefail
ln -sf /usr/share/zoneinfo/US/Eastern /etc/localtime
hwclock --systohc
locale-gen
echo "LANG=en_US.UTF-8" > /etc/locale.conf
echo "arch" > /etc/hostname
bootctl install
systemctl enable systemd-boot-update
touch /etc/vconsole.conf
# systemctl enable sshd
awk '$2 == "/" { $4 = $4 (index($4, "x-systemd.growfs") ? "" : ",x-systemd.growfs") } 1' /etc/fstab > /tmp/fstab
mv /tmp/fstab /etc/fstab

mkdir -p /etc/repart.d/
echo -e "[Partition]\nType=root" > /etc/repart.d/root.conf

echo -e "password\npassword" | passwd
