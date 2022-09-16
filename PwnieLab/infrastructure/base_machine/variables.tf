
variable "vsphere_server" {
  type = string
}
variable "vsphere_username" {
  type = string
}
variable "vsphere_password" {
  type = string
}

#Existing resources

variable "datacenter" {
  type        = string
}

variable "datastore" {
  type        = string
}

variable "host" {
  type        = string
}

variable "network" {
  type        = string
}

variable "folder" {
  type = string
}

# VM Config

variable "ip" {
  type = string
}

variable "gateway" {
  type = string
}

variable "domain" {
  type = string
}

variable "name" {
  type = string
}

variable "hash" {
  type = string
}

locals {
  image_name = join("_",[var.name,var.hash])
}

variable "data_folder" {
  type = string
}
variable "playbook_file" {
  type = string
}