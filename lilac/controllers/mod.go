package controllers

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
var (
	ATOM_CONTENT_TYPE   = "application/atom+xml; charset=UTF-8"
	XML_CONTENT_TYPE    = "application/xml; charset=UTF-8"
	TEXT_CONTENT_TYPE   = "text/plain; charset=UTF-8"
	CONTENT_TYPE_HEADER = "Content-Type"
)

type HandlerFunc = func(c *gin.Context) error

func Warp(f HandlerFunc) gin.HandlerFunc {
	return func(c *gin.Context) {
		if err := f(c); err != nil {
			c.AbortWithError(http.StatusInternalServerError, err)
		}
	}
}
