# module "ldap_box" {
#   source        = "./base_machine"
#   hostname      = "ldap"
#   template_name = var.ldap_template
#   ip            = var.ldap_ip
#   gateway       = var.gateway
#   domain        = var.domain
#   datacenter    = var.datacenter
#   cluster       = var.cluster
#   datastore     = var.datastore
#   network_name  = var.network_name
# }

# module "dhcp_box" {
#   source        = "./base_machine"
#   hostname      = "dhcp"
#   template_name = var.dhcp_template
#   ip            = var.dhcp_ip
#   gateway       = var.gateway
#   domain        = var.domain
#   datacenter    = var.datacenter
#   cluster       = var.cluster
#   datastore     = var.datastore
#   network_name  = var.network_name
# }

# module "nexus_box" {
#   source        = "./base_machine"
#   hostname      = "nexus"
#   template_name = var.nexus_template
#   ip            = var.nexus_ip
#   gateway       = var.gateway
#   domain        = var.domain
#   datacenter    = var.datacenter
#   cluster       = var.cluster
#   datastore     = var.datastore
#   network_name  = var.network_name
# }

# module "nextcloud_box" {
#   source        = "./base_machine"
#   hostname      = "nextcloud"
#   template_name = var.nextcloud_template
#   ip            = var.nextcloud_ip
#   gateway       = var.gateway
#   domain        = var.domain
#   datacenter    = var.datacenter
#   cluster       = var.cluster
#   datastore     = var.datastore
#   network_name  = var.network_name
# }

module "gitlab_box" {
  depends_on = [
    null_resource.gitlab_packer
  ]
  source     = "./base_machine"
  name       = "gitlab"
  template   = local.gitlab_image_name
  ip         = var.gitlab_ip
  gateway    = var.gateway
  domain     = var.domain
  datacenter = var.vsphere_datacenter
  datastore  = var.vsphere_datastore
  network    = var.vsphere_network
  folder     = var.vsphere_folder
  cluster    = var.vsphere_cluster
}

# module "ghidra_box" {
#   source        = "./base_machine"
#   hostname      = "ghidra"
#   template_name = var.ghidra_template
#   ip            = var.ghidra_ip
#   gateway       = var.gateway
#   domain        = var.domain
#   datacenter    = var.datacenter
#   cluster       = var.cluster
#   datastore     = var.datastore
#   network_name  = var.network_name
# }

# module "jenkins_box" {
#   source        = "./base_machine"
#   hostname      = "jenkins"
#   template_name = var.jenkins_template
#   ip            = var.jenkins_ip
#   gateway       = var.gateway
#   domain        = var.domain
#   datacenter    = var.datacenter
#   cluster       = var.cluster
#   datastore     = var.datastore
#   network_name  = var.network_name
# }

# module "keycloak_box" {
#   source        = "./base_machine"
#   hostname      = "keycloak"
#   template_name = var.keycloak_template
#   ip            = var.keycloak_ip
#   gateway       = var.gateway
#   domain        = var.domain
#   datacenter    = var.datacenter
#   cluster       = var.cluster
#   datastore     = var.datastore
#   network_name  = var.network_name
# }
