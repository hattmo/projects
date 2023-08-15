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

  network_adapters {
    network_card = "vmxnet3"
    network      = "LAN"
  }

  storage {
    disk_size             = 65536
    disk_thin_provisioned = true
  }
  guest_os_type = "other4xLinuxGuest"

  iso_url      = "https://geo.mirror.pkgbuild.com/iso/2023.09.01/archlinux-2023.09.01-x86_64.iso"
  iso_checksum = "0d71c9bc56af75c07e89cd41eaa5570ac914677ad6bc8e84935dc720ce58f960"

  boot_wait = "1s"
  boot_command = [
    "e",
    "<tab><wait2>",
    " ds=nocloud",
    "<enter>"
  ]
  shutdown_command = "sudo shutdown -h now"

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
    sources      = ["provision.sh", "fdisk_script", "in_chroot.sh", "loader.conf","arch.conf"]
    destination = "/home/arch/"
  }
  provisioner "shell" {
    inline = ["chmod +x ~/provision.sh", "sudo ~/provision.sh"]
  }
  provisioner "breakpoint" {
    disable = false
  }
}

packer {
  required_plugins {
    vsphere = {
      version = ">= 0.0.1"
      source  = "github.com/hashicorp/vsphere"
    }
  }
}
