variable "project_id" {
  type = string
}

variable "service_name" {
  type = string
}

variable "region" {
  type = string
}

variable "image_tag" {
  type    = string
  default = "latest"
}

variable "client_url" {
  type = string
}
