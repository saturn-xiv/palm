package http

import (
	"context"
	"fmt"
	h_template "html/template"
	"log/slog"
	http_ "net/http"
	"os"
	"os/signal"
	"path"
	t_template "text/template"
	"time"

	"github.com/BurntSushi/toml"
	"github.com/gorilla/handlers"
	"github.com/gorilla/mux"

	"github.com/saturn-xiv/palm/jasmine/controllers"
	"github.com/saturn-xiv/palm/jasmine/env/crypto"
	"github.com/saturn-xiv/palm/jasmine/web"
)

func Launch(port uint16, config_file string, theme string, version string) error {
	slog.Debug("load templates folder")
	t_tpl, err := t_template.ParseFS(gl_templates_fs, path.Join("templates", "*"))
	if err != nil {
		return err
	}

	slog.Debug("load views folder", slog.String("theme", theme))
	h_tpl, err := h_template.ParseFS(gl_views_fs, path.Join("views", theme, "*"))
	if err != nil {
		return err
	}

	slog.Debug(fmt.Sprintf("load configuration from %s", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	db, err := config.Database.Open()
	if err != nil {
		return err
	}
	_ctx := context.Background()
	redis, err := config.Redis.Open(_ctx)
	if err != nil {
		return err
	}
	cookie, err := config.OpenSessionStore()
	if err != nil {
		return err
	}

	_, _, jwt, err := crypto.Open(config.SecretsStore)
	if err != nil {
		return err
	}

	ctx := controllers.Context{
		DB:      db,
		Redis:   redis,
		Session: cookie,
		Jwt:     jwt,
	}

	router := mux.NewRouter()

	{
		router.PathPrefix("/static/").Handler(http_.StripPrefix("/static/", http_.FileServerFS(gl_assets_fs))).Methods(http_.MethodGet)
		router.HandleFunc("/robots.txt", web.WarpPlain(t_tpl, controllers.RobotsTxt)).Methods(http_.MethodGet).Name("robots.txt")
		router.HandleFunc("/sitemap.xml", web.WarpXml(controllers.SitemapIndex(&ctx))).Methods(http_.MethodGet).Name("sitemap.index")
		router.HandleFunc("/sitemap/{lang}.xml", web.WarpXml(controllers.SitemapByLang(&ctx))).Methods(http_.MethodGet).Name("sitemap.by-lang")
		router.HandleFunc("/rss/{lang}.xml", web.WarpXml(controllers.Rss(&ctx))).Methods(http_.MethodGet).Name("rss.by-lang")
		router.HandleFunc("/{lang}/", web.WarpHtml(h_tpl, controllers.HomeByLang(&ctx))).Methods(http_.MethodGet).Name("home.by-lang")
		router.HandleFunc("/", web.WarpHtml(h_tpl, controllers.Home(&ctx))).Methods(http_.MethodGet).Name("home.index")
	}
	{
		api := router.PathPrefix("/api").Subrouter()
		csrf, err := config.OpenCsrf()
		if err != nil {
			return err
		}
		api.Use(csrf)
		// TODO
	}
	start(handlers.CORS(
		handlers.AllowedMethods([]string{http_.MethodGet, http_.MethodPost, http_.MethodPut, http_.MethodPatch, http_.MethodDelete}),
		handlers.AllowCredentials(),
		handlers.AllowedOrigins(config.AllowedOrigins),
	)(router), port)
	return nil
}

func start(router http_.Handler, port uint16) {
	addr := fmt.Sprintf("0.0.0.0:%d", port)
	slog.Debug(fmt.Sprintf("listen on http://%s", addr))
	server := &http_.Server{
		Addr: addr,
		// FIXME using logging
		Handler: handlers.CombinedLoggingHandler(os.Stdout, router),
	}
	go func() {
		if err := server.ListenAndServe(); err != nil {
			slog.Error(err.Error())
		}
	}()
	c := make(chan os.Signal, 1)

	signal.Notify(c, os.Interrupt)

	<-c

	ctx, cancel := context.WithTimeout(context.Background(), time.Second*15)
	defer cancel()

	server.Shutdown(ctx)

	slog.Warn("shutting down")
	os.Exit(0)
}
