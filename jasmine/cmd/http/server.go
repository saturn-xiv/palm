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
	"strings"
	t_template "text/template"
	"time"

	"github.com/BurntSushi/toml"
	"github.com/gorilla/handlers"
	"github.com/gorilla/mux"

	"github.com/saturn-xiv/palm/jasmine/controllers"
	"github.com/saturn-xiv/palm/jasmine/env/crypto"
	"github.com/saturn-xiv/palm/jasmine/web"
)

func Launch(port uint16, config_file string, version string) error {
	web.EnsureStopped()
	slog.Debug("load embed text templates")
	t_tpl, err := t_template.ParseFS(gl_templates_fs, path.Join("templates", "*"))
	if err != nil {
		return err
	}

	slog.Debug("load embed html views")
	h_tpl, err := h_template.New("").Funcs(h_template.FuncMap{
		"join": strings.Join,
		"trim": strings.TrimSpace,
		"noescape": func(s string) h_template.HTML {
			return h_template.HTML(s)
		},
	}).ParseFS(gl_views_fs, path.Join("views", "**", "**"))
	if err != nil {
		return err
	}

	{
		root := "views"
		if _, err = os.Stat(root); err == nil {
			slog.Debug("load html views from filesystem", slog.String("folder", root))
			if h_tpl, err = h_tpl.ParseGlob(path.Join(root, "**", "**")); err != nil {
				return err
			}
		}
	}

	slog.Debug("load configuration", slog.String("file", config_file))
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
		router.PathPrefix("/static/").Handler(http_.StripPrefix("/static/", http_.FileServerFS(gl_assets_fs))).Methods(http_.MethodGet).Name("static")
		{
			root := "public"
			if _, err = os.Stat(root); err == nil {
				slog.Debug("mount public folder from filesystem")
				router.PathPrefix("/public/").Handler(http_.StripPrefix("/public/", http_.FileServer(http_.Dir(root)))).Methods(http_.MethodGet).Name(root)
			}
		}
		router.HandleFunc("/robots.txt", web.WarpPlain(t_tpl, controllers.RobotsTxt)).Methods(http_.MethodGet).Name("robots.txt")
		router.HandleFunc("/sitemap.xml", web.WarpXml(controllers.SitemapIndex(&ctx))).Methods(http_.MethodGet).Name("sitemap.index")
		router.HandleFunc("/sitemap/{lang}.xml", web.WarpXml(controllers.SitemapByLang(&ctx))).Methods(http_.MethodGet).Name("sitemap.by-lang")
		router.HandleFunc("/rss/{lang}.xml", web.WarpXml(controllers.Rss(&ctx))).Methods(http_.MethodGet).Name("rss.by-lang")
		router.HandleFunc("/p-{hash}.html", web.WarpHtml(h_tpl, controllers.ShowPage(&ctx))).Methods(http_.MethodGet).Name("pages.show.by-hash")
		router.HandleFunc("/{lang}/", web.WarpHtml(h_tpl, controllers.HomeByLang(&ctx))).Methods(http_.MethodGet).Name("home.by-lang")
		router.HandleFunc("/", web.WarpHtml(h_tpl, controllers.Home(&ctx))).Methods(http_.MethodGet).Name("home.index")
	}
	{
		// api := router.PathPrefix("/api").Subrouter()
		// csrf, err := config.OpenCsrf()
		// if err != nil {
		// 	return err
		// }
		// api.Use(csrf)
	}

	for _, t := range t_tpl.Templates() {
		slog.Debug("found text template", slog.String("name", t.Name()))
	}
	for _, t := range h_tpl.Templates() {
		slog.Debug("found html template", slog.String("name", t.Name()))
	}
	if err := router.Walk(func(route *mux.Route, router *mux.Router, ancestors []*mux.Route) error {
		methods, err := route.GetMethods()
		if err != nil {
			return err
		}
		path, err := route.GetPathTemplate()
		if err != nil {
			return err
		}
		slog.Debug("found route", slog.String("name", route.GetName()), slog.String("methods", strings.Join(methods, ",")), slog.String("path", path))
		return nil
	}); err != nil {
		return err
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
	slog.Debug("listen on", slog.String("address", addr))
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

	slog.Warn("server stopped gracefully")
	os.Exit(0)
}
