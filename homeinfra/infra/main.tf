variable "vsphere_un" {
  type = string
}

variable "vsphere_pw" {
  type      = string
  sensitive = true
}

provider "vsphere" {
  user                 = var.vsphere_un
  password             = var.vsphere_pw
  vsphere_server       = "vcsa.hattmo.com"
  allow_unverified_ssl = true
}

data "vsphere_datacenter" "datacenter" {
  name = "Datacenter"
}

data "vsphere_datastore" "datastore1" {
  name          = "datastore1"
  datacenter_id = data.vsphere_datacenter.datacenter.id
}

data "vsphere_datastore" "datastore2" {
  name          = "datastore2"
  datacenter_id = data.vsphere_datacenter.datacenter.id
}

data "vsphere_compute_cluster" "cluster" {
  name          = "MyCluster"
  datacenter_id = data.vsphere_datacenter.datacenter.id
}

data "vsphere_host" "esxi1" {
  name          = "esxi1.hattmo.com"
  datacenter_id = data.vsphere_datacenter.datacenter.id
}

data "vsphere_network" "lan" {
  name          = "LAN"
  datacenter_id = data.vsphere_datacenter.datacenter.id
}

data "vsphere_network" "wan" {
  name          = "WAN"
  datacenter_id = data.vsphere_datacenter.datacenter.id
}

resource "vsphere_folder" "infra" {
  datacenter_id = data.vsphere_datacenter.datacenter.id
  path          = "infra"
  type          = "vm"
}
