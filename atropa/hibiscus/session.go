package hibiscus

import (
	"encoding/base64"

	"github.com/gorilla/sessions"
)

func SetSessionKey(secret string) error {
	key, err := base64.URLEncoding.WithPadding(base64.NoPadding).DecodeString(secret)
	if err != nil {
		return err
	}
	gl_cookie_store = sessions.NewCookieStore(key)
	return nil
}
