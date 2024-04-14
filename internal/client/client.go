package client

import (
	http "net/http"
)

type Client struct {
	httpClient *http.Client
}
