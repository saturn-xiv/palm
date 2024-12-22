package rss

import (
	"encoding/xml"
	"time"
)

// https://www.rssboard.org/rss-specification
// https://en.wikipedia.org/wiki/RSS
type Feed struct {
	Version string   `xml:"version,attr"`
	XMLName xml.Name `xml:"rss"`
	Channel *Channel `xml:"channel"`
}

func New() *Feed {
	return &Feed{
		Version: "2.0",
		Channel: &Channel{
			Ttl: uint(time.Hour.Seconds()) * 8,
		},
	}
}

type Channel struct {
	Title         string     `xml:"title"`
	Description   string     `xml:"description"`
	Link          string     `xml:"link"`
	Copyright     string     `xml:"copyright"`
	LastBuildDate *time.Time `xml:"lastBuildDate"`
	PubDate       *time.Time `xml:"pubDate"`
	Ttl           uint       `xml:"ttl"`
	Item          []*Item    `xml:"item"`
}

type Item struct {
	Title       string     `xml:"title"`
	Description string     `xml:"description"`
	Link        string     `xml:"link"`
	Guid        *Guid      `xml:"guid"`
	PubDate     *time.Time `xml:"pubDate"`
}

type Guid struct {
	IsPermaLink bool   `xml:"isPermaLink,attr"`
	Text        string `xml:",chardata"`
}
