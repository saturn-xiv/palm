package controllers

import (
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/sabloger/sitemap-generator/smg"
	"gorm.io/gorm"
)

func SiteMap(db *gorm.DB) HandlerFunc {
	return func(c *gin.Context) error {
		home := "https://www.change-me.org"

		now := time.Now()
		sm := smg.NewSitemapIndex(true)
		sm.SetCompress(false)
		// TODO
		sm.SetHostname(home)
		sm.Add(&smg.SitemapIndexLoc{
			Loc:     "news/2021-01-05/a-news-page",
			LastMod: &now,
		})

		if _, err := sm.WriteTo(c.Writer); err != nil {
			return err
		}
		c.Header(CONTENT_TYPE_HEADER, XML_CONTENT_TYPE)
		c.Status(http.StatusOK)
		return nil
	}
}

func SiteMapByLang(db *gorm.DB) HandlerFunc {
	return func(c *gin.Context) error {
		// TODO
		home := "https://www.change-me.org"

		now := time.Now()
		sm := smg.NewSitemap(true)
		sm.SetCompress(false)
		sm.SetHostname(home)
		if err := sm.Add(&smg.SitemapLoc{
			Loc:        "news/2021-01-05/a-news-page",
			LastMod:    &now,
			ChangeFreq: smg.Weekly,
			Priority:   1,
		}); err != nil {
			return err
		}
		sm.SetLastMod(&now)
		sm.Finalize()

		if _, err := sm.WriteTo(c.Writer); err != nil {
			return err
		}
		c.Header(CONTENT_TYPE_HEADER, XML_CONTENT_TYPE)
		c.Status(http.StatusOK)
		return nil
	}
}
