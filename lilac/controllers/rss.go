package controllers

import (
	"fmt"
	"net/http"
	"time"

	"github.com/eduncan911/podcast"
	"github.com/gin-gonic/gin"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/i18n"
)

func RssByLang(db *gorm.DB, i18n *i18n.I18n) HandlerFunc {
	return func(c *gin.Context) error {
		// TODO
		home := "https://change-me.org"
		lang := "en"

		now := time.Now()
		rss := podcast.New("title", home, "description", &now, &now)

		rss.AddAuthor("who-am-i", "change@aaa.org")
		rss.AddAtomLink(fmt.Sprintf("%s/%s/rss.xml", home, lang))
		rss.AddImage("http://blablabla/i.jpg")
		rss.AddSummary(`link <a href="http://example.com">example.com</a>`)

		{
			item := podcast.Item{
				Title:       "Episode ",
				Link:        "http://example.com/1.mp3",
				Description: "Description for Episode ",
				PubDate:     &now,
			}
			item.AddImage("http://example.com/episode-1.png")
			item.AddSummary(`item <a href="http://example.com">example.com</a>`)
			item.AddEnclosure("http://e.com/1.mp3", podcast.MP3, 55*2)

			if _, err := rss.AddItem(item); err != nil {
				return err
			}
		}

		if err := rss.Encode(c.Writer); err != nil {
			return err
		}

		c.Header(CONTENT_TYPE_HEADER, ATOM_CONTENT_TYPE)
		c.Status(http.StatusOK)
		return nil
	}
}
