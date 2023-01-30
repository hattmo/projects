resource "vsphere_virtual_machine" "vm" {
  name             = "vault"
  resource_pool_id = data.vsphere_host.esxi1.resource_pool_id
  datastore_id     = data.vsphere_datastore.datastore1.id
  guest_id         = data.vsphere_virtual_machine.vault_template.guest_id

  network_interface {
    network_id = data.vsphere_network.lan.id
  }

  clone {
    template_uuid = data.vsphere_virtual_machine.vault_template.id
  }

  disk {
    label            = "disk0"
    size             = data.vsphere_virtual_machine.vault_template.disks.0.size
    thin_provisioned = data.vsphere_virtual_machine.vault_template.disks.0.thin_provisioned
  }
  folder = vsphere_folder.infra.path
}
