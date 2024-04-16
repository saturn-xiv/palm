package controllers

import (
	"net/http"
	"reflect"

	"github.com/gin-gonic/gin"
	"github.com/minio/minio-go/v7"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/env/redis"
	"github.com/saturn-xiv/palm/lilac/services"
)

func Heartbeat(db *gorm.DB, cache *redis.Client, jwt *crypto.Jwt, s3 *minio.Client) HandlerFunc {
	return func(c *gin.Context) error {
		token := c.Param("token")
		_, sub, _, err := jwt.Verify(token, reflect.TypeOf(services.CurrentUser{}).PkgPath(), "heartbeat")
		if err != nil {
			return err
		}
		{
			db, err := db.DB()
			if err != nil {
				return err
			}
			if err := db.Ping(); err != nil {
				return err
			}
		}
		redis, err := cache.Heartbeat(c)
		if err != nil {
			return err
		}

		c.JSON(http.StatusOK, gin.H{
			db.Dialector.Name(): true,
			"cache":             redis,
			"subject":           sub,
		})
		return nil
	}
}
