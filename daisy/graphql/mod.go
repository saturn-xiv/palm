package graphql

import (
	"crypto/rand"
	_ "embed"
	"encoding/base64"
	"encoding/gob"
	"net/http"

	"github.com/go-playground/validator/v10"
	graphql "github.com/graph-gophers/graphql-go"
	"github.com/graph-gophers/graphql-go/relay"
	"gorm.io/gorm"
)

var gl_validate = validator.New(validator.WithRequiredStructEnabled())

//go:embed schema.gql
var gl_schema_txt string

func Handler(db *gorm.DB, google_oauth2 GoogleOauth2Config) (http.Handler, error) {
	schema, err := graphql.ParseSchema(gl_schema_txt, &Root{db, google_oauth2})
	if err != nil {
		return nil, err
	}
	return &relay.Handler{Schema: schema}, nil
}

func random_alphanumeric(l int) (string, error) {
	buf := make([]byte, l)
	if _, err := rand.Read(buf); err != nil {
		return "", err
	}
	return base64.URLEncoding.EncodeToString(buf), nil
}

func init() {
	gob.Register(userProfile{})
}
