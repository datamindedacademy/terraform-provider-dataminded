package dataminded_api

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
)

type Chapter struct {
	Id   int
	Name string
}

func ListChapters(connection Connection) ([]Chapter, error) {
	response, err := http.Get(fmt.Sprintf("%s/chapter", baseUrl(connection)))
	if err != nil {
		return nil, err
	}

	responseData, err := io.ReadAll(response.Body)
	if err != nil {
		return nil, err
	}

	var chapters []Chapter
	err = json.Unmarshal(responseData, &chapters)
	if err != nil {
		return nil, err
	}

	return chapters, nil
}

func CreateChapter(connection Connection, name string) (Chapter, error) {
	body := []byte(fmt.Sprintf(`{
		"name": "%s"
	}`, name))

	response, err := http.Post(
		fmt.Sprintf("%s/chapter", baseUrl(connection)),
		"application/json",
		bytes.NewBuffer(body),
	)

	if err != nil {
		return Chapter{}, err
	}

	responseData, err := io.ReadAll(response.Body)
	if err != nil {
		return Chapter{}, err
	}

	var chapter Chapter
	err = json.Unmarshal(responseData, &chapter)
	if err != nil {
		return Chapter{}, err
	}

	return chapter, nil
}

func ReadChapter(connection Connection, id int) (Chapter, error) {
	response, err := http.Get(fmt.Sprintf("%s/chapter/%d", baseUrl(connection), id))
	if err != nil {
		return Chapter{}, err
	}

	responseData, err := io.ReadAll(response.Body)
	if err != nil {
		return Chapter{}, err
	}

	if string(responseData) == ERROR_USER_NOT_FOUND {
		return Chapter{
			Id: -1,
		}, nil
	}

	var chapter Chapter
	err = json.Unmarshal(responseData, &chapter)
	if err != nil {
		return Chapter{}, err
	}

	return chapter, nil
}

func UpdateChapter(connection Connection, id int, name string) (Chapter, error) {
	body := []byte(fmt.Sprintf(`{
		"name": "%s"
	}`, name))

	request, err := http.NewRequest(http.MethodPut,
		fmt.Sprintf("%s/chapter/%d", baseUrl(connection), id),
		bytes.NewBuffer(body),
	)
	request.Header.Set("Content-Type", "application/json")

	if err != nil {
		return Chapter{}, err
	}

	response, err := (&http.Client{}).Do(request)
	if err != nil {
		return Chapter{}, err
	}

	responseData, err := io.ReadAll(response.Body)
	if err != nil {
		return Chapter{}, err
	}

	var chapter Chapter
	err = json.Unmarshal(responseData, &chapter)
	if err != nil {
		return Chapter{}, err
	}

	return chapter, nil
}

func DeleteChapter(connection Connection, id int) error {

	request, err := http.NewRequest(http.MethodDelete,
		fmt.Sprintf("%s/chapter/%d", baseUrl(connection), id),
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
		return errors.New(fmt.Sprintf("Non 200 status code when deleting chapter %d", id))
	}

	return nil
}

func ChapterExists(chapter Chapter) bool {
	return chapter.Id != -1
}
