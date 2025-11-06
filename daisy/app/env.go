package app

import (
	"errors"
	"fmt"
	"log/slog"

	"gorm.io/driver/mysql"
	"gorm.io/driver/postgres"
	"gorm.io/driver/sqlserver"
	"gorm.io/gorm"
)

type Database struct {
	PostgreSql *PostgreSql `toml:"postgresql"`
	MySql      *PostgreSql `toml:"mysql"`
	SqlServer  *PostgreSql `toml:"sqlserver"`
	PoolSize   uint        `toml:"pool-size"`
}

func (p *Database) Open() (*gorm.DB, error) {
	if p.PostgreSql != nil {
		return p.PostgreSql.Open()
	}
	if p.MySql != nil {
		return p.MySql.Open()
	}
	if p.SqlServer != nil {
		return p.SqlServer.Open()
	}
	return nil, errors.New("couldn't open a database")
}

type PostgreSql struct {
	Host     string `toml:"host"`
	Port     uint16 `toml:"port"`
	DbName   string `toml:"db-name"`
	User     string `toml:"user"`
	Password string `toml:"password"`
}

func (p *PostgreSql) Open() (*gorm.DB, error) {
	slog.Info("open postgresql", "host", p.Host, "port", p.Port, "db-name", p.DbName, "user", p.User)
	return gorm.Open(postgres.New(postgres.Config{
		DSN:                  fmt.Sprintf("user=%s password=%s dbname=%s host=%s port=%d sslmode=disable TimeZone=UTC", p.User, p.Password, p.DbName, p.Host, p.Port),
		PreferSimpleProtocol: true,
	}), &gorm.Config{})
}

type MySql struct {
	Host     string `toml:"host"`
	Port     uint16 `toml:"port"`
	DbName   string `toml:"db-name"`
	User     string `toml:"user"`
	Password string `toml:"password"`
}

func (p *MySql) Open() (*gorm.DB, error) {
	// https://github.com/go-sql-driver/mysql#dsn-data-source-name
	slog.Info("open mysql", "host", p.Host, "port", p.Port, "db-name", p.DbName, "user", p.User)
	return gorm.Open(
		mysql.Open(fmt.Sprintf("%s:%s@tcp(%s:%d)/%s?charset=utf8mb4&parseTime=True&loc=Local", p.User, p.Password, p.Host, p.Port, p.DbName)),
		&gorm.Config{})
}

type SqlServer struct {
	Host     string `toml:"host"`
	Port     uint16 `toml:"port"`
	DbName   string `toml:"db-name"`
	User     string `toml:"user"`
	Password string `toml:"password"`
}

func (p *SqlServer) Open() (*gorm.DB, error) {
	slog.Info("open sqlserver", "host", p.Host, "port", p.Port, "db-name", p.DbName, "user", p.User)
	return gorm.Open(
		sqlserver.Open(fmt.Sprintf("sqlserver://%s:%s@%s:%d?database=%s", p.User, p.Password, p.Host, p.Port, p.DbName)),
		&gorm.Config{})
}
