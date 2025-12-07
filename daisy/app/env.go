package app

import (
	"errors"
	"fmt"
	"log/slog"

	"github.com/saturn-xiv/palm/daisy/models"
	"golang.org/x/oauth2"
	"golang.org/x/oauth2/google"
	"gorm.io/driver/mysql"
	"gorm.io/driver/postgres"
	"gorm.io/driver/sqlserver"
	"gorm.io/gorm"
	"gorm.io/gorm/logger"
)

type databaseVersion struct {
	Version string
}

type Database struct {
	PostgreSql *PostgreSql `toml:"postgresql"`
	MySql      *PostgreSql `toml:"mysql"`
	SqlServer  *PostgreSql `toml:"sqlserver"`
	PoolSize   uint        `toml:"pool-size"`
}

func (p *Database) Open(debug bool) (*gorm.DB, error) {
	ver, db, err := p.open(debug)
	if err != nil {
		return nil, err
	}
	slog.Debug(ver)
	if err = db.AutoMigrate(&models.GoogleOauth2User{}); err != nil {
		return nil, err
	}
	return db, nil
}
func (p *Database) open(debug bool) (string, *gorm.DB, error) {
	config := gorm.Config{}
	if debug {
		config.Logger = logger.Default.LogMode(logger.Info)
	} else {
		config.Logger = logger.Default.LogMode(logger.Warn)
	}
	if p.PostgreSql != nil {
		return p.PostgreSql.Open(&config)
	}
	if p.MySql != nil {
		return p.MySql.Open(&config)
	}
	if p.SqlServer != nil {
		return p.SqlServer.Open(&config)
	}
	return "", nil, errors.New("couldn't open a database")
}

type PostgreSql struct {
	Host     string `toml:"host"`
	Port     uint16 `toml:"port"`
	DbName   string `toml:"db-name"`
	User     string `toml:"user"`
	Password string `toml:"password"`
}

func (p *PostgreSql) Open(config *gorm.Config) (string, *gorm.DB, error) {
	slog.Info("open postgresql", "host", p.Host, "port", p.Port, "db-name", p.DbName, "user", p.User)
	db, err := gorm.Open(postgres.New(postgres.Config{
		DSN:                  fmt.Sprintf("user=%s password=%s dbname=%s host=%s port=%d sslmode=disable TimeZone=UTC", p.User, p.Password, p.DbName, p.Host, p.Port),
		PreferSimpleProtocol: true,
	}), config)
	if err != nil {
		return "", nil, err
	}
	var it databaseVersion
	if err := db.Raw("SELECT VERSION()").Scan(&it).Error; err != nil {
		return "", nil, err
	}
	return it.Version, db, nil
}

type MySql struct {
	Host     string `toml:"host"`
	Port     uint16 `toml:"port"`
	DbName   string `toml:"db-name"`
	User     string `toml:"user"`
	Password string `toml:"password"`
}

func (p *MySql) Open(config *gorm.Config) (string, *gorm.DB, error) {
	// https://github.com/go-sql-driver/mysql#dsn-data-source-name
	slog.Info("open mysql", "host", p.Host, "port", p.Port, "db-name", p.DbName, "user", p.User)
	db, err := gorm.Open(
		mysql.Open(fmt.Sprintf("%s:%s@tcp(%s:%d)/%s?charset=utf8mb4&parseTime=True&loc=Local", p.User, p.Password, p.Host, p.Port, p.DbName)),
		config)
	if err != nil {
		return "", nil, err
	}
	var it databaseVersion
	if err := db.Raw("SELECT VERSION()").Scan(&it).Error; err != nil {
		return "", nil, err
	}
	return it.Version, db, nil
}

type SqlServer struct {
	Host     string `toml:"host"`
	Port     uint16 `toml:"port"`
	DbName   string `toml:"db-name"`
	User     string `toml:"user"`
	Password string `toml:"password"`
}

func (p *SqlServer) Open(config *gorm.Config) (string, *gorm.DB, error) {
	slog.Info("open sqlserver", "host", p.Host, "port", p.Port, "db-name", p.DbName, "user", p.User)
	db, err := gorm.Open(
		sqlserver.Open(fmt.Sprintf("sqlserver://%s:%s@%s:%d?database=%s", p.User, p.Password, p.Host, p.Port, p.DbName)),
		config)
	if err != nil {
		return "", nil, err
	}
	var it databaseVersion
	if err := db.Raw("SELECT VERSION()").Scan(&it).Error; err != nil {
		return "", nil, err
	}
	return it.Version, db, nil
}

type GoogleOauth2 struct {
	ClientId     string `toml:"client-id"`
	ClientSecret string `toml:"client-secret"`
}

func (p *GoogleOauth2) Open(home string) *oauth2.Config {
	return &oauth2.Config{
		ClientID:     p.ClientId,
		ClientSecret: p.ClientSecret,
		RedirectURL:  fmt.Sprintf("%s/callback/google/oauth2", home),
		Scopes: []string{
			// "https://www.googleapis.com/auth/bigquery",
			// "https://www.googleapis.com/auth/blogger",
			"https://www.googleapis.com/auth/userinfo.email",
			"https://www.googleapis.com/auth/userinfo.profile",
		},
		Endpoint: google.Endpoint,
	}
}
