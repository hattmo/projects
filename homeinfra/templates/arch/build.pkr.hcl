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

  convert_to_template = true

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


  iso_url      = "https://geo.mirror.pkgbuild.com/iso/2023.03.01/archlinux-2023.03.01-x86_64.iso"
  iso_checksum = "816758ba8fd8a06dff539b9af08eb8100c8bad526ac19ef4486bce99cd3ca18c"

  # floppy_content = {
  #   "user-data" = templatefile("${path.root}/user-data.pkrtpl.hcl", {
  #     ca_cert_pub = file("${path.root}/../../admin/ca.pub")
  #   })
  #   "meta-data" = templatefile("${path.root}/meta-data.pkrtpl.hcl", {
  #   })
  # }
  # floppy_label = "cidata"

  # boot_wait = "1s"
  # boot_command = [
  #   "<bs><wait><bs><wait><bs><wait>",
  #   "<esc><f6><esc>",
  #   "ipv6.disable=1 net.ifnames=0 autoinstall ds=nocloud",
  #   "<enter>"
  # ]
  shutdown_command = "sudo shutdown -h now"

  communicator         = "ssh"
  ssh_username         = "admin"
  ssh_certificate_file = "${path.root}/../../admin/admin-cert.pub"
  ssh_private_key_file = "${path.root}/../../admin/admin"
  ssh_timeout = "1h"
}

build {
  name = "arch-base"
  sources = [
    "source.vsphere-iso.arch-base"
  ]
}

packer {
  required_plugins {
    vsphere = {
      version = ">= 0.0.1"
      source  = "github.com/hashicorp/vsphere"
    }
  }
}
