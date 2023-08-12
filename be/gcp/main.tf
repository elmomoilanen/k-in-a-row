terraform {
  backend "gcs" {
    bucket = "ad236620-c024-4eeb-91ce-89dda30f993f"
    prefix = "terraform/state"
  }
}

provider "google" {
  project = var.project_id
  region  = var.region
}

resource "google_cloud_run_service" "kinarow_backend" {
  name     = var.service_name
  location = var.region

  template {
    spec {
      containers {
        image = "${var.region}-${var.ar_domain}/${var.project_id}/${var.service_name}/${var.image_name}:${var.image_tag}"

        ports {
          name           = "http1"
          container_port = 8080
        }

        resources {
          limits = {
            memory = "512Mi"
            cpu    = "1"
          }
        }

        env {
          name  = "CLIENT_URL"
          value = var.client_url
        }
      }
      timeout_seconds = 60
    }

    metadata {
      annotations = {
        "autoscaling.knative.dev/maxScale" = "1"
        "run.googleapis.com/client-name"   = "terraform"
      }
    }
  }

  traffic {
    percent         = 100
    latest_revision = true
  }

  autogenerate_revision_name = true
}

resource "google_cloud_run_service_iam_member" "service-invoker" {
  location = var.region
  project  = var.project_id
  service  = google_cloud_run_service.kinarow_backend.name
  role     = "roles/run.invoker"
  member   = "allUsers"
}
