package dataminded_api

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
)

type User struct {
	Id   int64
	Name string
}

func ListUsers(connection Connection) ([]User, error) {
	response, err := http.Get(fmt.Sprintf("%s/user", baseUrl(connection)))
	if err != nil {
		return nil, err
	}

	responseData, err := io.ReadAll(response.Body)
	if err != nil {
		return nil, err
	}

	var users []User
	err = json.Unmarshal(responseData, &users)
	if err != nil {
		return nil, err
	}

	return users, nil
}
