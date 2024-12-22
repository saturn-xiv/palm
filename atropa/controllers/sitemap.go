package controllers

import (
	"fmt"
	"net/http"
	"time"

	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"github.com/saturn-xiv/palm/atropa/hibiscus"
	"github.com/saturn-xiv/palm/atropa/sitemap"
)

func SitemapXml(db *gorm.DB, jwt *crypto.Jwt) hibiscus.HandlerFunc {
	return func(c *hibiscus.Context) {
		host := c.Host()
		root := sitemap.SitemapIndex{
			Sitemap: []*sitemap.Sitemap{},
		}
		// TODO
		for _, lang := range []string{"en-US", "zh-Hans"} {
			root.Sitemap = append(root.Sitemap, &sitemap.Sitemap{
				Loc: fmt.Sprintf("https://%s/%s/sitemap.xml", host, lang),
			})
		}

		c.XML(http.StatusOK, root)
	}
}

func SitemapXmlByLang(db *gorm.DB, jwt *crypto.Jwt) hibiscus.HandlerFunc {
	return func(c *hibiscus.Context) {
		vars := c.Vars()
		host := c.Host()
		lang := vars["lang"]
		root := sitemap.UrlSet{
			Url: []*sitemap.Url{},
		}
		now := time.Now()
		// TODO

		{
			root.Url = append(root.Url, &sitemap.Url{
				Loc:     fmt.Sprintf("https://%s/%s/posts/", host, lang),
				LastMod: &now,
			})
			root.Url = append(root.Url, &sitemap.Url{
				Loc:     fmt.Sprintf("https://%s/%s/", host, lang),
				LastMod: &now,
			})
		}
		c.XML(http.StatusOK, root)
	}
}
