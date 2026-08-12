package chapter

import (
	"github.com/hashicorp/terraform-plugin-framework/types"
)

type ChapterResourceModel struct {
	Id   types.Int64  `tfsdk:"id"`
	Name types.String `tfsdk:"name"`
}
