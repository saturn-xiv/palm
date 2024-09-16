package controllers

import (
	"net/http"

	"github.com/gorilla/mux"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"github.com/saturn-xiv/palm/atropa/hibiscus"
)

func Mount(router *mux.Router, db *gorm.DB, jwt *crypto.Jwt) error {
	{
		group := router.PathPrefix("/api/twilio").Subrouter()
		callback := TwilioSmsStatusCallback(db, jwt)
		group.HandleFunc("/sms-status-callback/{token}", hibiscus.Wrap(callback)).Methods(http.MethodGet, http.MethodPost)
	}
	return nil
}
