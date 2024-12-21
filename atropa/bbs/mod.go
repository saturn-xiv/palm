package bbs

import (
	"net/http"

	"github.com/gorilla/mux"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"github.com/saturn-xiv/palm/atropa/hibiscus"
)

func Mount(router *mux.Router, db *gorm.DB, jwt *crypto.Jwt) error {

	{
		group := router.PathPrefix("/forums").Subrouter()
		group.HandleFunc("/", hibiscus.Wrap(IndexForum(db, jwt))).Methods(http.MethodGet)
		group.HandleFunc(`/{slug:[\w]+}`, hibiscus.Wrap(ShowForumBySlug(db, jwt))).Methods(http.MethodGet)
	}
	{
		group := router.PathPrefix("/topics").Subrouter()
		group.HandleFunc(`/archives-{year:[0-9]{4}}-{month:[0-9]{2}}`, hibiscus.Wrap(IndexTopicByYearAndMonth(db, jwt))).Methods(http.MethodGet)
		group.HandleFunc(`/{slug:[\w]+}`, hibiscus.Wrap(ShowTopicBySlug(db, jwt))).Methods(http.MethodGet)
	}
	{
		group := router.PathPrefix("/posts").Subrouter()
		group.HandleFunc(`/archives-{year:[0-9]{4}}-{month:[0-9]{2}}`, hibiscus.Wrap(IndexPostsByYearAndMonth(db, jwt))).Methods(http.MethodGet)

	}
	return nil
}
