package dataminded_api_test

import (
	"fmt"
	"testing"

	"terraform-provider-dataminded/internal/acceptance"
	"terraform-provider-dataminded/internal/dataminded_api"

	"github.com/stretchr/testify/assert"
)

func TestCreateChapter(t *testing.T) {
	data := acceptance.BuildTestData(t)
	connection := dataminded_api.Connection{
		Host: data.Host,
		Port: data.Port,
	}

	chapter, err := dataminded_api.CreateChapter(connection, data.RandomString)

	if err != nil {
		t.Log(err)
	}
	assert.Nil(t, err)
	assert.Equal(t, data.RandomString, chapter.Name)

	// Test that the chapter is read correctly
	chapter, err = dataminded_api.ReadChapter(connection, chapter.Id)
	if err != nil {
		t.Log(err)
	}
	assert.Nil(t, err)
	assert.Equal(t, data.RandomString, chapter.Name)

	// Test that the chapter is in the list of chapters
	var chapters []dataminded_api.Chapter
	chapters, err = dataminded_api.ListChapters(connection)
	if err != nil {
		t.Log(err)
	}
	assert.Nil(t, err)

	found := false
	for _, chapter = range chapters {
		if chapter.Name == data.RandomString {
			found = true
			break
		}
	}
	assert.Equal(t, true, found, "Newly created chapter not found in list of chapters")
}

func TestReadNonExistentChapter(t *testing.T) {
	data := acceptance.BuildTestData(t)
	connection := dataminded_api.Connection{
		Host: data.Host,
		Port: data.Port,
	}

	t.Log(data.RandomInteger)
	chapter, err := dataminded_api.ReadChapter(connection, data.RandomInteger)

	if err != nil {
		t.Log(err)
	}
	assert.Nil(t, err)
	assert.Equal(t, chapter.Id, -1)
}

func TestListChapters(t *testing.T) {
	data := acceptance.BuildTestData(t)
	connection := dataminded_api.Connection{
		Host: data.Host,
		Port: data.Port,
	}

	_, err := dataminded_api.ListChapters(connection)

	if err != nil {
		t.Log(err)
	}
	assert.Nil(t, err)
}

func TestUpdateChapter(t *testing.T) {
	data := acceptance.BuildTestData(t)
	connection := dataminded_api.Connection{
		Host: data.Host,
		Port: data.Port,
	}

	originalName := data.RandomString
	newName := fmt.Sprintf("%s-new", originalName)

	chapter, err := dataminded_api.CreateChapter(connection, originalName)
	originalId := chapter.Id

	if err != nil {
		t.Log(err)
	}
	assert.Nil(t, err)
	assert.Equal(t, originalName, chapter.Name)

	chapter, err = dataminded_api.UpdateChapter(connection, originalId, newName)
	if err != nil {
		t.Log(err)
	}
	assert.Nil(t, err)
	assert.Equal(t, newName, chapter.Name)

	// check that if we read the originalId we obtain the new name
	chapter, err = dataminded_api.ReadChapter(connection, originalId)
	if err != nil {
		t.Log(err)
	}
	assert.Nil(t, err)
	assert.Equal(t, newName, chapter.Name)
}

func TestDeleteChapter(t *testing.T) {
	data := acceptance.BuildTestData(t)
	connection := dataminded_api.Connection{
		Host: data.Host,
		Port: data.Port,
	}

	chapter, err := dataminded_api.CreateChapter(connection, data.RandomString)

	if err != nil {
		t.Log(err)
	}
	assert.Nil(t, err)
	assert.Equal(t, data.RandomString, chapter.Name)

	err = dataminded_api.DeleteChapter(connection, chapter.Id)
	if err != nil {
		t.Log(err)
	}
	assert.Nil(t, err)

	// check that the chapter no longer exists
	chapter, err = dataminded_api.ReadChapter(connection, data.RandomInteger)

	if err != nil {
		t.Log(err)
	}
	assert.Nil(t, err)
	assert.Equal(t, chapter.Id, -1)
}
