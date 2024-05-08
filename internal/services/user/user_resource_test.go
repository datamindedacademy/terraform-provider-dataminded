package user_test

import (
	"fmt"
	"testing"

	"terraform-provider-dataminded/internal/acceptance"
	"terraform-provider-dataminded/internal/dataminded_api"

	"github.com/hashicorp/terraform-plugin-testing/helper/resource"
)

type UserResource struct{}

func TestAccCreateUser(t *testing.T) {
	data := acceptance.BuildTestData(t)
	connection := dataminded_api.Connection{
		Host: data.Host,
		Port: data.Port,
	}
	r := UserResource{}

	resource.Test(t, resource.TestCase{
		Steps: []resource.TestStep{
			{
				Config:                   r.user_basic(connection, data.RandomString),
				ProtoV6ProviderFactories: acceptance.TestAccProtoV6ProviderFactories,
				Check: resource.ComposeTestCheckFunc(
					resource.TestCheckResourceAttr("dataminded_user.test", "name", fmt.Sprintf("test_%s", data.RandomString)),
				),
			},
		},
	})
}

func TestAccUpdateUser(t *testing.T) {
	data := acceptance.BuildTestData(t)
	connection := dataminded_api.Connection{
		Host: data.Host,
		Port: data.Port,
	}
	r := UserResource{}

	resource.Test(t, resource.TestCase{
		Steps: []resource.TestStep{
			{
				Config:                   r.user_basic(connection, data.RandomString),
				ProtoV6ProviderFactories: acceptance.TestAccProtoV6ProviderFactories,
			},
			{
				Config:                   r.user_basic(connection, fmt.Sprintf("%s2", data.RandomString)),
				ProtoV6ProviderFactories: acceptance.TestAccProtoV6ProviderFactories,
				Check: resource.ComposeTestCheckFunc(
					resource.TestCheckResourceAttr("dataminded_user.test", "name", fmt.Sprintf("test_%s2", data.RandomString)),
				),
			},
		},
	})
}

func (r UserResource) user_basic(connection dataminded_api.Connection, name string) string {
	template := r.template(connection)

	return fmt.Sprintf(
		`
		%[1]s

		resource "dataminded_user" "test" {
			name           = "test_%[2]s"
		}
		`, template, name)
}

func (r UserResource) template(connection dataminded_api.Connection) string {
	return fmt.Sprintf(`
		provider "dataminded" {
			host = "%s"
			port = %d
		}
	`, connection.Host, connection.Port)
}
