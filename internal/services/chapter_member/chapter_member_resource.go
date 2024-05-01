package chapter_member

import (
	"context"
	"fmt"

	"terraform-provider-dataminded/internal/dataminded_api"
	"terraform-provider-dataminded/internal/logging"

	"github.com/hashicorp/terraform-plugin-framework-validators/stringvalidator"
	"github.com/hashicorp/terraform-plugin-framework/resource"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/stringdefault"
	"github.com/hashicorp/terraform-plugin-framework/schema/validator"
	"github.com/hashicorp/terraform-plugin-framework/types"
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
		Description: fmt.Sprintf("Manage Data Minded chapter members"),
		Attributes: map[string]schema.Attribute{
			"id": schema.StringAttribute{
				Computed:    true,
				Description: "Internal id of terraform to manage this resource",
			},
			"chapter": schema.Int64Attribute{
				Required:    true,
				Description: "Id of the chapter in the sqlite database.",
			},
			"member": schema.Int64Attribute{
				Required:    true,
				Description: "Id of the user in the sqlite database.",
			},
			"role": schema.StringAttribute{
				Optional:    true,
				Computed:    true,
				Description: "Role of the chapter member",
				Default:     stringdefault.StaticString("Contributor"),
				Validators: []validator.String{
					stringvalidator.OneOf([]string{"Contributor", "Lead"}...),
				},
			},
		},
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

	chapterId := int(plan.ChapterId.ValueInt64())
	userId := int(plan.UserId.ValueInt64())
	role := plan.Role.ValueString()

	err := dataminded_api.CreateChapterMember(r.Connection, chapterId, userId, role)

	if err != nil {
		logging.AddError(ctx, "Creation of chapter member failed", err)
		return
	}

	// ChapterMember creation successfull --> Set state of computed variables (Id)
	plan.Id = types.StringValue(fmt.Sprintf("chapter/%d/member/%d", chapterId, userId))

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

	chapterId := int(state.ChapterId.ValueInt64())
	userId := int(state.UserId.ValueInt64())

	chapterMember, err := dataminded_api.ReadChapterMember(r.Connection, chapterId, userId)

	if err != nil {
		logging.AddError(ctx, "Reading chapter_member failed", err)
		return
	}

	if !dataminded_api.ChapterMemberExists(chapterMember) {
		resp.State.RemoveResource(ctx)
		return
	}

	// Set the read values
	state.Role = types.StringValue(chapterMember.Role)

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

	chapterId := int(state.ChapterId.ValueInt64())
	userId := int(state.UserId.ValueInt64())
	newRole := plan.Role.ValueString()

	err := dataminded_api.UpdateChapterMember(r.Connection, chapterId, userId, newRole)

	if err != nil {
		logging.AddError(ctx, "Updating chapter member failed", err)
		return
	}

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

	chapterId := int(state.ChapterId.ValueInt64())
	userId := int(state.UserId.ValueInt64())

	err := dataminded_api.DeleteChapterMember(r.Connection, chapterId, userId)

	if err != nil {
		logging.AddError(ctx, "Dropping chapter_member failed", err)
	}
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
