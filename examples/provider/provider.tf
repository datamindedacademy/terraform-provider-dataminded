terraform {
  required_providers {
    dataminded = {
      source = "hashicorp.com/dev/dataminded"
    }
  }
}

provider "dataminded" {
  host = "http://localhost"
  port = 3000
}
