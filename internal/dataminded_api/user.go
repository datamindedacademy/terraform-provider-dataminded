package dataminded_api

import (
	"bytes"
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

func CreateUser(connection Connection, name string) (User, error) {
	body := []byte(fmt.Sprintf(`{
		"name": "%s"
	}`, name))

	response, err := http.Post(
		fmt.Sprintf("%s/user", baseUrl(connection)),
		"application/json",
		bytes.NewBuffer(body),
	)

	if err != nil {
		return User{}, err
	}

	responseData, err := io.ReadAll(response.Body)
	if err != nil {
		return User{}, err
	}

	var user User
	err = json.Unmarshal(responseData, &user)
	if err != nil {
		return User{}, err
	}

	return user, nil
}
