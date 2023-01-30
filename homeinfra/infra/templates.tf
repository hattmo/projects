data "vsphere_virtual_machine" "vault_template" {
  name          = "vault_d0bca9a6-fc7f-41e3-963f-fc53a87556d0"
  datacenter_id = data.vsphere_datacenter.datacenter.id
}
