package app

import (
	"context"
	"encoding/base64"
	"fmt"
	"log/slog"
	"net/http"
	"os"

	"github.com/BurntSushi/toml"
	"github.com/gorilla/csrf"
	"github.com/gorilla/handlers"
	"github.com/gorilla/mux"

	"github.com/saturn_xiv/palm/daisy/cache"
	"github.com/saturn_xiv/palm/daisy/controllers"
	"github.com/saturn_xiv/palm/daisy/rbac"
	"github.com/saturn_xiv/palm/daisy/s3"
)

type HttpServerConfig struct {
	Theme     string              `toml:"theme"`
	CsrfKey   string              `toml:"csrf-key"` // openssl rand -base64 32
	CookieKey string              `toml:"cookie-key"`
	Minio     *s3.Config          `toml:"minio"`
	Database  *Database           `toml:"database"`
	Redis     *cache.RedisCluster `toml:"redis"`
}

func LaunchHttpServer(config_file string, port uint16, debug bool) error {
	if debug {
		slog.SetLogLoggerLevel(slog.LevelDebug)
	} else {
		slog.SetLogLoggerLevel(slog.LevelInfo)
	}

	slog.Debug("load configuration from", "file", config_file)
	var config HttpServerConfig
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	db, err := config.Database.Open()
	if err != nil {
		return err
	}
	ctx_ := context.Background()
	redis_client, err := config.Redis.Open(ctx_)
	if err != nil {
		return err
	}
	enforcer, err := rbac.NewEnforcer(db, config.Redis.Addresses(), config.Redis.Namespace)
	if err != nil {
		return err
	}
	minio_client, err := config.Minio.Open()
	if err != nil {
		return err
	}
	ctx := &controllers.Context{
		Db:       db,
		Enforcer: enforcer,
		Theme:    config.Theme,
		Cache:    redis_client,
		S3:       minio_client,
	}

	csrf_key, err := base64.StdEncoding.DecodeString(config.CsrfKey)
	if err != nil {
		return err
	}

	router := mux.NewRouter()
	router.HandleFunc("/", controllers.Html(ctx, controllers.Home)).Methods(http.MethodGet)
	router.HandleFunc("/robots.txt", controllers.Text(ctx, controllers.NginxConf)).Methods(http.MethodGet)
	router.HandleFunc("/service.txt", controllers.Text(ctx, controllers.ServiceConf)).Methods(http.MethodGet)
	router.Use(
		csrf.Protect(csrf_key, csrf.Path("/")),
		handlers.ProxyHeaders,
		handlers.CORS(
			handlers.AllowCredentials(),
			handlers.AllowedMethods([]string{http.MethodGet, http.MethodPost}),
			handlers.AllowedHeaders([]string{controllers.ContentType, controllers.Authorization})),
	)

	addr := fmt.Sprintf(":%d", port)
	slog.Info("HTTP server listening at", "address", addr)
	return http.ListenAndServe(addr, handlers.CombinedLoggingHandler(os.Stdout, router))
}
