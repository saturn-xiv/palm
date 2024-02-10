package seo

import "github.com/gin-gonic/gin"

func Register(router gin.IRouter) {
	router.GET("/", Home())
	router.GET("/robots.txt", RobotsTxt())
	router.GET("/sitemap.xml", SitemapXml())
	router.GET("/:lang/sitemap.xml", SitemapXmlByLang())
	router.GET("/:lang/rss.xml", RssXml())
	router.GET("/google.html", GoogleSiteVerify())
	router.GET("/baidu.html", BaiduSiteVerify())
	router.GET("/index-now.html", IndexNowSiteVerify())
}
