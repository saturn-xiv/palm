package web

import (
	"context"
	"errors"
	"fmt"
	"log/slog"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/BurntSushi/toml"
	"github.com/gorilla/handlers"
	"github.com/gorilla/mux"
	"gorm.io/gorm"

	daisy_controllers "github.com/saturn-xiv/palm/atropa/daisy/controllers"
	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"github.com/saturn-xiv/palm/atropa/hibiscus"
)

func Launch(port uint16, config_file string, version string, debug bool) error {
	slog.Debug(fmt.Sprintf("load configuration from %s", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	_, _, jwt, err := crypto.Open(config.KeysDir)
	if err != nil {
		return err
	}
	db, err := config.Database.Open()
	if err != nil {
		return err
	}

	router, err := mount(db, jwt)
	if err != nil {
		return err
	}

	address := fmt.Sprintf("0.0.0.0:%d", port)
	slog.Info(fmt.Sprintf("listen on http://%s", address))
	server := &http.Server{
		Addr:         address,
		WriteTimeout: time.Second * 15,
		ReadTimeout:  time.Second * 15,
		IdleTimeout:  time.Second * 60,
		Handler:      router,
	}
	go func() {
		if err := server.ListenAndServe(); err != nil && !errors.Is(err, http.ErrServerClosed) {
			slog.Error(err.Error())
		}
	}()

	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	<-quit
	slog.Warn("shutting down http server...")

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	if err := server.Shutdown(ctx); err != nil {
		return err
	}

	slog.Info("http server exiting")
	return nil
}

func mount(db *gorm.DB, jwt *crypto.Jwt) (http.Handler, error) {
	router := mux.NewRouter()

	if err := daisy_controllers.Mount(router, db, jwt); err != nil {
		return nil, err
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
