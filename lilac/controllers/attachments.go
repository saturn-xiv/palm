package controllers

import (
	"errors"
	"log/slog"
	"net/http"
	"path/filepath"
	"strconv"
	"time"

	"github.com/gabriel-vasile/mimetype"
	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
	"github.com/minio/minio-go/v7"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/models"
)

func resource_from_query(c *gin.Context) (*models.Resource, error) {
	ty := c.Query("resource_type")
	ids := c.Query("resource_id")
	if ty == "" || ids == "" {
		return nil, errors.New("empty resource queries")
	}
	id, err := strconv.ParseUint(ids, 10, 32)
	if err != nil {
		return nil, err
	}
	return &models.Resource{Type: ty, ID: uint32(id)}, nil
}

func AttachmentUpload(db *gorm.DB, jwt *crypto.Jwt, s3 *minio.Client) HandlerFunc {
	return func(c *gin.Context) error {
		bucket := c.Param("bucket")
		resource, _ := resource_from_query(c)
		user, err := NewCurrentUser(c, db, jwt)
		if err != nil {
			return err
		}
		form, err := c.MultipartForm()
		if err != nil {
			return err
		}
		files := form.File["file[]"]

		var items []models.Attachment
		for _, file := range files {
			slog.Debug("upload", slog.String("file", file.Filename), slog.Int64("size", file.Size))
			fd, err := file.Open()
			if err != nil {
				return err
			}
			defer fd.Close()
			content_type, err := mimetype.DetectReader(fd)
			if err != nil {
				return err
			}

			name := uuid.New().String()
			{
				ext := filepath.Ext(file.Filename)
				if ext != "" {
					name += ext
				}
			}

			if _, err = s3.PutObject(c, bucket, name, fd, file.Size, minio.PutObjectOptions{}); err != nil {
				return err
			}
			if err := db.Transaction(func(tx *gorm.DB) error {
				now := time.Now()
				it := models.Attachment{
					UserID:      user.Payload.ID,
					Title:       file.Filename,
					Bucket:      bucket,
					Name:        name,
					Size:        file.Size,
					ContentType: content_type.String(),
					Model: models.Model{
						UpdatedAt: now,
						CreatedAt: now,
					},
				}
				if rst := tx.Create(&it); rst.Error != nil {
					return rst.Error
				}
				if resource != nil {
					if rst := tx.Create(&models.AttachmentResource{
						AttachmentID: it.ID,
						Resource:     *resource,
						CreatedAt:    now,
					}); rst.Error != nil {
						return rst.Error
					}
				}
				items = append(items, it)
				return nil
			}); err != nil {
				return err
			}
		}
		c.JSON(http.StatusOK, items)
		return nil
	}
}
