package env

import (
	"fmt"
	"strings"

	pgadapter "github.com/casbin/casbin-pg-adapter"
	"github.com/casbin/casbin/v2"
	casbin_model "github.com/casbin/casbin/v2/model"
	rediswatcher "github.com/casbin/redis-watcher/v2"
	"github.com/go-pg/pg/v10"
	"github.com/redis/go-redis/v9"
	log "github.com/sirupsen/logrus"
)

type CasbinLogger struct {
	enabled bool
}

func (l *CasbinLogger) EnableLog(enable bool) {
	l.enabled = enable
}

func (l *CasbinLogger) IsEnabled() bool {
	return l.enabled
}

func (l *CasbinLogger) LogModel(model [][]string) {
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

func (l *CasbinLogger) LogEnforce(matcher string, request []interface{}, result bool, explains [][]string) {
	if !l.enabled {
		return
	}

	var reqStr strings.Builder
	reqStr.WriteString("Request: ")
	for i, rval := range request {
		if i != len(request)-1 {
			reqStr.WriteString(fmt.Sprintf("%v, ", rval))
		} else {
			reqStr.WriteString(fmt.Sprintf("%v", rval))
		}
	}
	reqStr.WriteString(fmt.Sprintf(" ---> %t\n", result))

	reqStr.WriteString("Hit Policy: ")
	for i, pval := range explains {
		if i != len(explains)-1 {
			reqStr.WriteString(fmt.Sprintf("%v, ", pval))
		} else {
			reqStr.WriteString(fmt.Sprintf("%v \n", pval))
		}
	}

	log.Info(reqStr.String())
}

func (l *CasbinLogger) LogPolicy(policy map[string][][]string) {
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

func (l *CasbinLogger) LogRole(roles []string) {
	if !l.enabled {
		return
	}

	log.Info("Roles: ", strings.Join(roles, "\n"))
}

func (l *CasbinLogger) LogError(err error, msg ...string) {
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

func casbinUpdateCallback(msg string) {
	log.Debugf("%s", msg)
}

func (p *Config) OpenCasbinEnforcer() (*casbin.Enforcer, error) {
	wtc, err := rediswatcher.NewWatcherWithCluster(
		strings.Join(p.Redis.Addrs(), ","),
		rediswatcher.WatcherOptions{
			ClusterOptions: redis.ClusterOptions{
				ClientName: "casbin",
			},
			Channel:    fmt.Sprintf("/%s-casbin", p.Redis.Namespace),
			IgnoreSelf: false,
		})
	if err != nil {
		return nil, err
	}

	db := pg.Connect(&pg.Options{
		User:     p.PostgreSql.User,
		Password: p.PostgreSql.User,
		Database: p.PostgreSql.DbName,
		Addr:     fmt.Sprintf("%s:%d", p.PostgreSql.Host, p.PostgreSql.Port),
	})
	defer db.Close()

	adp, err := pgadapter.NewAdapterByDB(db)
	if err != nil {
		return nil, err
	}

	// adp, err := pgadapter.NewAdapter(
	// 	fmt.Sprintf(
	// 		"postgresql://%s:%s@%s:%d/%s?sslmode=disable",
	// 		p.PostgreSql.User, p.PostgreSql.Password, p.PostgreSql.Host, p.PostgreSql.Port, p.PostgreSql.DbName,
	// 	),
	// 	p.PostgreSql.DbName,
	// )
	// if err != nil {
	// 	return nil, err
	// }

	mdl, err := casbin_model.NewModelFromString(gl_casbin_rbac_model)
	mdl.SetLogger(&CasbinLogger{})
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

	if err = wtc.SetUpdateCallback(casbinUpdateCallback); err != nil {
		return nil, err
	}

	return enf, nil
}
