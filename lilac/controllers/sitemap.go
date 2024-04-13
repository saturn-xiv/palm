package controllers

import (
	"io"
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/sabloger/sitemap-generator/smg"
	"gorm.io/gorm"
)

func SiteMap(db *gorm.DB) gin.HandlerFunc {
	return func(c *gin.Context) {
		if err := sitemap_index(c.Writer, db, "https://change-me"); err != nil {
			c.AbortWithError(http.StatusInternalServerError, err)
			return
		}
		c.Header(CONTENT_TYPE_HEADER, XML_CONTENT_TYPE)
		c.Status(http.StatusOK)
	}
}

func sitemap_index(writer io.Writer, _db *gorm.DB, home string) error {
	now := time.Now()
	sm := smg.NewSitemapIndex(true)
	sm.SetCompress(false)
	// TODO
	sm.SetHostname(home)
	sm.Add(&smg.SitemapIndexLoc{
		Loc:     "news/2021-01-05/a-news-page",
		LastMod: &now,
	})

	_, err := sm.WriteTo(writer)
	return err
}
func SiteMapByLang(db *gorm.DB) gin.HandlerFunc {
	return func(c *gin.Context) {

		if err := sitemap_by_lang(c.Writer, db, "https://change-me", "en-us"); err != nil {
			c.AbortWithError(http.StatusInternalServerError, err)
			return
		}
		c.Header(CONTENT_TYPE_HEADER, XML_CONTENT_TYPE)
		c.Status(http.StatusOK)
	}
}

func sitemap_by_lang(writer io.Writer, _db *gorm.DB, home string, _lang string) error {
	now := time.Now()
	sm := smg.NewSitemap(true)
	sm.SetCompress(false)
	// TODO
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

	_, err := sm.WriteTo(writer)
	return err
}
