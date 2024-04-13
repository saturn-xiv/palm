package web

import (
	"github.com/BurntSushi/toml"
	"github.com/fvbock/endless"
	"github.com/gin-gonic/gin"
	log "github.com/sirupsen/logrus"

	"github.com/saturn-xiv/palm/lilac/controllers"
	"github.com/saturn-xiv/palm/lilac/env"
	"github.com/saturn-xiv/palm/lilac/env/crypto"
)

func Launch(address string, config_file string, keys_dir string) error {
	log.Debugf("load configuration from %s", config_file)
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	db, err := config.Database.Open()
	if err != nil {
		return err
	}

	_, _, jwt, err := crypto.Open(keys_dir)
	if err != nil {
		return err
	}
	s3, err := config.Minio.Open()
	if err != nil {
		return err
	}

	gin.DisableConsoleColor()
	router := gin.New()
	router.Use(env.GinLogger())
	router.Use(gin.Recovery())
	router.POST("/attachments", controllers.AttachmentUpload(db, jwt, s3))
	log.Infof("listen on http://%s", address)
	endless.ListenAndServe(address, router)
	return nil
}
