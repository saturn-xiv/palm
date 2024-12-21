package cms

import (
	"errors"
	"log/slog"
	"net/http"
	"strconv"

	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"github.com/saturn-xiv/palm/atropa/hibiscus"
)

func ShowPageBySlug(db *gorm.DB, jwt *crypto.Jwt) hibiscus.HandlerFunc {
	return func(c *hibiscus.Context) {
		vars := c.Vars()
		slug, ok := vars["slug"]
		if !ok {
			c.Abort(http.StatusNotFound, errors.New("empty slug"))
			return
		}
		slog.Debug("show page by slug", slog.String("slug", slug))
		// TODO
		c.HTML(http.StatusOK, "", hibiscus.H{})
	}
}

func IndexPageByYearAndMonth(db *gorm.DB, jwt *crypto.Jwt) hibiscus.HandlerFunc {
	return func(c *hibiscus.Context) {
		vars := c.Vars()
		year, err := strconv.Atoi(vars["year"])
		if err != nil {
			c.Abort(http.StatusBadRequest, err)
			return
		}
		month, err := strconv.Atoi(vars["month"])
		if err != nil {
			c.Abort(http.StatusBadRequest, err)
			return
		}
		slog.Debug("index pages", slog.Int("year", year), slog.Int("month", month))
		// TODO
		c.HTML(http.StatusOK, "", hibiscus.H{
			"year":  year,
			"month": month,
		})
	}
}

func IndexPage(db *gorm.DB, jwt *crypto.Jwt) hibiscus.HandlerFunc {
	return func(c *hibiscus.Context) {
		slog.Debug("index pages")
		// TODO
		c.HTML(http.StatusOK, "", hibiscus.H{})
	}
}
