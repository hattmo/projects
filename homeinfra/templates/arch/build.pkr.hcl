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
  cpu_cores = 1
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

  iso_url      = "https://geo.mirror.pkgbuild.com/iso/2023.03.01/archlinux-2023.03.01-x86_64.iso"
  iso_checksum = "816758ba8fd8a06dff539b9af08eb8100c8bad526ac19ef4486bce99cd3ca18c"

  boot_wait = "1s"
  boot_command = [
    "<tab><wait2>",
    " ds=nocloud",
    "<enter>"
  ]
  shutdown_command = "sudo shutdown -h now"

  floppy_content = {
    "user-data" = "#cloud-config\n\npassword: password\nchpasswd: { expire: False }\nssh_pwauth: True\n",
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
    source      = "fdisk_script"
    destination = "~/fdisk_script"
  }
  provisioner "file" {
    source      = "provision.sh"
    destination = "~/provision.sh"
  }
  provisioner "shell" {
    inline = ["chmod +x ~/provision.sh", "sudo ~/provision.sh"]
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
