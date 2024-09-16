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

	"github.com/saturn-xiv/palm/atropa/controllers"
	"github.com/saturn-xiv/palm/atropa/env/crypto"
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
	cache, err := config.Redis.Open()
	if err != nil {
		return err
	}
	db, err := config.Database.Open()
	if err != nil {
		return err
	}

	router, err := controllers.Mount(db, cache, jwt)
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
