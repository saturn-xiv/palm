package balsam

import (
	"io/fs"

	"github.com/gin-gonic/gin"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/env/crypto"
)

func Mount(router gin.IRouter, views_fs fs.FS, theme string, db *gorm.DB, jwt *crypto.Jwt) error {

	{
		router.GET("/robots.txt", RobotsTxt(views_fs))
		router.GET("/", Home())
	}
	return nil
}
