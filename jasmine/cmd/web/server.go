package web

import (
	"context"
	"fmt"
	h_template "html/template"
	"io"
	"log/slog"
	"net/http"
	"os"
	"os/signal"
	"path"
	"text/template"
	"time"

	"github.com/BurntSushi/toml"
	"github.com/gorilla/handlers"
	"github.com/gorilla/mux"

	"github.com/saturn-xiv/palm/jasmine/controllers"
	"github.com/saturn-xiv/palm/jasmine/env/crypto"
)

func Launch(port uint16, config_file string, theme string, version string) error {
	slog.Debug(fmt.Sprintf("load configuration from %s", config_file))

	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	slog.Debug("load text templates")
	t_tpl, err := template.ParseGlob(path.Join("templates", "*.txt"))
	if err != nil {
		return err
	}
	slog.Debug("load html templates")
	h_tpl, err := h_template.ParseGlob(path.Join("views", theme, "*.html"))
	if err != nil {
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

	_, _, jwt, err := crypto.Open(config.Keys)
	if err != nil {
		return err
	}

	ctx := controllers.Context{
		DB:           db,
		Redis:        redis,
		Session:      cookie,
		Jwt:          jwt,
		HTMLTemplate: h_tpl,
		TextTemplate: t_tpl,
	}

	router := mux.NewRouter()

	{
		router.PathPrefix("/static/").Handler(http.StripPrefix("/static/", http.FileServer(http.Dir("assets"))))
		router.HandleFunc("/robots.txt", warp(controllers.RobotsTxt)).Methods(http.MethodGet).Name("robots.txt")
		router.HandleFunc("/sitemap.xml", warp(controllers.SitemapIndex(&ctx))).Methods(http.MethodGet).Name("sitemap.index")
		router.HandleFunc("/sitemap/{lang}.xml", warp(controllers.SitemapByLang(&ctx))).Methods(http.MethodGet).Name("sitemap.by-lang")
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
		handlers.AllowedMethods([]string{http.MethodGet, http.MethodPost, http.MethodPut, http.MethodPatch, http.MethodDelete}),
		handlers.AllowCredentials(),
		handlers.AllowedOrigins(config.AllowedOrigins),
	)(router), port)
	return nil
}

func start(router http.Handler, port uint16) {
	addr := fmt.Sprintf("0.0.0.0:%d", port)
	slog.Debug(fmt.Sprintf("listen on http://%s", addr))
	server := &http.Server{
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

func warp(h controllers.HttpHandler) func(w http.ResponseWriter, r *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		if e := h(w, r); e != nil {
			slog.Error(e.Error())
			w.Header().Set(controllers.HeaderContentType, controllers.ContentTypeText)
			w.WriteHeader(http.StatusInternalServerError)
			io.WriteString(w, e.Error())
		}
	}
}
