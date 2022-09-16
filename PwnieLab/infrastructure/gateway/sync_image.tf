resource "null_resource" "packer" {
  triggers = {
    "files_hash" = var.hash
  }

  provisioner "local-exec" {
    command     = "packer build build.pkr.hcl"
    working_dir = "../packer/"
    environment = {
      PKR_VAR_vsphere_server     = var.vsphere_server,
      PKR_VAR_vsphere_username   = var.vsphere_username,
      PKR_VAR_vsphere_password   = var.vsphere_password,
      PKR_VAR_vsphere_datacenter = var.datacenter,
      PKR_VAR_vsphere_datastore  = var.datastore,
      PKR_VAR_vsphere_folder     = var.folder,
      PKR_VAR_vsphere_host       = var.host,
      PKR_VAR_vsphere_network    = var.network,
      PKR_VAR_vm_name            = local.image_name,
      PKR_VAR_data_folder        = var.data_folder,
      PKR_VAR_playbook_file      = var.playbook_file
    }
  }
}
