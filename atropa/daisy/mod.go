package daisy

import (
	"github.com/gin-gonic/gin"
	"google.golang.org/grpc"

	"github.com/saturn-xiv/palm/atropa/env/crypto"
)

func Mount(router gin.IRouter, jwt *crypto.Jwt, backend *grpc.ClientConn) error {
	{
		group := router.Group("/api/twilio")
		callback := TwilioSmsStatusCallback(jwt, backend)
		group.GET("/sms-status-callback/:token", callback)
		group.POST("/sms-status-callback/:token", callback)
	}

	return nil
}
