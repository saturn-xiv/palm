package env

import (
	"fmt"
	"time"

	log "github.com/sirupsen/logrus"
	"gorm.io/driver/sqlserver"
	"gorm.io/gorm"
)

type SqlServer struct {
	Host     string `toml:"host"`
	Port     uint16 `toml:"port"`
	DbName   string `toml:"dbname"`
	User     string `toml:"user"`
	Password string `toml:"password"`
	PoolSize int    `toml:"pool-size"`
}

func (p *SqlServer) Url() string {
	return fmt.Sprintf("sqlserver://%s:%s@%s:%d?database=%s",
		p.User, p.Password, p.Host, p.Port, p.DbName,
	)
}

func (p *SqlServer) Open(config *gorm.Config) (*gorm.DB, error) {
	log.Infof("open sqlserver://%s@%s:%d/%s", p.User, p.Host, p.Port, p.DbName)
	db, err := gorm.Open(sqlserver.Open(p.Url()), config)
	if err != nil {
		return nil, err
	}
	{
		it, err := db.DB()
		if err != nil {
			return nil, err
		}
		it.SetMaxOpenConns(p.PoolSize)
		it.SetConnMaxIdleTime(time.Hour)
	}
	return db, nil
}
