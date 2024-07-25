package web

import (
	"context"
	"encoding/base64"
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

	balsam_pb "github.com/saturn-xiv/palm/atropa/balsam/services/v2"
	"github.com/saturn-xiv/palm/atropa/controllers"
	"github.com/saturn-xiv/palm/atropa/env/crypto"
)

func Launch(port uint16, config_file string, keys_dir string, version string, debug bool) error {
	slog.Debug(fmt.Sprintf("load configuration from %s", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	_, _, jwt, err := crypto.Open(keys_dir)
	if err != nil {
		return err
	}
	cache, err := config.Redis.Open()
	if err != nil {
		return err
	}
	backend, err := config.Backend.Open()
	if err != nil {
		return err
	}
	{
		// TODO change to fetch layout
		slog.Debug("test backend-rpc service")
		hi := "hello, palm!"
		ctx := context.Background()
		cli := balsam_pb.NewHMacClient(backend)
		res, err := cli.Sign(ctx, &balsam_pb.HMacSignRequest{
			Plain: []byte(hi),
		})
		if err != nil {
			return err
		}
		slog.Debug("receive", slog.String("hmac code", base64.StdEncoding.WithPadding(base64.NoPadding).EncodeToString(res.Code)))
	}

	gin.DisableConsoleColor()
	if !debug {
		gin.SetMode(gin.ReleaseMode)
	}
	router := gin.Default()
	if err = controllers.Mount(router, config.Theme, cache, jwt, backend); err != nil {
		return err
	}

	address := fmt.Sprintf("0.0.0.0:%d", port)
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
