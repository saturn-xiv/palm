package app

import (
	"log/slog"

	"github.com/BurntSushi/toml"
)

type NetScanConfig struct {
	PostgreSql PostgreSql `toml:"postgresql"`
}

func NetScan(config_file string, debug bool) error {
	if debug {
		slog.SetLogLoggerLevel(slog.LevelDebug)
	} else {
		slog.SetLogLoggerLevel(slog.LevelInfo)
	}

	slog.Debug("load configuration from", "file", config_file)
	var config NetScanConfig

	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	_, err := config.PostgreSql.Open()
	if err != nil {
		return err
	}
	return nil
}
