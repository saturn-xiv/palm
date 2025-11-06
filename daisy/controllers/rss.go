package controllers

import (
	"time"

	"github.com/gorilla/feeds"
)

func Rss(ctx *Context, req *Session) (string, error) {
	// TODO
	now := time.Now()
	feed := &feeds.Feed{
		Title:       "ttt",
		Link:        &feeds.Link{Href: "http://change-me.org/blog"},
		Description: "",
		Author:      &feeds.Author{Name: "Aaa", Email: "aaa@aaa.com"},
		Created:     now,
	}

	feed.Items = []*feeds.Item{
		&feeds.Item{
			Title:       "Daisy",
			Link:        &feeds.Link{Href: "http://change-me.org/blog/daisy/"},
			Description: "Follower",
			Author:      &feeds.Author{Name: "Bbb", Email: "bbb@bbb.com"},
			Created:     now,
		},
	}

	return feed.ToRss()
}
