provider "vsphere" {
  vsphere_server = var.vsphere_server
  user           = var.vsphere_username
  password       = var.vsphere_password

  # If you have a self-signed cert
  allow_unverified_ssl = true
}
