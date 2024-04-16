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
	"github.com/gin-gonic/gin"

	"github.com/saturn-xiv/palm/lilac/controllers"
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/i18n"
)

func Launch(address string, config_file string, keys_dir string) error {
	slog.Debug(fmt.Sprintf("load configuration from %s", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	db, err := config.Database.Open()
	if err != nil {
		return err
	}
	cache, err := config.Redis.Open(config.Namespace)
	if err != nil {
		return err
	}

	_, _, jwt, err := crypto.Open(keys_dir)
	if err != nil {
		return err
	}
	s3, err := config.Minio.Open()
	if err != nil {
		return err
	}
	i18n, err := i18n.New(db)
	if err != nil {
		return err
	}

	gin.DisableConsoleColor()
	router := gin.New()
	if err = controllers.Mount(router, db, cache, jwt, i18n, s3); err != nil {
		return err
	}

	slog.Info(fmt.Sprintf("listen on http://%s", address))
	server := &http.Server{
		Addr:    address,
		Handler: router,
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
