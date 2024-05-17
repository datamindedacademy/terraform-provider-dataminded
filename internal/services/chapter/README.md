# Chapter 

You can interact with the chapter API endpoints via the CRUD calls in `dataminded_api/chapter.go`

Steps to implement the `chapter`

1. Complete the ChapterModel struct in `main.go` and the `Schema` interface method in `chapter_resource.go`.
2. Complete the `Create` / `Read` / `Update` and `Delete` interface methods of the chapter resource. If a chapter does not seem to exist while reading, remove it from the state so it gets created.
3. Run the acceptance tests to validate your implementation: `make testacc`
