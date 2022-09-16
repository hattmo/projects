resource "null_resource" "gitlab_packer" {
  triggers = {
    "gitlab_files_hash" = var.gitlab_files_hash
  }

  provisioner "local-exec" {
    command     = "packer build -on-error=abort build.pkr.hcl"
    working_dir = "../packer/"
    environment = {
      PKR_VAR_vsphere_server   = var.vsphere_server,
      PKR_VAR_vsphere_username = var.vsphere_username,
      PKR_VAR_vsphere_password = var.vsphere_password,
      PKR_VAR_vsphere_datacenter       = var.vsphere_datacenter,
      PKR_VAR_vsphere_datastore = var.vsphere_datastore,
      PKR_VAR_vsphere_folder   = var.vsphere_folder,
      PKR_VAR_vsphere_host     = var.vsphere_host,
      PKR_VAR_vsphere_network  = var.vsphere_network,
      PKR_VAR_vm_name          = local.gitlab_image_name,
    }
  }
}
