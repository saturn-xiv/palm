package controllers

import (
	"errors"
	"net/http"
	"strings"

	"github.com/gin-gonic/gin"
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/services"
	"gorm.io/gorm"
)

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
const (
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

func NewCurrentUser(c *gin.Context, db *gorm.DB, jwt *crypto.Jwt) (*services.CurrentUser, error) {
	auth := c.GetHeader("Authorization")
	token, ok := strings.CutPrefix(auth, "Bearer ")
	if !ok {
		return nil, errors.New("authorization token is not provided")
	}
	return services.CurrentUserFromToken(token, db, jwt)
}
