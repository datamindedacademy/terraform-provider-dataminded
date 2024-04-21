package dataminded_api

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
)

type User struct {
	Id   int
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

func ReadUser(connection Connection, id int) (User, error) {
	response, err := http.Get(fmt.Sprintf("%s/user/%d", baseUrl(connection), id))
	if err != nil {
		return User{}, err
	}

	responseData, err := io.ReadAll(response.Body)
	if err != nil {
		return User{}, err
	}

	if string(responseData) == ERROR_USER_NOT_FOUND {
		return User{
			Id: -1,
		}, nil
	}

	var user User
	err = json.Unmarshal(responseData, &user)
	if err != nil {
		return User{}, err
	}

	return user, nil
}

func UpdateUser(connection Connection, id int, name string) (User, error) {
	body := []byte(fmt.Sprintf(`{
		"name": "%s"
	}`, name))

	request, err := http.NewRequest(http.MethodPut,
		fmt.Sprintf("%s/user/%d", baseUrl(connection), id),
		bytes.NewBuffer(body),
	)
	request.Header.Set("Content-Type", "application/json")

	if err != nil {
		return User{}, err
	}

	response, err := (&http.Client{}).Do(request)
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

func DeleteUser(connection Connection, id int) error {

	request, err := http.NewRequest(http.MethodDelete,
		fmt.Sprintf("%s/user/%d", baseUrl(connection), id),
		nil,
	)

	if err != nil {
		return err
	}

	response, err := (&http.Client{}).Do(request)
	if err != nil {
		return err
	}

	if response.StatusCode != 200 {
		return errors.New(fmt.Sprintf("Non 200 status code when deleting user %d", id))
	}

	return nil
}
