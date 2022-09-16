

# Provider creds

variable "VSPHERE_SERVER" {
  description = "vSphere server"
  type        = string
}

variable "VSPHERE_USER" {
  description = "vSphere username"
  type        = string
}

variable "VSPHERE_PASSWORD" {
  description = "vSphere password"
  type        = string
  sensitive   = true
}

variable "DATACENTER" {
  type = string
}
variable "DATASTORE" {
  type = string
}
variable "BASE_ISO_DATASTORE_PATH" {
  type = string
}

# Existing resources

# variable "datacenter" {
#   description = "vSphere data center"
#   type        = string
# }

# variable "cluster" {
#   description = "vSphere cluster"
#   type        = string
# }

# variable "datastore" {
#   description = "vSphere datastore"
#   type        = string
# }

# variable "network_name" {
#   description = "vSphere network name"
#   type        = string
# }

# # Global config

# variable "gateway" {
#   description = "IPv4 Gateway"
#   type        = string
# }

# variable "domain" {
#   description = "Domain"
#   type        = string
# }

# # Individual config

# variable "ldap_template" {
#   type = string
# }
# variable "ldap_ip" {
#   type = string
# }
# variable "dhcp_template" {
#   type = string
# }
# variable "dhcp_ip" {
#   type = string
# }
# variable "nexus_template" {
#   type = string
# }
# variable "nexus_ip" {
#   type = string
# }
# variable "nextcloud_template" {
#   type = string
# }
# variable "nextcloud_ip" {
#   type = string
# }
# variable "gitlab_template" {
#   type = string
# }
# variable "gitlab_ip" {
#   type = string
# }
# variable "ghidra_template" {
#   type = string
# }
# variable "ghidra_ip" {
#   type = string
# }
# variable "jenkins_template" {
#   type = string
# }
# variable "jenkins_ip" {
#   type = string
# }
# variable "keycloak_template" {
#   type = string
# }
# variable "keycloak_ip" {
#   type = string
# }
