module "ca" {
  source             = "../modules/ca"
  ca_common_name     = "${var.common_name}"
  organization_name  = "${var.organization_name}"
  ca_public_key_path = "../ca.crt"
}

module "lab-cert" {
  source                = "../modules/certificate"
  common_name           = "${var.common_name}"
  organization_name     = "${var.organization_name}"
  cert_private_key_path = "../${var.common_name}.key"
  dns_names             = var.cert_domains 
  ca_key_algorithm      = "${module.ca.ca_key_algorithm}"
  ca_private_key_pem    = "${module.ca.ca_private_key_pem}"
  ca_cert_pem           = "${module.ca.ca_cert_pem}"
  cert_public_key_path  = "../${var.common_name}.crt"
}
