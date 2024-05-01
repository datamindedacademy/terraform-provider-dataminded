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

resource "dataminded_user" "me" {
  name = "Jonas"
}

resource "dataminded_chapter" "platform_chapter" {
  name = "Platform"
}

resource "dataminded_chapter_member" "jonas_platform" {
  chapter = dataminded_chapter.platform_chapter.id
  member  = dataminded_user.me.id
  role    = "Contributor"
}
