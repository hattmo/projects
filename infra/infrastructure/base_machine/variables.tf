#Existing resources

variable "datacenter" {
  description = "vSphere data center"
  type        = string
}

variable "datastore" {
  description = "vSphere datastore"
  type        = string
}

variable "cluster" {
  description = "vSphere cluster"
  type        = string
}

variable "network" {
  description = "vSphere network name"
  type        = string
}

variable "folder" {
  type = string
}

# VM Config

variable "template" {
  description = "Template name (ie: image_path)"
  type        = string
}

variable "ip" {
  type = string
  description = "Static Ip of the machine"
}

variable "gateway" {
  description = "IPv4 Gateway"
  type = string
}

variable "domain" {
  description = "Domain"
  type = string
}

variable "name" {
  description = "Hostname"
  type = string
}
