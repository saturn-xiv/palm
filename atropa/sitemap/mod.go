package sitemap

import (
	"encoding/xml"
	"time"
)

// https://developers.google.com/search/docs/crawling-indexing/sitemaps/large-sitemaps
type SitemapIndex struct {
	XMLName xml.Name   `xml:"http://www.sitemaps.org/schemas/sitemap/0.9 sitemapindex"`
	Sitemap []*Sitemap `xml:"sitemap"`
}

type Sitemap struct {
	Loc string `xml:"loc"`
}

// https://developers.google.com/search/docs/crawling-indexing/sitemaps/build-sitemap

type UrlSet struct {
	XMLName xml.Name `xml:"http://www.sitemaps.org/schemas/sitemap/0.9 urlset"`
	Url     []*Url   `xml:"url"`
}

type Url struct {
	Loc     string     `xml:"loc"`
	LastMod *time.Time `xml:"lastmod"`
}
