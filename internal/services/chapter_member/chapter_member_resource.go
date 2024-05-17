package chapter_member

import (
	"context"
	"fmt"

	"terraform-provider-dataminded/internal/dataminded_api"
	"terraform-provider-dataminded/internal/logging"

	"github.com/hashicorp/terraform-plugin-framework/resource"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema"
)

// Ensure the implementation satisfies the expected interfaces.
var (
	_ resource.Resource              = &ChapterMemberResource{}
	_ resource.ResourceWithConfigure = &ChapterMemberResource{}
)

func NewChapterMemberResource() resource.Resource {
	return &ChapterMemberResource{}
}

type ChapterMemberResource struct {
	Connection dataminded_api.Connection
}

func (r *ChapterMemberResource) Metadata(_ context.Context, req resource.MetadataRequest, resp *resource.MetadataResponse) {
	resp.TypeName = req.ProviderTypeName + "_chapter_member"
}

func (r *ChapterMemberResource) Schema(_ context.Context, _ resource.SchemaRequest, resp *resource.SchemaResponse) {
	resp.Schema = schema.Schema{
		Description: "Manage Data Minded chapter members",
		Attributes:  map[string]schema.Attribute{},
	}
}

func (r *ChapterMemberResource) Create(ctx context.Context, req resource.CreateRequest, resp *resource.CreateResponse) {
	ctx = logging.WithDiagnostics(ctx, &resp.Diagnostics)

	var plan ChapterMemberResourceModel
	diags := req.Plan.Get(ctx, &plan)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}
	// TODO: use plan to create the chapter_member, and return result to the state

	diags = resp.State.Set(ctx, plan)
	resp.Diagnostics.Append(diags...)
}

func (r *ChapterMemberResource) Read(ctx context.Context, req resource.ReadRequest, resp *resource.ReadResponse) {
	ctx = logging.WithDiagnostics(ctx, &resp.Diagnostics)

	var state ChapterMemberResourceModel

	// Read input configured in data block
	resp.Diagnostics.Append(
		req.State.Get(ctx, &state)...,
	)

	if logging.HasError(ctx) {
		return
	}

	// Use the state to read the chapter_member from the API
	// If the chapter_member is not found, mark it for deletion

	diags := resp.State.Set(ctx, &state)
	resp.Diagnostics.Append(diags...)
}

func (r *ChapterMemberResource) Update(ctx context.Context, req resource.UpdateRequest, resp *resource.UpdateResponse) {
	ctx = logging.WithDiagnostics(ctx, &resp.Diagnostics)

	var state ChapterMemberResourceModel
	resp.Diagnostics.Append(
		req.State.Get(ctx, &state)...,
	)

	var plan ChapterMemberResourceModel
	resp.Diagnostics.Append(
		req.Plan.Get(ctx, &plan)...,
	)

	if resp.Diagnostics.HasError() {
		return
	}

	// Use the plan to update the chapter_member

	diags := resp.State.Set(ctx, plan)
	resp.Diagnostics.Append(diags...)
}

func (r *ChapterMemberResource) Delete(ctx context.Context, req resource.DeleteRequest, resp *resource.DeleteResponse) {
	ctx = logging.WithDiagnostics(ctx, &resp.Diagnostics)

	var state ChapterMemberResourceModel

	// Read input configured in data block
	resp.Diagnostics.Append(
		req.State.Get(ctx, &state)...,
	)

	if logging.HasError(ctx) {
		return
	}
	// Use the state to delete the chapter_member

}

func (r *ChapterMemberResource) Configure(_ context.Context, req resource.ConfigureRequest, resp *resource.ConfigureResponse) {

	if req.ProviderData == nil {
		return
	}

	connection, ok := req.ProviderData.(*dataminded_api.Connection)

	if !ok {
		resp.Diagnostics.AddError(
			"Unexpected Configure Type",
			fmt.Sprintf("Expected *dataminded_api.Connection got: %T.", req.ProviderData),
		)

		return
	}

	r.Connection = *connection
}
