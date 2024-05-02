package cmd

import (
	"fmt"
	"log/slog"
)

func LaunchRpcServer(port uint16, config_file string, version string) error {
	slog.Info(fmt.Sprintf("listen on tcp://0.0.0.0:%d", port))
	return nil
}
