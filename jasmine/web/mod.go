package web

import (
	"bytes"
	"encoding/base64"
	"encoding/gob"
	"net/http"

	"google.golang.org/protobuf/proto"
)

const (
	ContentTypeJSON = "application/json; charset=utf-8"
	ContentTypeXML  = "application/xml"
	ContentTypeHTML = "text/html; charset=utf-8"
	ContentTypeText = "text/plain; charset=utf-8"
)

const (
	HeaderContentType   = "Content-Type"
	HeaderAuthorization = "Authorization"
	BearerTokenPrefix   = "Bearer "
)

type H map[string]interface{}

type JsonHttpHandler func(r *http.Request) (interface{}, error)
type XmlHttpHandler func(r *http.Request) (interface{}, error)
type PlainHttpHandler func(r *http.Request) (string, interface{}, error)
type HtmlHttpHandler func(r *http.Request) (string, interface{}, error)

// ----------------------------------------------------------------------------

func ProtoBufMessageToString(m proto.Message) (string, error) {
	out, err := proto.Marshal(m)
	if err != nil {
		return "", err
	}
	return base64.RawURLEncoding.EncodeToString(out), nil
}

func ProtoBufMessageFromString(s string, m proto.Message) error {
	buf, err := base64.RawURLEncoding.DecodeString(s)
	if err != nil {
		return err
	}
	return proto.Unmarshal(buf, m)
}

// ----------------------------------------------------------------------------
func ToString(v interface{}) (string, error) {
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	if err := enc.Encode(v); err != nil {
		return "", nil
	}
	return base64.RawURLEncoding.EncodeToString(buf.Bytes()), nil
}

func FromString(s string, v interface{}) error {
	tmp, err := base64.RawURLEncoding.DecodeString(s)
	if err != nil {
		return err
	}
	buf := bytes.NewBuffer(tmp)
	dec := gob.NewDecoder(buf)
	return dec.Decode(v)
}
