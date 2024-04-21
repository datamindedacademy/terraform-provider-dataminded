package dataminded_api

import (
	"fmt"
	"testing"

	"github.com/hashicorp/terraform-provider-scaffolding-framework/internal/acceptance"
	"github.com/stretchr/testify/assert"
)

func TestCreateUser(t *testing.T) {
	data := acceptance.BuildTestData(t)
	connection := Connection{
		Host: data.Host,
		Port: data.Port,
	}

	user, err := CreateUser(connection, data.RandomString)

	if err != nil {
		t.Log(err)
	}
	assert.Nil(t, err)
	assert.Equal(t, data.RandomString, user.Name)

	// Test that the user is read correctly
	user, err = ReadUser(connection, user.Id)
	if err != nil {
		t.Log(err)
	}
	assert.Nil(t, err)
	assert.Equal(t, data.RandomString, user.Name)

	// Test that the user is in the list of users
	var users []User
	users, err = ListUsers(connection)
	if err != nil {
		t.Log(err)
	}
	assert.Nil(t, err)

	found := false
	for _, user = range users {
		if user.Name == data.RandomString {
			found = true
			break
		}
	}
	assert.Equal(t, true, found, "Newly created user not found in list of users")
}

func TestReadNonExistentUser(t *testing.T) {
	data := acceptance.BuildTestData(t)
	connection := Connection{
		Host: data.Host,
		Port: data.Port,
	}

	t.Log(data.RandomInteger)
	user, err := ReadUser(connection, data.RandomInteger)

	if err != nil {
		t.Log(err)
	}
	assert.Nil(t, err)
	assert.Equal(t, user.Id, -1)
}

func TestListUsers(t *testing.T) {
	data := acceptance.BuildTestData(t)
	connection := Connection{
		Host: data.Host,
		Port: data.Port,
	}

	_, err := ListUsers(connection)

	if err != nil {
		t.Log(err)
	}
	assert.Nil(t, err)
}

func TestUpdateUser(t *testing.T) {
	data := acceptance.BuildTestData(t)
	connection := Connection{
		Host: data.Host,
		Port: data.Port,
	}

	originalName := data.RandomString
	newName := fmt.Sprintf("%s-new", originalName)

	user, err := CreateUser(connection, originalName)
	originalId := user.Id

	if err != nil {
		t.Log(err)
	}
	assert.Nil(t, err)
	assert.Equal(t, originalName, user.Name)

	user, err = UpdateUser(connection, originalId, newName)
	if err != nil {
		t.Log(err)
	}
	assert.Nil(t, err)
	assert.Equal(t, newName, user.Name)

	// check that if we read the originalId we obtain the new name
	user, err = ReadUser(connection, originalId)
	if err != nil {
		t.Log(err)
	}
	assert.Nil(t, err)
	assert.Equal(t, newName, user.Name)
}

func TestDeleteUser(t *testing.T) {
	data := acceptance.BuildTestData(t)
	connection := Connection{
		Host: data.Host,
		Port: data.Port,
	}

	user, err := CreateUser(connection, data.RandomString)

	if err != nil {
		t.Log(err)
	}
	assert.Nil(t, err)
	assert.Equal(t, data.RandomString, user.Name)

	err = DeleteUser(connection, user.Id)
	if err != nil {
		t.Log(err)
	}
	assert.Nil(t, err)

	// check that the user no longer exists
	user, err = ReadUser(connection, data.RandomInteger)

	if err != nil {
		t.Log(err)
	}
	assert.Nil(t, err)
	assert.Equal(t, user.Id, -1)
}
