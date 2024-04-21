package dataminded_api

import (
	"testing"

	"github.com/hashicorp/terraform-provider-scaffolding-framework/internal/acceptance"
	"github.com/stretchr/testify/assert"
)

func TestListUsers(t *testing.T) {
	data := acceptance.BuildTestData(t)
	connection := Connection{
		Host: data.Host,
		Port: data.Port,
	}

	_, err := ListUsers(connection)

	assert.NotNil(t, err)
}
