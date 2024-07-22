package controllers

import (
	"net/http"

	"github.com/graphql-go/graphql"
	"github.com/graphql-go/handler"
)

func Graphql() (http.Handler, error) {
	query := graphql.ObjectConfig{
		Name: "Query",
		Fields: graphql.Fields{
			"localesByLang": &graphql.Field{
				Type: graphql.String,
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return "world", nil
				},
			},
		},
	}
	mutation := graphql.ObjectConfig{
		Name: "Mutation",
		Fields: graphql.Fields{
			"setLocale": &graphql.Field{
				Type: graphql.String,
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return "world", nil
				},
			},
		},
	}
	config := graphql.SchemaConfig{Query: graphql.NewObject(query), Mutation: graphql.NewObject(mutation)}
	schema, err := graphql.NewSchema(config)
	if err != nil {
		return nil, err
	}
	return handler.New(&handler.Config{
		Schema:   &schema,
		Pretty:   true,
		GraphiQL: true,
	}), nil

}
