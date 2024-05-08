resource "dataminded_chapter_member" "jonas_platform" {
  chapter = dataminded_chapter.platform_chapter.id
  member  = dataminded_user.me.id
  role    = "Contributor"
}
