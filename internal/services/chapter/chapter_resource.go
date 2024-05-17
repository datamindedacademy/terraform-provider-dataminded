package chapter

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
	_ resource.Resource              = &ChapterResource{}
	_ resource.ResourceWithConfigure = &ChapterResource{}
)

func NewChapterResource() resource.Resource {
	return &ChapterResource{}
}

type ChapterResource struct {
	Connection dataminded_api.Connection
}

func (r *ChapterResource) Metadata(_ context.Context, req resource.MetadataRequest, resp *resource.MetadataResponse) {
	resp.TypeName = req.ProviderTypeName + "_chapter"
}

func (r *ChapterResource) Schema(_ context.Context, _ resource.SchemaRequest, resp *resource.SchemaResponse) {
	resp.Schema = schema.Schema{
		Description: "Manage Data Minded chapters",
		Attributes:  map[string]schema.Attribute{},
	}
}

func (r *ChapterResource) Create(ctx context.Context, req resource.CreateRequest, resp *resource.CreateResponse) {
	ctx = logging.WithDiagnostics(ctx, &resp.Diagnostics)

	var plan ChapterResourceModel
	diags := req.Plan.Get(ctx, &plan)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	// TODO: use plan to create the chapter, and return result to the state

	diags = resp.State.Set(ctx, plan)
	resp.Diagnostics.Append(diags...)
}

func (r *ChapterResource) Read(ctx context.Context, req resource.ReadRequest, resp *resource.ReadResponse) {
	ctx = logging.WithDiagnostics(ctx, &resp.Diagnostics)

	var state ChapterResourceModel

	// Read input configured in data block
	resp.Diagnostics.Append(
		req.State.Get(ctx, &state)...,
	)

	if logging.HasError(ctx) {
		return
	}
	// TODO: use state to read the chapter, and return result to the state.
	// If the chapter does not exist, mark it for deletion from the state.

	diags := resp.State.Set(ctx, &state)
	resp.Diagnostics.Append(diags...)
}

func (r *ChapterResource) Update(ctx context.Context, req resource.UpdateRequest, resp *resource.UpdateResponse) {
	ctx = logging.WithDiagnostics(ctx, &resp.Diagnostics)

	var state ChapterResourceModel
	resp.Diagnostics.Append(
		req.State.Get(ctx, &state)...,
	)

	var plan ChapterResourceModel
	resp.Diagnostics.Append(
		req.Plan.Get(ctx, &plan)...,
	)

	if resp.Diagnostics.HasError() {
		return
	}

	// TODO: use the plan to update the chapter

	diags := resp.State.Set(ctx, plan)
	resp.Diagnostics.Append(diags...)
}

func (r *ChapterResource) Delete(ctx context.Context, req resource.DeleteRequest, resp *resource.DeleteResponse) {
	ctx = logging.WithDiagnostics(ctx, &resp.Diagnostics)

	var state ChapterResourceModel

	// Read input configured in data block
	resp.Diagnostics.Append(
		req.State.Get(ctx, &state)...,
	)

	if logging.HasError(ctx) {
		return
	}

	// TODO: use state to delete the chapter
}

func (r *ChapterResource) Configure(_ context.Context, req resource.ConfigureRequest, resp *resource.ConfigureResponse) {

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
