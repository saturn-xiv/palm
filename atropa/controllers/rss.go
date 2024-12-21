package controllers

import (
	"net/http"

	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"github.com/saturn-xiv/palm/atropa/hibiscus"
)

func RssByLang(db *gorm.DB, jwt *crypto.Jwt) hibiscus.HandlerFunc {
	return func(c *hibiscus.Context) {
		// TODO
		c.HTML(http.StatusOK, "home", hibiscus.H{})
	}
}
