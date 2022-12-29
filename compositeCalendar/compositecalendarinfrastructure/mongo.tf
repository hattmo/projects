provider "mongodbatlas" {}

resource "mongodbatlas_project" "compositecalendar_project" {
  name   = "compositecalendar"
  org_id = "5c204824a6f23983351d687b"
}

resource "mongodbatlas_cluster" "compositecalendar_cluster" {
  project_id                  = mongodbatlas_project.compositecalendar_project.id
  provider_name               = "TENANT"
  backing_provider_name       = "GCP"
  name                        = "compositecalendar"
  provider_region_name        = "CENTRAL_US"
  provider_instance_size_name = "M2"
  mongo_db_major_version      = "4.2"
  num_shards                  = 1
  replication_factor          = 3

}

resource "mongodbatlas_project_ip_whitelist" "compositecalendar_whitelist" {
  project_id = mongodbatlas_project.compositecalendar_project.id
  cidr_block = "0.0.0.0/0"
  comment    = "Kubernetes Whitelist"
}

resource "random_password" "password" {
  length  = 16
  special = false
  keepers = {
    database = mongodbatlas_cluster.compositecalendar_cluster.cluster_id
  }
}

resource "mongodbatlas_database_user" "compositecalendar_user" {
  username           = "compositecalendar"
  password           = random_password.password.result
  project_id         = mongodbatlas_project.compositecalendar_project.id
  auth_database_name = "admin"

  roles {
    role_name     = "readWriteAnyDatabase"
    database_name = "admin"
  }
}


output "db_connection" {
  value     = mongodbatlas_cluster.compositecalendar_cluster.srv_address
  sensitive = true
}

output "db_username" {
  value     = mongodbatlas_database_user.compositecalendar_user.username
  sensitive = true
}

output "db_password" {
  value     = mongodbatlas_database_user.compositecalendar_user.password
  sensitive = true
}