package env

import (
	"fmt"
	"log/slog"
	"time"

	mysql_ "gorm.io/driver/mysql"
	"gorm.io/gorm"
)

type MySql struct {
	Host     string `toml:"host"`
	Port     uint16 `toml:"port"`
	DbName   string `toml:"dbname"`
	User     string `toml:"user"`
	Password string `toml:"password"`
	PoolSize int    `toml:"pool-size"`
}

// https://github.com/go-sql-driver/mysql#dsn-data-source-name
func (p *MySql) Url() string {
	return fmt.Sprintf("%s:%s@tcp(%s:%d)/%s?charset=utf8mb4&parseTime=True&loc=UTC",
		p.User, p.Password, p.Host, p.Port, p.DbName,
	)
}

func (p *MySql) Open(config *gorm.Config) (*gorm.DB, error) {
	slog.Info(fmt.Sprintf("open mysql://%s@%s:%d/%s", p.User, p.Host, p.Port, p.DbName))
	db, err := gorm.Open(mysql_.Open(p.Url()), config)
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
