package dataminded_api

import "fmt"

type Connection struct {
	Host string
	Port int64
}

func baseUrl(connection Connection) string {
	return fmt.Sprintf("%s:%d", connection.Host, connection.Port)
}
