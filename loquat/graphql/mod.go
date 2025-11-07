package graphql

import (
	_ "embed"
	"net/http"

	"github.com/go-playground/validator/v10"
	graphql "github.com/graph-gophers/graphql-go"
	"github.com/graph-gophers/graphql-go/relay"
	"gorm.io/gorm"
)

var (
	ContentType   = "Content-Type"
	Authorization = "Authorization"
	Bearer        = "Bearer "
)

var gl_validate = validator.New()

//go:embed schema.txt
var gl_schema_txt string

func Handler(db *gorm.DB) (http.Handler, error) {
	schema, err := graphql.ParseSchema(gl_schema_txt, &Root{db: db})
	if err != nil {
		return nil, err
	}
	return &relay.Handler{Schema: schema}, nil
}

type Mutation struct {
	db *gorm.DB
}

type Query struct {
	db *gorm.DB
}

func (p *Query) Version() string { return "2025.11.6" }

type Root struct {
	db *gorm.DB
}

func (p *Root) Query() *Query {
	return &Query{db: p.db}
}

func (p *Root) Mutation() *Mutation {
	return &Mutation{db: p.db}
}
