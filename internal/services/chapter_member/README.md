# Chapter Member

You can interact with the chapter member API endpoints via the CRUD calls in `dataminded_api/chapter_member.go`

Steps to implement the `chapter_member`

1. Complete the ChapterMemberModel struct in `main.go` and the `Schema` interface method in `chapter_member_resource.go`. Make sure to [validate](https://developer.hashicorp.com/terraform/plugin/framework/validation) the `Role `attribute: there are only two valid roles, `Lead `and `Contributor `. The role is [optional](https://developer.hashicorp.com/terraform/plugin/sdkv2/schemas/schema-behaviors), with default value `Contributor `.
2. Complete the `Create` / `Read` / `Update` and `Delete` interface methods of the chapter_member resource. If a chapter_member does not seem to exist while reading, remove it from the state so it gets created.
3. Run the acceptance tests to validate your implementation: `make testacc`
