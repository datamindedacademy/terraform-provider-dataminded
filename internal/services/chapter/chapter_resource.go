package chapter

import (
	"context"
	"fmt"

	"terraform-provider-dataminded/internal/dataminded_api"
	"terraform-provider-dataminded/internal/logging"

	"github.com/hashicorp/terraform-plugin-framework/resource"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema"
	"github.com/hashicorp/terraform-plugin-framework/types"
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
		Attributes: map[string]schema.Attribute{
			"id": schema.Int64Attribute{
				Computed:    true,
				Description: "Id of the chapter in the sqlite database.",
			},
			"name": schema.StringAttribute{
				Required:    true,
				Description: "Name of the chapter",
			},
		},
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

	name := plan.Name.ValueString()

	chapter, err := dataminded_api.CreateChapter(r.Connection, name)

	if err != nil {
		logging.AddError(ctx, "Chapter creation failed", err)
		return
	}

	// Chapter creation successful --> Set state of computed variables (Id)
	plan.Id = types.Int64Value(int64(chapter.Id))

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

	id := state.Id.ValueInt64()
	chapter, err := dataminded_api.ReadChapter(r.Connection, int(id))

	if err != nil {
		logging.AddError(ctx, "Reading chapter failed", err)
		return
	}

	if !dataminded_api.ChapterExists(chapter) {
		resp.State.RemoveResource(ctx)
		return
	}

	// Set the read values
	// We don't have to set Id since this value was used to read
	state.Name = types.StringValue(chapter.Name)

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

	id := int(state.Id.ValueInt64())
	newName := plan.Name.ValueString()

	chapter, err := dataminded_api.UpdateChapter(r.Connection, id, newName)

	if err != nil {
		logging.AddError(ctx, "Updating chapter failed", err)
		return
	}

	// Chapter update successful --> Set state of computed variables (Id)
	plan.Id = types.Int64Value(int64(chapter.Id))

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

	id := int(state.Id.ValueInt64())

	err := dataminded_api.DeleteChapter(r.Connection, id)

	if err != nil {
		logging.AddError(ctx, "Dropping chapter failed", err)
	}
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
