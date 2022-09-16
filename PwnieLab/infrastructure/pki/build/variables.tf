variable "common_name" {
  description = "Common Name"
  default     = "mayhem.lab"
}

variable "cert_domains" {
  description = "Subject Alternative Names"
  type        = list(string)
}

variable "organization_name" {
  description = "Organization Name"
  default     = "Mayhem Lab"
}
