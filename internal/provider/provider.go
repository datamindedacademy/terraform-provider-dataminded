// Copyright (c) HashiCorp, Inc.
// SPDX-License-Identifier: MPL-2.0

package provider

import (
	"context"

	"terraform-provider-dataminded/internal/dataminded_api"
	"terraform-provider-dataminded/internal/services/chapter"
	"terraform-provider-dataminded/internal/services/chapter_member"
	"terraform-provider-dataminded/internal/services/user"

	"terraform-provider-dataminded/internal/services/functions"

	"github.com/hashicorp/terraform-plugin-framework/datasource"
	"github.com/hashicorp/terraform-plugin-framework/function"
	"github.com/hashicorp/terraform-plugin-framework/provider"
	"github.com/hashicorp/terraform-plugin-framework/provider/schema"
	"github.com/hashicorp/terraform-plugin-framework/resource"
)

// Ensure ScaffoldingProvider satisfies various provider interfaces.
var _ provider.Provider = &DataMindedProvider{}
var _ provider.ProviderWithFunctions = &DataMindedProvider{}

// ScaffoldingProvider defines the provider implementation.
type DataMindedProvider struct {
	// version is set to the provider version on release, "dev" when the
	// provider is built and ran locally, and "test" when running acceptance
	// testing.
	version string
}

func (p *DataMindedProvider) Metadata(ctx context.Context, req provider.MetadataRequest, resp *provider.MetadataResponse) {
	resp.TypeName = "dataminded"
	resp.Version = p.version
}

func (p *DataMindedProvider) Schema(ctx context.Context, req provider.SchemaRequest, resp *provider.SchemaResponse) {
	resp.Schema = schema.Schema{
		Attributes: map[string]schema.Attribute{
			"host": schema.StringAttribute{
				MarkdownDescription: "Host address where the Data Minded API runs",
				Required:            true,
			},
			"port": schema.Int64Attribute{
				MarkdownDescription: "Port of the Data Minded API host",
				Required:            true,
			},
		},
	}
}

func (p *DataMindedProvider) Configure(ctx context.Context, req provider.ConfigureRequest, resp *provider.ConfigureResponse) {
	var data ProviderConfigModel

	resp.Diagnostics.Append(req.Config.Get(ctx, &data)...)

	if resp.Diagnostics.HasError() {
		return
	}

	connection := dataminded_api.Connection{
		Host: data.Host.ValueString(),
		Port: data.Port.ValueInt64(),
	}

	resp.DataSourceData = &connection
	resp.ResourceData = &connection
}

func (p *DataMindedProvider) Resources(ctx context.Context) []func() resource.Resource {
	return []func() resource.Resource{
		user.NewUserResource,
		chapter.NewChapterResource,
		chapter_member.NewChapterMemberResource,
	}
}

func (p *DataMindedProvider) DataSources(ctx context.Context) []func() datasource.DataSource {
	return []func() datasource.DataSource{}
}

func (p *DataMindedProvider) Functions(ctx context.Context) []func() function.Function {
	return []func() function.Function{
		functions.NewConfigParser,
	}
}

func New(version string) func() provider.Provider {
	return func() provider.Provider {
		return &DataMindedProvider{
			version: version,
		}
	}
}
