package env

import (
	"fmt"
	"log/slog"
	"time"

	postgres_ "gorm.io/driver/postgres"
	"gorm.io/gorm"
)

type PostgreSql struct {
	Host     string `toml:"host"`
	Port     uint16 `toml:"port"`
	DbName   string `toml:"dbname"`
	User     string `toml:"user"`
	Password string `toml:"password"`
	PoolSize int    `toml:"pool-size"`
}

func (p *PostgreSql) Url() string {
	return fmt.Sprintf("host=%s port=%d user=%s password=%s dbname=%s sslmode=disable TimeZone=UTC",
		p.Host, p.Port, p.User, p.Password, p.DbName,
	)
}

func (p *PostgreSql) Open(config *gorm.Config) (*gorm.DB, error) {
	slog.Info(fmt.Sprintf("open postgresql://%s@%s:%d/%s", p.User, p.Host, p.Port, p.DbName))
	db, err := gorm.Open(postgres_.Open(p.Url()), config)
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
