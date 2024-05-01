package dataminded_api_test

import (
	"testing"

	"terraform-provider-dataminded/internal/acceptance"
	"terraform-provider-dataminded/internal/dataminded_api"

	"github.com/stretchr/testify/assert"
)

func TestCreateChapterMember(t *testing.T) {
	data := acceptance.BuildTestData(t)
	connection := dataminded_api.Connection{
		Host: data.Host,
		Port: data.Port,
	}

	user, err := dataminded_api.CreateUser(connection, data.RandomString)
	assert.Nil(t, err)

	chapter, err := dataminded_api.CreateChapter(connection, data.RandomString)
	assert.Nil(t, err)

	role := "Lead"
	err = dataminded_api.CreateChapterMember(connection, chapter.Id, user.Id, role)
	assert.Nil(t, err)

	// Test that the chapter member is read correctly
	member, err := dataminded_api.ReadChapterMember(connection, chapter.Id, user.Id)
	assert.Nil(t, err)

	assert.Equal(t, role, member.Role)
}

func TestReadNonExistentChapterMember(t *testing.T) {
	data := acceptance.BuildTestData(t)
	connection := dataminded_api.Connection{
		Host: data.Host,
		Port: data.Port,
	}

	t.Log(data.RandomInteger)
	chapterMember, err := dataminded_api.ReadChapterMember(connection, data.RandomInteger, data.RandomInteger)

	assert.Nil(t, err)
	assert.Equal(t, chapterMember.ChapterId, -1)
}

func TestUpdateChapterMember(t *testing.T) {
	data := acceptance.BuildTestData(t)
	connection := dataminded_api.Connection{
		Host: data.Host,
		Port: data.Port,
	}

	user, err := dataminded_api.CreateUser(connection, data.RandomString)
	assert.Nil(t, err)

	chapter, err := dataminded_api.CreateChapter(connection, data.RandomString)
	assert.Nil(t, err)

	initialRole := "Lead"
	newRole := "Contributor"
	err = dataminded_api.CreateChapterMember(connection, chapter.Id, user.Id, initialRole)
	assert.Nil(t, err)

	err = dataminded_api.UpdateChapterMember(connection, chapter.Id, user.Id, newRole)
	assert.Nil(t, err)

	member, err := dataminded_api.ReadChapterMember(connection, chapter.Id, user.Id)
	assert.Nil(t, err)
	assert.Equal(t, newRole, member.Role)
}

func TestDeleteChapterMember(t *testing.T) {
	data := acceptance.BuildTestData(t)
	connection := dataminded_api.Connection{
		Host: data.Host,
		Port: data.Port,
	}

	user, err := dataminded_api.CreateUser(connection, data.RandomString)
	assert.Nil(t, err)

	chapter, err := dataminded_api.CreateChapter(connection, data.RandomString)
	assert.Nil(t, err)

	err = dataminded_api.CreateChapterMember(connection, chapter.Id, user.Id, "Contributor")
	assert.Nil(t, err)

	err = dataminded_api.DeleteChapterMember(connection, chapter.Id, user.Id)
	assert.Nil(t, err)

	// check that the chapter no longer exists
	member, err := dataminded_api.ReadChapterMember(connection, chapter.Id, user.Id)

	assert.Nil(t, err)
	assert.Equal(t, member.ChapterId, -1)
}
