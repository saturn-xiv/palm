package env

import (
	"fmt"
	"strings"

	"github.com/casbin/casbin/v2"
	casbin_model "github.com/casbin/casbin/v2/model"
	gormadapter "github.com/casbin/gorm-adapter/v3"
	rediswatcher "github.com/casbin/redis-watcher/v2"
	"github.com/redis/go-redis/v9"
	log "github.com/sirupsen/logrus"
	"gorm.io/gorm"
)

type Logger struct {
	enabled bool
}

func (l *Logger) EnableLog(enable bool) {
	l.enabled = enable
}

func (l *Logger) IsEnabled() bool {
	return l.enabled
}

func (l *Logger) LogModel(model [][]string) {
	if !l.enabled {
		return
	}
	var str strings.Builder
	str.WriteString("Model: ")
	for _, v := range model {
		str.WriteString(fmt.Sprintf("%v\n", v))
	}

	log.Info(str.String())
}

func (l *Logger) LogEnforce(matcher string, request []interface{}, result bool, explains [][]string) {
	if !l.enabled {
		return
	}

	var req_s strings.Builder
	req_s.WriteString("Request: ")
	for i, rval := range request {
		if i != len(request)-1 {
			req_s.WriteString(fmt.Sprintf("%v, ", rval))
		} else {
			req_s.WriteString(fmt.Sprintf("%v", rval))
		}
	}
	req_s.WriteString(fmt.Sprintf(" ---> %t\n", result))

	req_s.WriteString("Hit Policy: ")
	for i, pval := range explains {
		if i != len(explains)-1 {
			req_s.WriteString(fmt.Sprintf("%v, ", pval))
		} else {
			req_s.WriteString(fmt.Sprintf("%v \n", pval))
		}
	}

	log.Info(req_s.String())
}

func (l *Logger) LogPolicy(policy map[string][][]string) {
	if !l.enabled {
		return
	}

	var str strings.Builder
	str.WriteString("Policy: ")
	for k, v := range policy {
		str.WriteString(fmt.Sprintf("%s : %v\n", k, v))
	}

	log.Info(str.String())
}

func (l *Logger) LogRole(roles []string) {
	if !l.enabled {
		return
	}

	log.Info("Roles: ", strings.Join(roles, "\n"))
}

func (l *Logger) LogError(err error, msg ...string) {
	if !l.enabled {
		return
	}
	log.Error(msg, err)
}

var gl_casbin_rbac_model string = `
[request_definition]
r = sub, obj, act

[policy_definition]
p = sub, obj, act

[role_definition]
g = _, _

[policy_effect]
e = some(where (p.eft == allow))

[matchers]
m = g(r.sub, p.sub) && r.obj == p.obj && r.act == p.act
`

func updateCallback(msg string) {
	log.Debugf("%s", msg)
}

func OpenCasbinEnforcer(namespace string, db *gorm.DB, redis_addrs []string) (*casbin.Enforcer, error) {
	log.Debugf("open casbin redis watcher")
	wtc, err := rediswatcher.NewWatcherWithCluster(
		strings.Join(redis_addrs, ","),
		rediswatcher.WatcherOptions{
			ClusterOptions: redis.ClusterOptions{
				ClientName: "casbin-watcher",
			},
			Channel:    fmt.Sprintf("%s://casbin-watcher", namespace),
			IgnoreSelf: false,
		})
	if err != nil {
		return nil, err
	}

	log.Debugf("open casbin gorm adapter")
	adp, err := gormadapter.NewAdapterByDB(db)
	if err != nil {
		return nil, err
	}

	mdl, err := casbin_model.NewModelFromString(gl_casbin_rbac_model)
	mdl.SetLogger(&Logger{})
	if err != nil {
		return nil, err
	}

	enf, err := casbin.NewEnforcer(mdl, adp)
	if err != nil {
		return nil, err
	}

	if err = enf.SetWatcher(wtc); err != nil {
		return nil, err
	}

	if err = wtc.SetUpdateCallback(updateCallback); err != nil {
		return nil, err
	}

	return enf, nil
}
