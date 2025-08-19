package web

import (
	"bytes"
	"encoding/base64"
	"encoding/gob"
	"fmt"
	"log/slog"
	"net/http"
	"os"
	"reflect"
	"strings"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
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

var (
	ErrorUserNotSignedIn = status.Error(codes.PermissionDenied, "not signed in")
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
		return "", err
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

// ----------------------------------------------------------------------------
func ToCode(s string) string {
	return strings.TrimSpace(strings.ToLower(s))
}
func ResourceType(o interface{}) string {
	it := reflect.TypeOf(o).Elem()
	return fmt.Sprintf("%s.%s", it.PkgPath(), it.Name())
}

// ----------------------------------------------------------------------------
func EnsureStopped() {
	name := ".stop"
	if _, err := os.Stat(name); err == nil {
		slog.Warn("stop file exists, will be exit...", slog.String("name", name))
		os.Exit(0)
	}
}
