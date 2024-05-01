package provider

import (
	"github.com/hashicorp/terraform-plugin-framework/types"
)

type ProviderConfigModel struct {
	Host types.String `tfsdk:"host"`
	Port types.Int64  `tfsdk:"port"`
}
