package user

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
	_ resource.Resource              = &UserResource{}
	_ resource.ResourceWithConfigure = &UserResource{}
)

func NewUserResource() resource.Resource {
	return &UserResource{}
}

type UserResource struct {
	Connection dataminded_api.Connection
}

func (r *UserResource) Metadata(_ context.Context, req resource.MetadataRequest, resp *resource.MetadataResponse) {
	resp.TypeName = req.ProviderTypeName + "_user"
}

func (r *UserResource) Schema(_ context.Context, _ resource.SchemaRequest, resp *resource.SchemaResponse) {
	resp.Schema = schema.Schema{
		Description: fmt.Sprintf("Manage Data Minded users"),
		Attributes: map[string]schema.Attribute{
			"id": schema.Int64Attribute{
				Computed:    true,
				Description: "Id of the user in the sqlite database.",
			},
			"name": schema.StringAttribute{
				Required:    true,
				Description: "Name of the user",
			},
		},
	}
}

func (r *UserResource) Create(ctx context.Context, req resource.CreateRequest, resp *resource.CreateResponse) {
	ctx = logging.WithDiagnostics(ctx, &resp.Diagnostics)

	var plan UserResourceModel
	diags := req.Plan.Get(ctx, &plan)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	name := plan.Name.ValueString()

	user, err := dataminded_api.CreateUser(r.Connection, name)

	if err != nil {
		logging.AddError(ctx, "User creation failed", err)
		return
	}

	// User creation successfull --> Set state of computed variables (Id)
	plan.Id = types.Int64Value(int64(user.Id))

	diags = resp.State.Set(ctx, plan)
	resp.Diagnostics.Append(diags...)
}

func (r *UserResource) Read(ctx context.Context, req resource.ReadRequest, resp *resource.ReadResponse) {
	ctx = logging.WithDiagnostics(ctx, &resp.Diagnostics)

	var state UserResourceModel

	// Read input configured in data block
	resp.Diagnostics.Append(
		req.State.Get(ctx, &state)...,
	)

	if logging.HasError(ctx) {
		return
	}

	id := state.Id.ValueInt64()
	user, err := dataminded_api.ReadUser(r.Connection, int(id))

	if err != nil {
		logging.AddError(ctx, "Reading user failed", err)
		return
	}

	if !dataminded_api.UserExists(user) {
		resp.State.RemoveResource(ctx)
		return
	}

	// Set the read values
	// We don't have to set Id since this value was used to read
	state.Name = types.StringValue(user.Name)

	diags := resp.State.Set(ctx, &state)
	resp.Diagnostics.Append(diags...)
}

func (r *UserResource) Update(ctx context.Context, req resource.UpdateRequest, resp *resource.UpdateResponse) {
	ctx = logging.WithDiagnostics(ctx, &resp.Diagnostics)

	var state UserResourceModel
	resp.Diagnostics.Append(
		req.State.Get(ctx, &state)...,
	)

	var plan UserResourceModel
	resp.Diagnostics.Append(
		req.Plan.Get(ctx, &plan)...,
	)

	if resp.Diagnostics.HasError() {
		return
	}

	id := int(state.Id.ValueInt64())
	newName := plan.Name.ValueString()

	user, err := dataminded_api.UpdateUser(r.Connection, id, newName)

	if err != nil {
		logging.AddError(ctx, "Updating user failed", err)
		return
	}

	// User update successfull --> Set state of computed variables (Id)
	plan.Id = types.Int64Value(int64(user.Id))

	diags := resp.State.Set(ctx, plan)
	resp.Diagnostics.Append(diags...)
}

func (r *UserResource) Delete(ctx context.Context, req resource.DeleteRequest, resp *resource.DeleteResponse) {
	ctx = logging.WithDiagnostics(ctx, &resp.Diagnostics)

	var state UserResourceModel

	// Read input configured in data block
	resp.Diagnostics.Append(
		req.State.Get(ctx, &state)...,
	)

	if logging.HasError(ctx) {
		return
	}

	id := int(state.Id.ValueInt64())

	err := dataminded_api.DeleteUser(r.Connection, id)

	if err != nil {
		logging.AddError(ctx, "Dropping user failed", err)
	}
}

func (r *UserResource) Configure(_ context.Context, req resource.ConfigureRequest, resp *resource.ConfigureResponse) {

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
