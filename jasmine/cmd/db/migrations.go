package db

import (
	"embed"
	"errors"
	"fmt"
	"log/slog"
	"net/url"
	"path"

	"github.com/BurntSushi/toml"
	"github.com/amacneil/dbmate/v2/pkg/dbmate"
	_ "github.com/amacneil/dbmate/v2/pkg/driver/mysql"
	_ "github.com/amacneil/dbmate/v2/pkg/driver/postgres"
	_ "github.com/amacneil/dbmate/v2/pkg/driver/sqlite"

	"github.com/saturn-xiv/palm/jasmine/env"
)

//go:embed postgresql/migrations/*.sql
var fs_postgresql_migrations embed.FS

//go:embed mysql/migrations/*.sql
var fs_mysql_migrations embed.FS

//go:embed postgresql/migrations/*.sql
var fs_sqlite3_migrations embed.FS

func Migrate(config_file string) error {
	slog.Debug("load configuration from", slog.String("file", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	db, err := config.open_dbmate()
	if err != nil {
		return err
	}
	return db.Migrate()
}

func Rollback(config_file string) error {
	slog.Debug("load configuration from", slog.String("file", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	db, err := config.open_dbmate()
	if err != nil {
		return err
	}
	return db.Rollback()
}

func Status(config_file string) error {
	slog.Debug("load configuration from", slog.String("file", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	db, err := config.open_dbmate()
	if err != nil {
		return err
	}
	// items, err := db.FindMigrations()
	// if err != nil {
	// 	return err
	// }
	// fmt.Println("VERSION", "FILENAME", "APPLIED")
	// for _, m := range items {
	// 	if m.Applied {
	// 		fmt.Println(m.Version, m.FileName, "Y")
	// 	} else {
	// 		fmt.Println(m.Version, m.FileName, "N")
	// 	}
	// }
	_, err = db.Status(false)
	return err
}

func (p *Config) dbmate_url() (string, error) {

	if len(p.Database.PostgreSql.DbName) > 0 {
		return fmt.Sprintf(
			"postgres://%s:%s@%s:%d/%s?sslmode=disable",
			p.Database.PostgreSql.User,
			p.Database.PostgreSql.Password,
			p.Database.PostgreSql.Host,
			p.Database.PostgreSql.Port,
			p.Database.PostgreSql.DbName,
		), nil
	}
	if len(p.Database.MySql.DbName) > 0 {
		return p.Database.MySql.Url(), nil
	}

	if len(p.Database.Sqlite3.File) > 0 {
		return fmt.Sprintf("sqlite:%s", p.Database.Sqlite3.File), nil
	}
	return "", errors.New("unsupported database")
}

func (p *Config) open_dbmate() (*dbmate.DB, error) {
	s, err := p.dbmate_url()
	if err != nil {
		return nil, err
	}
	u, err := url.Parse(s)
	if err != nil {
		return nil, err
	}
	db := dbmate.New(u)
	db.Verbose = true
	db.AutoDumpSchema = false
	db.MigrationsTableName = fmt.Sprintf("%s_schema_migrations", env.PLUGIN_NAME)

	switch db.DatabaseURL.Scheme {
	case "postgres":
		db.FS = fs_postgresql_migrations
		db.MigrationsDir = []string{path.Join("postgresql", "migrations")}
	case "mysql":
		db.FS = fs_mysql_migrations
		db.MigrationsDir = []string{path.Join("mysql", "migrations")}
	case "sqlite":
		db.FS = fs_sqlite3_migrations
		db.MigrationsDir = []string{path.Join("sqlite3", "migrations")}
	default:
		slog.Warn("Unsupported migration driver", slog.String("schema", u.Scheme))
	}

	return db, nil
}
