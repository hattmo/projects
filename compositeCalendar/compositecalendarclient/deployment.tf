
terraform {
  backend "gcs" {
    bucket = "compositecalendar-tf"
    prefix = "client"
  }
}

data "terraform_remote_state" "infrastructure" {
  backend = "gcs"
  config = {
    bucket = "compositecalendar-tf"
    prefix = "infrastructure"
  }
}

provider "google" {
  project = "compositecalendar"
}

provider "kubernetes" {
  load_config_file       = "false"
  host                   = data.terraform_remote_state.infrastructure.outputs.kube_host
  client_certificate     = base64decode(data.terraform_remote_state.infrastructure.outputs.kube_client_certificate)
  client_key             = base64decode(data.terraform_remote_state.infrastructure.outputs.kube_client_key)
  cluster_ca_certificate = base64decode(data.terraform_remote_state.infrastructure.outputs.kube_ca_certificate)
}

locals {
  version   = jsondecode(file("./package.json")).version
  appname   = trimprefix(jsondecode(file("./package.json")).name, "@hattmo/")
  imagename = trimprefix(jsondecode(file("./package.json")).name, "@")
}


resource "kubernetes_deployment" "app" {
  metadata {
    name = "${local.appname}-deployment"
    labels = {
      app = local.appname
    }
  }
  spec {
    replicas = 3
    selector {
      match_labels = {
        app = local.appname
      }
    }
    template {
      metadata {
        labels = {
          app = local.appname
        }
      }
      spec {
        container {
          name  = local.appname
          image = local.imagename
        }
      }
    }
  }
}

resource "kubernetes_service" "app" {
  metadata {
    name = "${local.appname}-service"
  }
  spec {
    selector = {
      app = local.appname
    }
    session_affinity = "ClientIP"
    port {
      port        = 80
      target_port = 80
    }

    type = "NodePort"
  }
}


resource "kubernetes_ingress" "compositecalendar-ingress" {
  metadata {
    name = "compositecalendar-ingress"
    annotations = {
      "networking.gke.io/managed-certificates" = "compositecalendar-cert"
    }
  }

  spec {
    backend {
      service_name = "compositecalendarclient-service"
      service_port = 80
    }

    rule {
      http {
        path {
          backend {
            service_name = "compositecalendarclient-service"
            service_port = 80
          }
          path = "/"
        }
        path {
          backend {
            service_name = "compositecalendarauth-service"
            service_port = 80
          }
          path = "/auth"
        }
        path {
          backend {
            service_name = "compositecalendarauth-service"
            service_port = 80
          }
          path = "/login"
        }
        path {
          backend {
            service_name = "compositecalendarapi-service"
            service_port = 80
          }
          path = "/api/*"
        }
      }
    }
  }
}


resource "google_dns_managed_zone" "compositecalendar-zone" {
  name     = "compositecalendar"
  dns_name = "compositecalendar.com."
  dnssec_config {
    kind          = "dns#managedZoneDnsSecConfig"
    non_existence = "nsec3"
    state         = "on"

    default_key_specs {
      algorithm  = "rsasha256"
      key_length = 2048
      key_type   = "keySigning"
      kind       = "dns#dnsKeySpec"
    }
    default_key_specs {
      algorithm  = "rsasha256"
      key_length = 1024
      key_type   = "zoneSigning"
      kind       = "dns#dnsKeySpec"
    }
  }
}

resource "google_dns_record_set" "compositecalendar-record" {
  name         = "compositecalendar.com."
  rrdatas      = length(kubernetes_ingress.compositecalendar-ingress.load_balancer_ingress) > 0 ? kubernetes_ingress.compositecalendar-ingress.load_balancer_ingress[*].ip : ["127.0.0.1"]
  ttl          = "300"
  type         = "A"
  managed_zone = google_dns_managed_zone.compositecalendar-zone.name
}