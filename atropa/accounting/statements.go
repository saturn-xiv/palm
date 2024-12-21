package accounting

import (
	"net/http"

	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"github.com/saturn-xiv/palm/atropa/hibiscus"
)

func ShowStatementByYearAndMonthAndDay(db *gorm.DB, jwt *crypto.Jwt) hibiscus.HandlerFunc {
	return func(c *hibiscus.Context) {
		// TODO
		c.HTML(http.StatusOK, "", hibiscus.H{})
	}
}

func ShowStatementByYearAndMonth(db *gorm.DB, jwt *crypto.Jwt) hibiscus.HandlerFunc {
	return func(c *hibiscus.Context) {
		// TODO
		c.HTML(http.StatusOK, "", hibiscus.H{})
	}
}

func ShowStatementByYear(db *gorm.DB, jwt *crypto.Jwt) hibiscus.HandlerFunc {
	return func(c *hibiscus.Context) {
		// TODO
		c.HTML(http.StatusOK, "", hibiscus.H{})
	}
}
