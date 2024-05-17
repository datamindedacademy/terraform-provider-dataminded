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
  chapter_roles = merge([
    for chapter, users in local.chapter_config : {
      for user in users : "${chapter}-${user.name}" => {
        user    = user.name
        role    = try(user.role, "Contributor")
        chapter = chapter
      }
  }]...) // Bleh, this is ugly... Let's use a provider-defined function instead!
}


resource "dataminded_user" "user" {
  for_each = local.users
  name     = each.key
}

resource "dataminded_chapter" "chapter" {
  for_each = local.chapters
  name     = each.key
}

resource "dataminded_chapter_member" "chapter_member" {
  for_each = provider::dataminded::chapter_config_parser(file("${path.module}/chapter_config.yaml"))
  chapter  = dataminded_chapter.chapter[each.value.chapter].id
  member   = dataminded_user.user[each.value.user].id
}



