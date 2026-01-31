package rbac

import (
	_ "embed"
	"fmt"
	"log/slog"
	"strings"

	"github.com/casbin/casbin/v3"
	"github.com/casbin/casbin/v3/model"
	gormadapter "github.com/casbin/gorm-adapter/v3"
	rediswatcher "github.com/casbin/redis-watcher/v2"
	"gorm.io/gorm"
)

//go:embed rbac_model.conf
var gl_rbac_model string

func updateCallback(msg string) {
	slog.Debug(msg)
}

func NewEnforcer(db *gorm.DB, redis_addresses []string, redis_namespace string) (*casbin.Enforcer, error) {
	// gormadapter.TurnOffAutoMigrate(db)
	adapter, err := gormadapter.NewAdapterByDB(db)
	if err != nil {
		return nil, err
	}
	model, err := model.NewModelFromString(gl_rbac_model)
	if err != nil {
		return nil, err
	}
	watcher, err := rediswatcher.NewWatcherWithCluster(
		strings.Join(redis_addresses, ","),
		rediswatcher.WatcherOptions{
			Channel:    fmt.Sprintf("%s://casbin", redis_namespace),
			IgnoreSelf: true,
		})
	if err != nil {
		return nil, err
	}
	if err = watcher.SetUpdateCallback(updateCallback); err != nil {
		return nil, err
	}

	enforcer, err := casbin.NewEnforcer(model, adapter)
	if err != nil {
		return nil, err
	}
	if err = enforcer.SetWatcher(watcher); err != nil {
		return nil, err
	}
	if err = enforcer.LoadPolicy(); err != nil {
		return nil, err
	}
	return enforcer, nil

}
