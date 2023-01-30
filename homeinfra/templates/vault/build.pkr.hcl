variable "vsphere_un" {
  type = string
}

variable "vsphere_pw" {
  type = string
}

source "vsphere-clone" "vault" {
  vcenter_server      = "vcsa.hattmo.com"
  username            = var.vsphere_un
  password            = var.vsphere_pw
  datacenter          = "Datacenter"
  vm_name             = format("vault_%s", uuidv4())
  folder              = "templates"
  host                = "esxi1.hattmo.com"
  datastore           = "datastore1"
  insecure_connection = true

  template = "ubuntu_f1dc6a56-9a7d-4f35-800a-e9716a0270b1"
  convert_to_template = true

  shutdown_command = "sudo shutdown -h now"

  communicator         = "ssh"
  ssh_username         = "admin"
  ssh_certificate_file = "${path.root}/../../admin/admin-cert.pub"
  ssh_private_key_file = "${path.root}/../../admin/admin"

}

build {
  name = "vault"
  sources = [
    "source.vsphere-clone.vault"
  ]
  provisioner "shell" {
    script = "${path.root}/setup.sh"
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
