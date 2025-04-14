variable "vsphere_un" {
  type = string
}

variable "vsphere_pw" {
  type = string
}

source "vsphere-iso" "arch-base" {
  vcenter_server      = "vcsa.hattmo.com"
  username            = var.vsphere_un
  password            = var.vsphere_pw
  datacenter          = "Datacenter"
  vm_name             = format("arch_%s", uuidv4())
  folder              = "templates"
  host                = "esxi1.hattmo.com"
  datastore           = "datastore1"
  insecure_connection = true

  convert_to_template = false

  CPUs      = 4
  cpu_cores = 4
  RAM       = 32768

  firmware  = "efi"

  network_adapters {
    network_card = "vmxnet3"
    network      = "Trusted LAN"
  }

  storage {
    disk_size             = 32768
    disk_thin_provisioned = true
  }
  guest_os_type = "other4xLinuxGuest"

  iso_url      = "https://mirror.rackspace.com/archlinux/iso/latest/archlinux-x86_64.iso"
  iso_checksum = "file:https://mirror.rackspace.com/archlinux/iso/latest/sha256sums.txt"

  boot_wait = "1s"
  boot_command = [
    "e",
    "<end><wait2>",
    " ds=nocloud",
    "<enter>"
  ]
  shutdown_command = "sudo poweroff"

  floppy_content = {
    "user-data" = file("./user-data.pktpl.hcl"),
    "meta-data" = ""
  }
  floppy_label = "cidata"


  communicator = "ssh"
  ssh_username = "arch"
  ssh_password = "password"
  ssh_timeout  = "1h"
}

build {
  name = "arch-base"
  sources = [
    "source.vsphere-iso.arch-base"
  ]

  provisioner "file" {
    sources      = ["provision.sh", "fdisk_script", "in_chroot.sh", "loader.conf","arch.conf", "mkinitcpio.conf"]
    destination = "/home/arch/"
  }
  
  provisioner "breakpoint" {
    disable = true
  }

  provisioner "shell" {
    inline = ["chmod +x ~/provision.sh", "sudo ~/provision.sh"]
  }
}

packer {
  required_plugins {
    vsphere = {
      version = "~> 1"
      source  = "github.com/hashicorp/vsphere"
    }
  }
}
