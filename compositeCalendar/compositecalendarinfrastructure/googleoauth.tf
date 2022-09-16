variable "OAUTH_CLIENT_ID" {}
variable "OAUTH_CLIENT_SECRET" {}

output "oauth_client_id" {
    value = var.OAUTH_CLIENT_ID
    sensitive = true
}

output "oauth_client_secret" {
    value = var.OAUTH_CLIENT_SECRET
    sensitive = true
}