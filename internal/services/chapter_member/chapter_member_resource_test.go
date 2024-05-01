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
				Config:                   r.chapter_member_basic(connection, data.RandomString),
				ProtoV6ProviderFactories: acceptance.TestAccProtoV6ProviderFactories,
				Check: resource.ComposeTestCheckFunc(
					resource.TestCheckResourceAttr("dataminded_chapter_member.test", "role", "Contributor"),
				),
			},
		},
	})
}

func (r ChapterMemberResource) chapter_member_basic(connection dataminded_api.Connection, name string) string {
	template := r.template(connection, name)

	return fmt.Sprintf(
		`
		%[1]s

		resource "dataminded_chapter_member" "test" {
			chapter = dataminded_chapter.test.id
			member = dataminded_user.test.id
		}
		`, template)
}

func (r ChapterMemberResource) template(connection dataminded_api.Connection, name string) string {
	return fmt.Sprintf(`
		provider "dataminded" {
			host = "%[1]s"
			port = %[2]d
		}

		resource "dataminded_user" "test" {
			name           = "user_%[3]s"
		}

		resource "dataminded_chapter" "test" {
			name           = "chapter_%[3]s"
		}
	`, connection.Host, connection.Port, name)
}
