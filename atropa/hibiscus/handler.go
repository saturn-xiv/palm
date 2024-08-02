package hibiscus

import (
	"embed"
	"net/http"

	"github.com/gorilla/mux"
)

type HandlerFunc = func(c *Context)

func Wrap(f HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		f(&Context{
			responseWriter: w,
			request:        r,
		})
	}
}

func Static(router *mux.Router, path string, dir string) {
	router.PathPrefix(path).Handler(http.StripPrefix(path, http.FileServer(http.Dir(dir)))).Methods(http.MethodHead, http.MethodGet)
}

func StaticFS(router *mux.Router, path string, fs embed.FS) {
	router.PathPrefix(path).Handler(http.StripPrefix(path, http.FileServer(http.FS(fs)))).Methods(http.MethodHead, http.MethodGet)
}
