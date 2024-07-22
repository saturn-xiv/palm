package daisy

import (
	"github.com/gin-gonic/gin"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/daisy/models"
	"github.com/saturn-xiv/palm/atropa/env/crypto"
)

func Mount(router gin.IRouter, db *gorm.DB, jwt *crypto.Jwt) error {
	if err := db.AutoMigrate(models.TwilioSmsLogs{}); err != nil {
		return err
	}
	{
		group := router.Group("/api/twilio")
		callback := TwilioSmsStatusCallback(db, jwt)
		group.GET("/sms-status-callback/:token", callback)
		group.POST("/sms-status-callback/:token", callback)
	}

	return nil
}
