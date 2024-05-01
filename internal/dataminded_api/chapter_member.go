package dataminded_api

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
)

type ChapterMember struct {
	ChapterId int
	UserId    int
	Role      string
}

func ReadChapterMember(connection Connection, chapterId int, userId int) (ChapterMember, error) {
	response, err := http.Get(fmt.Sprintf("%s/chapter/%d/member/%d", baseUrl(connection), chapterId, userId))
	if err != nil {
		return ChapterMember{}, err
	}

	responseData, err := io.ReadAll(response.Body)
	if err != nil {
		return ChapterMember{}, err
	}

	if string(responseData) == ERROR_CHAPTER_MEMBER_NOT_FOUND {
		return ChapterMember{
			UserId:    -1,
			ChapterId: -1,
		}, nil
	}

	var member ChapterMember
	err = json.Unmarshal(responseData, &member)
	if err != nil {
		return ChapterMember{}, err
	}

	return member, nil
}

func CreateChapterMember(connection Connection, chapterId int, userId int, role string) error {

	body := []byte(fmt.Sprintf(`{
			"role": "%s"
		}`, role))

	response, err := http.Post(
		fmt.Sprintf("%s/chapter/%d/member/%d", baseUrl(connection), chapterId, userId),
		"application/json",
		bytes.NewBuffer(body),
	)

	if err != nil {
		return err
	}

	responseData, err := io.ReadAll(response.Body)
	if err != nil {
		return err
	}

	if response.StatusCode >= 400 {
		return errors.New(fmt.Sprintf("Non 200 status code when creating chapter member %d/%d. Detailed error: %s", chapterId, userId, string(responseData)))
	}

	return nil
}

func UpdateChapterMember(connection Connection, chapterId int, userId int, role string) error {
	body := []byte(fmt.Sprintf(`{
		"role": "%s"
	}`, role))

	request, err := http.NewRequest(http.MethodPut,
		fmt.Sprintf("%s/chapter/%d/member/%d", baseUrl(connection), chapterId, userId),
		bytes.NewBuffer(body),
	)
	request.Header.Set("Content-Type", "application/json")

	if err != nil {
		return err
	}

	response, err := (&http.Client{}).Do(request)
	if err != nil {
		return err
	}

	responseData, err := io.ReadAll(response.Body)
	if err != nil {
		return err
	}

	if response.StatusCode >= 400 {
		return errors.New(fmt.Sprintf("Non 200 status code when updating chapter member %d/%d. Detailed error: %s", chapterId, userId, string(responseData)))
	}

	return nil
}

func DeleteChapterMember(connection Connection, chapterId int, userId int) error {

	request, err := http.NewRequest(http.MethodDelete,
		fmt.Sprintf("%s/chapter/%d/member/%d", baseUrl(connection), chapterId, userId),
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
		return errors.New(fmt.Sprintf("Non 200 status code when deleting chapter member %d/%d", chapterId, userId))
	}

	return nil
}

func ChapterMemberExists(member ChapterMember) bool {
	return member.UserId != -1
}
