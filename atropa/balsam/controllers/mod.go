package controllers

import (
	"embed"
	"net/http"

	"github.com/gorilla/mux"
	"google.golang.org/grpc"

	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"github.com/saturn-xiv/palm/atropa/hibiscus"
)

//go:embed views/*
var gl_views_fs embed.FS

//go:embed templates/*
var gl_templates_fs embed.FS

func Mount(router *mux.Router, theme string, jwt *crypto.Jwt, backend *grpc.ClientConn) error {
	if err := hibiscus.LoadTemplates(&gl_views_fs, theme); err != nil {
		return err
	}

	{
		router.PathPrefix("/robots.txt").Handler(hibiscus.Wrap(RobotsTxt(gl_templates_fs))).Methods(http.MethodGet)
		router.PathPrefix("/").Handler(hibiscus.Wrap(Home())).Methods(http.MethodGet)
	}

	return nil
}
