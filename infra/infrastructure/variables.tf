

# Provider creds

variable "vsphere_server" {
  description = "vSphere server"
  type        = string
}

variable "vsphere_username" {
  description = "vSphere username"
  type        = string
}

variable "vsphere_password" {
  description = "vSphere password"
  type        = string
}

# Existing resources


variable "vsphere_datacenter" {
  type = string
}

variable "vsphere_datastore" {
  type = string
}

variable "vsphere_cluster" {
  type = string
}


variable "vsphere_host" {
  type = string
}

variable "vsphere_folder"{
  type = string
}

variable "vsphere_network" {
  description = "vSphere network name"
  type        = string
}

# Global config

variable "gateway" {
  description = "IPv4 Gateway"
  type        = string
}

variable "domain" {
  description = "Domain"
  type        = string
}

# Individual config

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

variable "gitlab_ip" {
  type = string
}

variable "gitlab_files_hash" {
  type = string
}

locals {
  gitlab_image_name = join("_",["gitlab",var.gitlab_files_hash])
}

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
