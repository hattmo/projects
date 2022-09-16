# source blocks are analogous to the "builders" in json templates. They are used
# in build blocks. A build block runs provisioners and post-processors on a
# source. Read the documentation for source blocks here:
# https://www.packer.io/docs/templates/hcl_templates/blocks/source
source "vsphere-iso" "k3os" {
  CPUs      = 4
  RAM       = 8192
  boot_wait = "5s"

  boot_command = [
    "<esc><wait1><esc><wait1><esc><wait1><esc><wait1><esc><wait1>",
    "<down>e<wait1>",
    "<down><down><down><down><down><down><down><left> ",
    "k3os.install.silent=true k3os.install.device=/dev/sda k3os.install.config_url=http://{{ .HTTPIP }}:{{ .HTTPPort }}/config.yml",
    "<leftCtrlOn>x<leftCtrlOff>"
  ]

  guest_os_type       = "ubuntu64Guest"
  insecure_connection = true
  network_adapters {
    network_card = "vmxnet3"
    network      = "LAN"
  }
  ssh_username           = "rancher"
  ssh_private_key_file   = data.sshkey.install.private_key_path
  ssh_keypair_name       = ""
  ssh_handshake_attempts = "100"
  ssh_timeout            = "10m"

  iso_url      = "https://github.com/rancher/k3os/releases/download/v0.21.5-k3s2r1/k3os-amd64.iso"
  iso_checksum = "a465b0c52ce415173f6ef38fda5d090580fbaae0970556a62f21c7db8eeb72b1"

  storage {
    disk_size             = 15000
    disk_thin_provisioned = true
  }

  http_content = {
    "/config.yml" = templatefile("${path.root}/config.pkr.hcl", {
      "SSH_KEY" : data.sshkey.install.public_key
    })
  }


  vcenter_server      = "vcsa.hattmo.com"
  username            = ""
  password            = ""
  datacenter          = "Datacenter"
  vm_name             = "k3os"
  folder              = "tmp"
  host                = "esxi1.hattmo.com"
  datastore           = "datastore1"
  convert_to_template = false

}

build {
  name = "k3os_test"
  sources = [
    "source.vsphere-iso.k3os"
  ]
}


data "sshkey" "install" {
}

packer {
  required_plugins {
    vsphere = {
      version = ">= 0.0.1"
      source  = "github.com/hashicorp/vsphere"
    }
    sshkey = {
      version = ">= 1.0.1"
      source  = "github.com/ivoronin/sshkey"
    }
  }
}
