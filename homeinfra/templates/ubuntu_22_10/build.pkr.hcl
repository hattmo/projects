variable "vsphere_un" {
  type = string
}

variable "vsphere_pw" {
  type = string
}

source "vsphere-iso" "ubuntu-base" {
  vcenter_server      = "vcsa.hattmo.com"
  username            = var.vsphere_un
  password            = var.vsphere_pw
  datacenter          = "Datacenter"
  vm_name             = format("ubuntu_%s", uuidv4())
  folder              = "templates"
  host                = "esxi1.hattmo.com"
  datastore           = "datastore1"
  insecure_connection = true

  guest_os_type       = "ubuntu64Guest"
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


  iso_url      = "https://mirrors.iu13.net/ubuntu-releases/22.10/ubuntu-22.10-live-server-amd64.iso"
  iso_checksum = "874452797430a94ca240c95d8503035aa145bd03ef7d84f9b23b78f3c5099aed"

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

  configuration_parameters = {
    "guestinfo.userdata.encoding" = "base64"
    "guestinfo.userdata" = base64encode(templatefile("${path.root}/user-data.pkrtpl.hcl", {
      ca_cert_pub = file("${path.root}/../../admin/ca.pub")
    }))
    "guestinfo.metadata.encoding" = "base64"
    "guestinfo.metadata" = base64encode(templatefile("${path.root}/meta-data.pkrtpl.hcl", {
    }))
    "guestinfo.vendordata.encoding" = "base64"
    "guestinfo.vendordata" = base64encode(templatefile("${path.root}/vendor-data.pkrtpl.hcl", {
    }))
  }

  shutdown_command = "sudo shutdown -h now"

  communicator         = "ssh"
  ssh_username         = "admin"
  ssh_certificate_file = "${path.root}/../../admin/admin-cert.pub"
  ssh_private_key_file = "${path.root}/../../admin/admin"
  ssh_timeout = "1h"
  
}

build {
  name = "ubuntu-base"
  sources = [
    "source.vsphere-iso.ubuntu-base"
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
