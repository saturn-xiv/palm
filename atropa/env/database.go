package env

import (
	"embed"
	"fmt"
	"log/slog"
	"net/url"

	"github.com/amacneil/dbmate/v2/pkg/dbmate"
	_ "github.com/amacneil/dbmate/v2/pkg/driver/mysql"
	_ "github.com/amacneil/dbmate/v2/pkg/driver/postgres"
	_ "github.com/amacneil/dbmate/v2/pkg/driver/sqlite"
	"gorm.io/gorm"
)

type Database struct {
	PostgreSql PostgreSql `toml:"postgresql,omitempty"`
	MySql      MySql      `toml:"mysql,omitempty"`
	SqlServer  SqlServer  `toml:"sqlserver,omitempty"`
}

func (p *Database) Open() (*gorm.DB, error) {
	config := gorm.Config{
		Logger:      &gormLogger{},
		PrepareStmt: true,
	}
	if len(p.PostgreSql.DbName) > 0 {
		return p.PostgreSql.Open(&config)
	}
	if len(p.MySql.DbName) > 0 {

		return p.MySql.Open(&config)
	}
	if len(p.SqlServer.DbName) > 0 {
		return p.SqlServer.Open(&config)
	}
	it := Sqlite3{File: "tmp/db"}
	return it.Open(&config)
}

func db_migrate(dbu string, fs embed.FS) error {
	url, err := url.Parse(dbu)
	if err != nil {
		return err
	}
	db := dbmate.New(url)
	db.FS = fs
	db.MigrationsDir = []string{fmt.Sprintf("db/%s/migrations", url.Scheme)}

	migrations, err := db.FindMigrations()
	if err != nil {
		return err
	}
	for _, it := range migrations {
		slog.Debug("found migration", slog.String("version", it.Version), slog.String("file", it.FilePath))
	}
	if err = db.Migrate(); err != nil {
		return err
	}
	return nil
}
