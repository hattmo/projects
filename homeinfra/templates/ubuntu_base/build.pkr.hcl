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


  iso_url = "https://releases.ubuntu.com/focal/ubuntu-20.04.5-live-server-amd64.iso"
  iso_checksum = "5035be37a7e9abbdc09f0d257f3e33416c1a0fb322ba860d42d74aa75c3468d4"

  floppy_content = {
    "user-data" = templatefile("${path.root}/user-data.pkrtpl.hcl", {
      ca_cert_pub = file("${path.root}/../../admin/ca.pub")
    })
    "meta-data" = templatefile("${path.root}/meta-data.pkrtpl.hcl", {
    })
  }
  floppy_label = "cidata"

  boot_wait = "1s"
  boot_command = [
    "<bs><wait><bs><wait><bs><wait>",
    "<esc><f6><esc>",
    "ipv6.disable=1 net.ifnames=0 autoinstall ds=nocloud",
    "<enter>"
  ]
  shutdown_command = "sudo shutdown -h now"

  communicator         = "ssh"
  ssh_username         = "admin"
  ssh_certificate_file = "${path.root}/../../admin/admin-cert.pub"
  ssh_private_key_file = "${path.root}/../../admin/admin"

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
