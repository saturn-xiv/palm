package wechat_oauth2

import (
	"encoding/json"
	"errors"
	"io"
	"log/slog"
	"net/http"
)

type ErrorResponse struct {
	Code    int    `json:"errcode"`
	Message string `json:"errmsg"`
}

func CheckResponseError(action string, res *http.Response, val interface{}) error {
	if res.StatusCode != http.StatusOK {
		buf, err := io.ReadAll(res.Body)
		if err != nil {
			return err
		}
		body := string(buf)
		slog.Error(action, slog.Int("status", res.StatusCode), slog.String("body", body))
		return errors.New(body)
	}
	dec := json.NewDecoder(res.Body)
	{
		var it ErrorResponse
		if err := dec.Decode(&it); err == nil {
			if it.Code != 0 {
				slog.Error(action, slog.Int("code", it.Code), slog.String("message", it.Message))
				return errors.New(it.Message)
			}
		}
	}

	var it UserInfoResponse
	if err := dec.Decode(&it); err != nil {
		return err
	}
	return nil
}
