package user

import (
	"github.com/hashicorp/terraform-plugin-framework/types"
)

type UserDatasourceModel struct {
	Id   types.Int64  `tfsdk:"id"`
	Name types.String `tfsdk:"name"`
}

type UserResourceModel struct {
	Id   types.Int64  `tfsdk:"id"`
	Name types.String `tfsdk:"name"`
}
