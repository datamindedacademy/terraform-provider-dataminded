package chapter_member_test

import (
	"fmt"
	"testing"

	"terraform-provider-dataminded/internal/acceptance"
	"terraform-provider-dataminded/internal/dataminded_api"

	"github.com/hashicorp/terraform-plugin-testing/helper/resource"
)

type ChapterMemberResource struct{}

func TestAccCreateChapterMember(t *testing.T) {
	data := acceptance.BuildTestData(t)
	connection := dataminded_api.Connection{
		Host: data.Host,
		Port: data.Port,
	}
	r := ChapterMemberResource{}

	resource.Test(t, resource.TestCase{
		Steps: []resource.TestStep{
			{
				Config:                   r.chapter_member_basic(connection),
				ProtoV6ProviderFactories: acceptance.TestAccProtoV6ProviderFactories,
			},
		},
	})
}

func (r ChapterMemberResource) chapter_member_basic(connection dataminded_api.Connection) string {
	template := r.template(connection)

	return fmt.Sprintf(
		`
		%[1]s

		resource "dataminded_chapter_member" "test" {
		}
		`, template)
}

func (r ChapterMemberResource) template(connection dataminded_api.Connection) string {
	return fmt.Sprintf(`
		provider "dataminded" {
			host = "%s"
			port = %d
		}
	`, connection.Host, connection.Port)
}
