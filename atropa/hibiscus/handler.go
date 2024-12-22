package hibiscus

import (
	"embed"
	"io/fs"
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

func StaticFS(router *mux.Router, path string, content *embed.FS, dir string) error {
	fs, err := fs.Sub(content, dir)
	if err != nil {
		return err
	}
	router.PathPrefix(path).Handler(http.StripPrefix(path, http.FileServer(http.FS(fs)))).Methods(http.MethodHead, http.MethodGet)
	return nil
}
