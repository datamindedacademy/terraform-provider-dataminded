package chapter_member

import (
	"github.com/hashicorp/terraform-plugin-framework/types"
)

type ChapterMemberResourceModel struct {
	Id        types.String `tfsdk:"id"`
	ChapterId types.Int64  `tfsdk:"chapter"`
	UserId    types.Int64  `tfsdk:"member"`
	Role      types.String `tfsdk:"role"`
}
