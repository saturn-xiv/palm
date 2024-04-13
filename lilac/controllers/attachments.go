package controllers

import (
	"github.com/gin-gonic/gin"
	"github.com/minio/minio-go/v7"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
)

func AttachmentUpload(db *gorm.DB, jwt *crypto.Jwt, s3 *minio.Client) HandlerFunc {
	// TODO
	return func(c *gin.Context) error {
		return nil
	}
}
