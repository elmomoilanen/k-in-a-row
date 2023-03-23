output "url" {
  value = google_cloud_run_service.kinarow_backend.status[0].url
}
