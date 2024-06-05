package rpc

import (
	"fmt"
	"log/slog"
	"reflect"

	"github.com/BurntSushi/toml"

	"github.com/saturn-xiv/palm/atropa/env"
)

func Launch(port uint16, config_file string, version string) error {
	slog.Debug(fmt.Sprintf("load configuration from %s", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	db, err := config.Database.Open()
	if err != nil {
		return err
	}
	_, err = env.OpenCasbinEnforcer(config.Redis.Namespace, db, config.Redis.Options().Addrs)
	if err != nil {
		return err
	}

	_ = fmt.Sprintf("0.0.0.0:%d", port)

	return nil
}

func serviceName(it interface{}) string {
	return fmt.Sprintf("%s/%s", reflect.TypeOf(it).Elem().PkgPath(), reflect.TypeOf(it).Elem().Name())
}
