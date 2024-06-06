package web

import (
	"fmt"
	"log/slog"

	"github.com/BurntSushi/toml"
)

func Launch(port uint16, config_file string, version string) error {
	slog.Debug(fmt.Sprintf("load configuration from %s", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	// TODO
	return nil
}
