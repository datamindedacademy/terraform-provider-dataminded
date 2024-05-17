terraform {
  required_providers {
    dataminded = {
      source = "dataminded/data-minded"
    }

  }
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
    }

  ]...)
  chapter_roles_bis = provider::dataminded::chapter_config_parser(file("${path.module}/chapter_config.yaml"))
}
