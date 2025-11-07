package graphql

import (
	_ "embed"
	"net/http"

	graphql "github.com/graph-gophers/graphql-go"
	"github.com/graph-gophers/graphql-go/relay"
)

//go:embed schema.txt
var gl_schema_txt string

func Handler() (http.Handler, error) {
	schema, err := graphql.ParseSchema(gl_schema_txt, &Root{})
	if err != nil {
		return nil, err
	}
	return &relay.Handler{Schema: schema}, nil
}
