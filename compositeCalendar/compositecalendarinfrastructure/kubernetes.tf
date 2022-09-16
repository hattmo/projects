provider "google" {
  project = "compositecalendar"
}


resource "google_container_cluster" "primary" {
  name     = "compositecalendar-cluster"
  location = "us-central1"

  remove_default_node_pool = true
  initial_node_count       = 1

  min_master_version = "1.15.9-gke.26"
  enable_legacy_abac = true
  master_auth {
    client_certificate_config {
      issue_client_certificate = true
    }
  }
}

resource "google_container_node_pool" "primary_preemptible_nodes" {
  name       = "compositecalendar-nodepool"
  location   = "us-central1"
  cluster    = google_container_cluster.primary.name
  node_count = 1

  node_config {
    preemptible  = true
    machine_type = "n1-standard-1"

    metadata = {
      disable-legacy-endpoints = "true"
    }

    oauth_scopes = [
      "https://www.googleapis.com/auth/logging.write",
      "https://www.googleapis.com/auth/monitoring",
    ]
  }
}

output "kube_client_certificate" {
  value     = google_container_cluster.primary.master_auth.0.client_certificate
  sensitive = true
}

output "kube_client_key" {
  value     = google_container_cluster.primary.master_auth.0.client_key
  sensitive = true
}

output "kube_ca_certificate" {
  value     = google_container_cluster.primary.master_auth.0.cluster_ca_certificate
  sensitive = true
}

output "kube_host" {
  value     = google_container_cluster.primary.endpoint
  sensitive = true
}