package questionnaires

import (
	"net/http"

	"github.com/gorilla/mux"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"github.com/saturn-xiv/palm/atropa/hibiscus"
)

func Mount(router *mux.Router, db *gorm.DB, jwt *crypto.Jwt) error {
	root := router.PathPrefix("/questionnaires").Subrouter()
	{
		group := root.PathPrefix("/form").Subrouter()
		group.HandleFunc("/{uid}", hibiscus.Wrap(GatherForm(db, jwt))).Methods(http.MethodPost)
		group.HandleFunc("/{token}", hibiscus.Wrap(ShowForm(db, jwt))).Methods(http.MethodGet)
	}
	return nil
}
