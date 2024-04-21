package dataminded_api

import (
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
