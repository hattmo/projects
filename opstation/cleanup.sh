#!/bin/bash
for MOUNTPATH in $(mount | grep "/tmp/opstation" | cut -d " " -f 3); do
    echo unmounting $MOUNTPATH
    sudo umount $MOUNTPATH
done
for LOOP in $(losetup | grep /home/matthew | cut -d " " -f 1); do 
    echo detaching $LOOP
    sudo losetup -d $LOOP
done