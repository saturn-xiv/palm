package controllers

import (
	"net/http"
	"os"
	"time"

	"github.com/gorilla/handlers"
	"github.com/gorilla/mux"
	"gorm.io/gorm"

	daisy_controllers "github.com/saturn-xiv/palm/atropa/daisy/controllers"
	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"github.com/saturn-xiv/palm/atropa/env/redis"
	"github.com/saturn-xiv/palm/atropa/hibiscus"
)

func Mount(db *gorm.DB, cache *redis.Client, jwt *crypto.Jwt) (http.Handler, error) {
	router := mux.NewRouter()

	if err := daisy_controllers.Mount(router, db, jwt); err != nil {
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
