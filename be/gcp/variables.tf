variable "project_id" {
  type = string
}

variable "region" {
  type = string
}

variable "ar_domain" {
  type = string
  default = "docker.pkg.dev"
}

variable "image_name" {
  type = string
}

variable "image_tag" {
  type    = string
  default = "latest"
}

variable "service_name" {
  type = string
}

variable "client_url" {
  type = string
}
