#!/bin/bash
set -euxo pipefail
#ntpupdate -vu time.nist.gov

echo -e "g\n" | fdisk /dev/sda
sfdisk /dev/sda < ./fdisk_script 

mkswap /dev/sda2
swapon /dev/sda2

mkfs.ext4 /dev/sda3
e2label /dev/sda3 "ROOT"
mount /dev/sda3 /mnt

mkfs.fat -F 32 /dev/sda1
mount --mkdir=0700 /dev/sda1 /mnt/boot
# chown root:root /mnt/boot
# chmod 700 /mnt/boot
# pacman-key --init
# pacman-key --populate archlinux
# pacman-key --refresh-keys

while true; do
    # Check synchronization status
    sync_status=$(timedatectl show -p NTPSynchronized --value)
    if [ "$sync_status" == "yes" ]; then
        echo "NTP is ready!"
        break
    fi

    # Wait for 5 seconds before checking again
    sleep 5
done



while true; do
    # Check synchronization status
    pacman_status=$(systemctl show pacman-init.service -p Result --value)
    if [ "$pacman_status" == "success" ]; then
        echo "pacman ready!"
        break
    else
        echo "waiting for keyrings"
    fi

    # Wait for 5 seconds before checking again
    sleep 5
done
sleep 30s

pacstrap -K /mnt base mkinitcpio mkinitcpio-systemd-tool linux linux-firmware intel-ucode
genfstab -U /mnt >> /mnt/etc/fstab
#awk -i inplace '/^[^#]/ && $3 ~ "ext[2-4]" { $4 = $4 ",x-systemd.growfs" } 1' OFS="\t" /etc/fstab

mkdir -p /mnt/boot/loader/entries
sudo chown root:root loader.conf arch.conf
mv ./loader.conf /mnt/boot/loader/loader.conf
mv ./arch.conf /mnt/boot/loader/entries/arch.conf

mv ./in_chroot.sh /mnt
arch-chroot /mnt ./in_chroot.sh
rm -f /mnt/in_chroot.sh

# mv ./mkinitcpio.conf /mnt/etc/mkinitcpio.conf.d/custom.conf
