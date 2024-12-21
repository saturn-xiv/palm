package controllers

import (
	"embed"
	"io/fs"
	"net/http"
	"os"
	"path"
	"time"

	"github.com/gorilla/handlers"
	"github.com/gorilla/mux"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/accounting"
	"github.com/saturn-xiv/palm/atropa/bbs"
	"github.com/saturn-xiv/palm/atropa/cms"
	daisy_controllers "github.com/saturn-xiv/palm/atropa/daisy/controllers"
	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"github.com/saturn-xiv/palm/atropa/hibiscus"
)

func Mount(node *embed.FS, third_assets string, theme string, db *gorm.DB, jwt *crypto.Jwt) (http.Handler, error) {
	router := mux.NewRouter().StrictSlash(true)
	router.PathPrefix("/3rd/").Handler(http.StripPrefix("/3rd/", http.FileServer(http.Dir(third_assets))))
	{
		dir, err := fs.Sub(node, path.Join("assets", theme))
		if err != nil {
			return nil, err
		}
		router.PathPrefix("/static/").Handler(http.StripPrefix("/static/", http.FileServer(http.FS(dir))))
	}

	if err := cms.Mount(router, db, jwt); err != nil {
		return nil, err
	}
	if err := bbs.Mount(router, db, jwt); err != nil {
		return nil, err
	}
	if err := accounting.Mount(router, db, jwt); err != nil {
		return nil, err
	}
	if err := daisy_controllers.Mount(router, db, jwt); err != nil {
		return nil, err
	}
	{
		router.HandleFunc("/{lang}/rss.xml", hibiscus.Wrap(RssByLang(db, jwt))).Methods(http.MethodGet)
		router.HandleFunc("/{lang}/sitemap.xml", hibiscus.Wrap(SitemapXmlByLang(db, jwt))).Methods(http.MethodGet)
		router.HandleFunc("/sitemap.xml", hibiscus.Wrap(SitemapXml(db, jwt))).Methods(http.MethodGet)
		router.HandleFunc("/robot.txt", hibiscus.Wrap(RobotTxt(db, jwt))).Methods(http.MethodGet)
		router.HandleFunc("/", hibiscus.Wrap(Home(db, jwt))).Methods(http.MethodGet)
	}

	handler := handlers.CORS(
		handlers.MaxAge(int(time.Minute*3)),
		handlers.AllowedHeaders([]string{hibiscus.HTTP_COOKIE_HEADER, hibiscus.HTTP_AUTHORIZATION_HEADER, hibiscus.HTTP_FORWARDED_FOR_HEADER}),
		handlers.AllowCredentials(),
		handlers.AllowedMethods([]string{http.MethodGet, http.MethodPost, http.MethodHead, http.MethodPut, http.MethodPatch, http.MethodDelete}),
	)(router)
	handler = handlers.CombinedLoggingHandler(os.Stdout, handler)
	handler = handlers.RecoveryHandler()(handler)

	return handler, nil
}
