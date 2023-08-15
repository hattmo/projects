#!/bin/bash
ln -sf /usr/share/zoneinfo/US/Eastern /etc/localtime
hwclock --systohc
locale-gen
echo "LANG=en_US.UTF-8" > /etc/locale.conf
echo "arch" > /etc/hostname
bootctl install
systemctl enable systemd-boot-update
systemctl enable sshd
echo -e "password\npassword" | passwd