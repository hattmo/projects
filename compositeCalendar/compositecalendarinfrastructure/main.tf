terraform {
  backend "gcs" {
    bucket = "compositecalendar-tf"
    prefix = "infrastructure"
  }
}