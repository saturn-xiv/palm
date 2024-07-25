package balsam

import (
	"io/fs"

	"github.com/gin-gonic/gin"
	"google.golang.org/grpc"

	"github.com/saturn-xiv/palm/atropa/env/crypto"
)

func Mount(router gin.IRouter, views_fs fs.FS, theme string, jwt *crypto.Jwt, backend *grpc.ClientConn) error {

	{
		router.GET("/robots.txt", RobotsTxt(views_fs))
		router.GET("/", Home())
	}
	return nil
}
