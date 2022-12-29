data "vsphere_datacenter" "dc" {
  name = var.vsphere_datacenter
}

data "vsphere_host" "host" {
  name          = var.vsphere_host
  datacenter_id = data.vsphere_datacenter.dc.id
}

data "vsphere_network" "WAN" {
  name = var.wan_network
}

resource "vsphere_host_virtual_switch" "switch" {
  name           = "PwnieVS"
  host_system_id = data.vsphere_host.host.id

  network_adapters = []

  active_nics  = []
  standby_nics = []
}

resource "vsphere_host_port_group" "LAN" {
  name                = "PwniePG"
  host_system_id      = data.vsphere_host.host.id
  virtual_switch_name = vsphere_host_virtual_switch.switch.name
}

module "Gateway" {
  source = "./gateway"
}


locals {
  Servers = {
    gitlab = {
      name          = "gitlab"
      data_folder   = "data"
      playbook_file = "gilab/gitlab.yml"
      ip            = var.gitlab_ip
      hash          = var.gitlab_files_hash
    }
  }
}

module "Server" {
  depends_on = [
    module.Gateway
  ]
  for_each         = local.Servers
  source           = "./base_machine"
  vsphere_server   = var.vsphere_server
  vsphere_username = var.vsphere_username
  vsphere_password = var.vsphere_password
  datacenter       = var.vsphere_datacenter
  datastore        = var.vsphere_datastore
  host             = var.vsphere_host
  network          = vsphere_host_port_group.LAN.name
  folder           = var.vsphere_folder
  gateway          = var.gateway
  domain           = var.domain
  ip               = each.value.ip
  name             = each.value.name
  hash             = each.value.hash
  data_folder      = each.value.data_folder
  playbook_file    = each.value.playbook_file
}



