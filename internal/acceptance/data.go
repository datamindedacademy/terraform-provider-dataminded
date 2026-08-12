package acceptance

import (
	"math/rand"
	"os"
	"strconv"
	"testing"
)

const (
	// charSetAlphaNum is the alphanumeric character set for use with randStringFromCharSet.
	charSetAlphaNum = "abcdefghijklmnopqrstuvwxyz012346789"
)

type TestData struct {
	Host string
	Port int64

	// RandomInteger is a random integer which is unique to this test case
	RandomInteger int

	// RandomString is a random 5 character string is unique to this test case
	RandomString string
}

func BuildTestData(t *testing.T) TestData {
	testData := TestData{
		Host:          "http://localhost",
		Port:          apiPort(),
		RandomInteger: rand.Intn(1000000) + 99999,
		RandomString:  randString(5),
	}
	return testData
}

// apiPort mirrors `make api PORT=...`, so that a machine with something else on
// 3000 can run the API and the acceptance tests against the same port.
func apiPort() int64 {
	if p, err := strconv.ParseInt(os.Getenv("PORT"), 10, 64); err == nil {
		return p
	}
	return 3000
}

// randString generates a random alphanumeric string of the length specified.
func randString(strlen int) string {
	return randStringFromCharSet(strlen, charSetAlphaNum)
}

// randStringFromCharSet generates a random string by selecting characters from
// the charset provided.
func randStringFromCharSet(strlen int, charSet string) string {
	result := make([]byte, strlen)
	for i := 0; i < strlen; i++ {
		result[i] = charSet[rand.Intn(len(charSet))]
	}
	return string(result)
}
