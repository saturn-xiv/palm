package controllers

import (
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/minio/minio-go/v7"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/redis"
)

func HealthCheck(db *gorm.DB, cache *redis.Client, s3 *minio.Client) HandlerFunc {
	return func(c *gin.Context) error {
		{
			db, err := db.DB()
			if err != nil {
				return err
			}
			if err := db.Ping(); db != nil {
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
		})
		return nil
	}
}
