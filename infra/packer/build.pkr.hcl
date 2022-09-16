# source blocks are analogous to the "builders" in json templates. They are used
# in build blocks. A build block runs provisioners and post-processors on a
# source. Read the documentation for source blocks here:
# https://www.packer.io/docs/templates/hcl_templates/blocks/source
source "vsphere-iso" "base" {
  CPUs            = 4
  RAM          = 2048
  boot_wait = "5s"
  boot_command = [
        "<enter><enter><f6><esc><wait> ",
        "autoinstall",
        "<enter>"
  ]

  disk_controller_type = ["pvscsi"]
  guest_os_type        = "ubuntu64Guest"
  insecure_connection  = true
  network_adapters {
    network_card = "vmxnet3"
    network = var.vsphere_network
  }
  ssh_password = "ubuntu"
  ssh_username = "ubuntu"
  ssh_handshake_attempts = "50"
  ssh_timeout = "6m"
  storage {
    disk_size             = 32768
    disk_thin_provisioned = true
  }

  iso_url = "https://releases.ubuntu.com/20.04.4/ubuntu-20.04.4-live-server-amd64.iso"
  iso_checksum = "none"

  cd_files = ["./data/user-data","./data/meta-data"]
  cd_label = "cidata"

  vcenter_server = var.vsphere_server
  username       = var.vsphere_username
  password       = var.vsphere_password
  datacenter     = var.vsphere_datacenter
  datastore      = var.vsphere_datastore
  vm_name        = var.vm_name
  folder         = var.vsphere_folder
  host           = var.vsphere_host
  convert_to_template = true

}

# a build block invokes sources and runs provisioning steps on them. The
# documentation for build blocks can be found here:
# https://www.packer.io/docs/templates/hcl_templates/blocks/build
build {
  sources = ["source.vsphere-iso.base"]

  provisioner "shell" {
    inline = ["ls /"]
  }
}

variable "vsphere_server" {
    type = string
}
variable "vsphere_username" {
    type = string
}
variable "vsphere_password"{
    type = string
}
variable "vsphere_datacenter"{
    type = string
}

variable "vsphere_datastore" {
  type = string
}
variable "vsphere_folder"{
    type = string
}
variable "vsphere_host"{
    type = string
}
variable "vsphere_network" {
    type = string
}
variable "vm_name"{
    type = string
}

packer {
  required_plugins {
    vsphere = {
      version = ">= 0.0.1"
      source = "github.com/hashicorp/vsphere"
    }
  }
}
