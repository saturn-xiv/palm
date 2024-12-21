package cms

import (
	"net/http"

	"github.com/gorilla/mux"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"github.com/saturn-xiv/palm/atropa/hibiscus"
)

func Mount(router *mux.Router, db *gorm.DB, jwt *crypto.Jwt) error {
	group := router.PathPrefix("/pages").Subrouter()

	group.HandleFunc("/", hibiscus.Wrap(IndexPage(db, jwt))).Methods(http.MethodGet)
	group.HandleFunc(`/archives-{year:[0-9]{4}}-{month:[0-9]{2}}`, hibiscus.Wrap(IndexPageByYearAndMonth(db, jwt))).Methods(http.MethodGet)
	group.HandleFunc(`/{slug:[\w]+}`, hibiscus.Wrap(ShowPageBySlug(db, jwt))).Methods(http.MethodGet)

	return nil
}
