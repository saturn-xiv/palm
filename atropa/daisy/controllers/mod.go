package controllers

import (
	"net/http"

	"github.com/gorilla/mux"
	"google.golang.org/grpc"

	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"github.com/saturn-xiv/palm/atropa/hibiscus"
)

func Mount(router *mux.Router, jwt *crypto.Jwt, backend *grpc.ClientConn) error {
	{
		group := router.PathPrefix("/api/twilio").Subrouter()
		callback := TwilioSmsStatusCallback(jwt, backend)
		group.HandleFunc("/sms-status-callback/{token}", hibiscus.Wrap(callback)).Methods(http.MethodGet, http.MethodPost)
	}
	return nil
}
