package cmd

import (
	"embed"
	"fmt"
	"html/template"
	"net/http"

	"github.com/BurntSushi/toml"
	"github.com/gin-gonic/gin"
	log "github.com/sirupsen/logrus"

	graphql_controllers "github.com/saturn-xiv/palm/controllers/graphql"
	seo_controllers "github.com/saturn-xiv/palm/controllers/seo"
	twilio_controllers "github.com/saturn-xiv/palm/controllers/twilio"
	"github.com/saturn-xiv/palm/env"
)

//go:embed assets/*
var gl_assets_fs embed.FS

//go:embed templates/*
var gl_templates_fs embed.FS

func launch_web_server(port int, config_file string) error {
	log.Debugf("load configuration from %s", config_file)
	config := env.Config{}
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	db, err := config.OpenDatabase()
	if err != nil {
		return err
	}
	redis_casbin_watcher, err := config.Redis.CasbinWatcher()
	if err != nil {
		return err
	}
	enforcer, err := env.OpenCasbin(db, redis_casbin_watcher, "rbac_model.conf")
	if err != nil {
		return err
	}

	address := fmt.Sprintf("127.0.0.1:%d", port)
	router := gin.Default()

	tpl := template.Must(template.New("").ParseFS(gl_templates_fs, "templates/*/*.html"))
	router.SetHTMLTemplate(tpl)
	router.StaticFS("/public", http.FS(gl_assets_fs))
	seo_controllers.Register(router)

	graphal_handler, err := graphql_controllers.Handler()
	if err != nil {
		return err
	}
	router.Any("/graphql", gin.WrapH(graphal_handler))

	twilio_router := router.Group("/twilio")
	twilio_controllers.Register(twilio_router)

	log.Infof("listen on http://%s", address)
	return router.Run(address)
}
