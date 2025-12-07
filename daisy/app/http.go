package app

import (
	"context"
	"encoding/base64"
	"fmt"
	"log/slog"
	"net/http"
	"os"
	"time"

	"github.com/BurntSushi/toml"
	"github.com/gorilla/csrf"
	"github.com/gorilla/handlers"
	"github.com/gorilla/mux"

	"github.com/saturn-xiv/palm/daisy/cache"
	"github.com/saturn-xiv/palm/daisy/controllers"
	"github.com/saturn-xiv/palm/daisy/graphql"
	"github.com/saturn-xiv/palm/daisy/queue"
	"github.com/saturn-xiv/palm/daisy/rbac"
	"github.com/saturn-xiv/palm/daisy/s3"
)

type HttpServerConfig struct {
	Theme        string              `toml:"theme"`
	CsrfKey      string              `toml:"csrf-key"` // openssl rand -base64 32
	CookieKey    string              `toml:"cookie-key"`
	Minio        *s3.Config          `toml:"minio"`
	Database     *Database           `toml:"database"`
	Redis        *cache.RedisCluster `toml:"redis"`
	RabbitMQ     *queue.RabbitMQ     `toml:"rabbitmq"`
	GoogleOauth2 *GoogleOauth2       `toml:"google-oauth2"`
}

func LaunchHttpServer(config_file string, port uint16, debug bool) error {
	slog.Debug("load configuration from", "file", config_file)
	var config HttpServerConfig
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	db, err := config.Database.Open(debug)
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
	graphql_hnd, err := graphql.Handler(db, redis_client, config.GoogleOauth2)
	if err != nil {
		return err
	}

	router := mux.NewRouter()
	router.HandleFunc("/", controllers.Html(ctx, controllers.Home)).Methods(http.MethodGet)
	router.Handle("/graphql", graphql_hnd).Methods(http.MethodGet, http.MethodPost)
	router.HandleFunc("/robots.txt", controllers.Text(ctx, controllers.NginxConf)).Methods(http.MethodGet)
	router.HandleFunc("/service.txt", controllers.Text(ctx, controllers.ServiceConf)).Methods(http.MethodGet)
	router.HandleFunc("/{lang}/rss.xml", controllers.Xml(ctx, controllers.Rss)).Methods(http.MethodGet)

	router.PathPrefix("/assets/").Handler(http.StripPrefix("/assets/", controllers.Assets())).Methods(http.MethodGet)
	router.PathPrefix("/3rd/").Handler(http.StripPrefix("/3rd/", http.FileServer(http.Dir("node_modules")))).Methods(http.MethodGet)

	router.Use(
		csrf.Protect(csrf_key, csrf.Path("/")),
		handlers.ProxyHeaders,
		handlers.CORS(
			handlers.AllowCredentials(),
			handlers.AllowedMethods([]string{http.MethodGet, http.MethodPost}),
			handlers.AllowedHeaders([]string{controllers.ContentType, controllers.Authorization})),
	)

	addr := fmt.Sprintf("127.0.0.1:%d", port)
	slog.Info("HTTP server listening at", "address", addr)
	server := &http.Server{
		Handler:      handlers.CombinedLoggingHandler(os.Stdout, router),
		Addr:         addr,
		WriteTimeout: 15 * time.Second,
		ReadTimeout:  15 * time.Second,
	}
	return server.ListenAndServe()
}
