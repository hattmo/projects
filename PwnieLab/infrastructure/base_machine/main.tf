data "vsphere_datacenter" "dc" {
  name = var.datacenter
}

data "vsphere_datastore" "datastore" {
  name          = var.datastore
  datacenter_id = data.vsphere_datacenter.dc.id
}

data "vsphere_resource_pool" "pool" {
  name          = "${var.host}/Resources"
  datacenter_id = "${data.vsphere_datacenter.dc.id}"
}

data "vsphere_network" "network" {
  name          = var.network
  datacenter_id = data.vsphere_datacenter.dc.id
}

# data "vsphere_compute_cluster" "compute_cluster" {
#   name          = var.cluster
#   datacenter_id = "${data.vsphere_datacenter.dc.id}"
# }

data "vsphere_virtual_machine" "base_template" {
  depends_on = [
    null_resource.packer
  ]
  name          = local.image_name
  datacenter_id = data.vsphere_datacenter.dc.id
}

resource "vsphere_virtual_machine" "cloned-vm" {
  depends_on = [
    null_resource.packer
  ]
  name             = var.name
  folder           = var.folder
  datastore_id     = data.vsphere_datastore.datastore.id
  resource_pool_id = data.vsphere_resource_pool.pool.id
  num_cpus = 2
  memory   = 2048

  network_interface {
    network_id = data.vsphere_network.network.id
  }

  wait_for_guest_net_timeout = -1
  wait_for_guest_ip_timeout  = -1

  disk {
    label            = "disk0"
    thin_provisioned = true
    size             = 50
  }

  guest_id = "ubuntu64Guest"

  clone {
    template_uuid = data.vsphere_virtual_machine.base_template.id
    customize {
      linux_options {
        host_name = var.name
        domain    = var.domain
      }
      network_interface {
        ipv4_address = var.ip
        ipv4_netmask = 24
      }
      ipv4_gateway = var.gateway
    }
  }
}
