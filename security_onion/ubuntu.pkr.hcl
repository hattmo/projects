source "vsphere-iso" "ubuntu" {
  CPUs      = 4
  RAM       = 8192
  boot_wait = "5s"

  boot_command = [
    "<enter><enter><f6><esc><wait>",
    "autoinstall ds=nocloud-net;s=http://172.30.1.3:{{.HTTPPort}}/",
    "<enter><wait>"
  ]

  guest_os_type       = "ubuntu64Guest"
  insecure_connection = true
  network_adapters {
    network_card = "vmxnet3"
    network      = "LAN"
  }
  ssh_username = "matthew"
  ssh_password = "ubuntu"
  ssh_timeout = "1h"

  iso_url      = "https://releases.ubuntu.com/20.04/ubuntu-20.04.4-live-server-amd64.iso"
  iso_checksum = "28ccdb56450e643bad03bb7bcf7507ce3d8d90e8bf09e38f6bd9ac298a98eaad"

  storage {
    disk_size             = 131072
    disk_thin_provisioned = true
  }

  http_content = {
    "/user-data" = templatefile("${path.root}/conf/user-data.pkr.hcl", {})
    "/meta-data" = templatefile("${path.root}/conf/meta-data.pkr.hcl", {})
  }

  vcenter_server      = ""
  username            = ""
  password            = ""
  datacenter          = ""
  vm_name             = ""
  folder              = ""
  host                = ""
  datastore           = ""

}

build {
  name = "ubuntu"
  sources = [
    "source.vsphere-iso.ubuntu"
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
