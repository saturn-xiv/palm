package controllers

import (
	"embed"
	"net/http"
	"os"
	"time"

	"github.com/gorilla/handlers"
	"github.com/gorilla/mux"
	"google.golang.org/grpc"

	balsam_controllers "github.com/saturn-xiv/palm/atropa/balsam/controllers"
	daisy_controllers "github.com/saturn-xiv/palm/atropa/daisy/controllers"
	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"github.com/saturn-xiv/palm/atropa/env/redis"
	"github.com/saturn-xiv/palm/atropa/hibiscus"
)

//go:embed assets/* themes/jekyll-al-folio/assets/* themes/hugo-even/static/* themes/hugo-universal/static/*
var gl_assets_fs embed.FS

func Mount(theme string, cache *redis.Client, jwt *crypto.Jwt, backend *grpc.ClientConn) (http.Handler, error) {
	router := mux.NewRouter()
	{
		hibiscus.Static(router, "/3rd/", "./node_modules")
		hibiscus.StaticFS(router, "/public/", gl_assets_fs)
	}
	{
		handler, err := Graphql(jwt, backend)
		if err != nil {
			return nil, err
		}
		router.PathPrefix("/graphql").Handler(handler).Methods(http.MethodGet, http.MethodPost, http.MethodHead)
	}
	if err := daisy_controllers.Mount(router, jwt, backend); err != nil {
		return nil, err
	}
	if err := balsam_controllers.Mount(router, theme, jwt, backend); err != nil {
		return nil, err
	}

	handler := handlers.CORS(
		handlers.MaxAge(int(time.Minute*3)),
		handlers.AllowedHeaders([]string{hibiscus.HTTP_COOKIE_HEADER, hibiscus.HTTP_AUTHORIZATION_HEADER, hibiscus.HTTP_FORWARDED_FOR_HEADER}),
		handlers.AllowCredentials(),
		handlers.AllowedMethods([]string{http.MethodGet, http.MethodPost, http.MethodHead, http.MethodPut, http.MethodPatch, http.MethodDelete}),
	)(router)
	handler = handlers.CombinedLoggingHandler(os.Stdout, handler)
	handler = handlers.RecoveryHandler()(handler)

	return handler, nil
}
