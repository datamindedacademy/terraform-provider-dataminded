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


locals {
  chapter_config = yamldecode(file("${path.module}/chapter_config.yaml"))
  chapters       = keys(local.chapter_config)
  users          = toset(flatten([for users in values(local.chapter_config) : [for user in users : user.name]]))
}


resource "dataminded_user" "user" {
  for_each = local.users
  name     = each.key
}

resource "dataminded_chapter" "chapter" {
  for_each = toset(local.chapters)
  name     = each.key
}

// Exercise 3: the provider-defined function replaces the nested comprehension
// that flattened chapter_config.yaml by hand. The chapter is carried in the
// map key rather than the value, hence the split.
resource "dataminded_chapter_member" "chapter_member" {
  for_each = provider::dataminded::chapter_config_parser(file("${path.module}/chapter_config.yaml"))
  chapter  = dataminded_chapter.chapter[split("-", each.key)[0]].id
  member   = dataminded_user.user[each.value.name].id
  role     = each.value.role
}



