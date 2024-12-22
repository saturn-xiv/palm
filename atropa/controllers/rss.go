package controllers

import (
	"fmt"
	"net/http"
	"time"

	"gorm.io/gorm"

	"github.com/google/uuid"
	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"github.com/saturn-xiv/palm/atropa/hibiscus"
	"github.com/saturn-xiv/palm/atropa/rss"
)

func RssByLang(db *gorm.DB, jwt *crypto.Jwt) hibiscus.HandlerFunc {
	return func(c *hibiscus.Context) {
		vars := c.Vars()
		host := c.Host()
		lang := vars["lang"]
		feed := rss.New()
		now := time.Now()
		// TODO
		{
			feed.Channel.Item = append(feed.Channel.Item, &rss.Item{
				Title:       "aaa",
				Description: "ddd",
				Link:        fmt.Sprintf("https://%s/%s/posts/%s", host, lang, "sss"),
				Guid: &rss.Guid{
					IsPermaLink: true,
					Text:        uuid.New().String(),
				},
				PubDate: &now,
			})

		}
		c.XML(http.StatusOK, feed)
	}
}
