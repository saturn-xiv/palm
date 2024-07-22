package env

import (
	"bytes"
	"database/sql"
	"errors"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"text/template"
	"time"

	"github.com/go-playground/validator/v10"
)

var validate *validator.Validate

type Migration struct {
	Up      string
	Down    string
	Name    string
	Version string
	RunAt   *time.Time
}

type Database interface {
	Version() string
	CreateSchemaMigrationsTable() string
	DropSchemaMigrationsTable() string
	HibernateSequence() (string, string)
	ByVersion() string
	Insert() string
	Up() string
	Down() string
	All() string
	Latest() string
}

func New(migrations_dir string, name string) error {
	if err := validate.Var(name, "required,alphanumunicode,min=2,max=255"); err != nil {
		return err
	}
	root := filepath.Join(migrations_dir, time.Now().Format("20060102150405")+"-"+name)
	{
		slog.Debug("create folder", slog.String("path", root))
		os.MkdirAll(root, 0700)
	}
	{
		f := filepath.Join(root, "up.sql")
		slog.Debug("create", slog.String("file", f))
		fd, err := os.Create(f)
		if err != nil {
			return err
		}
		defer fd.Close()
		fd.WriteString(fmt.Sprintf("CREATE TABLE IF NOT EXISTS %s(id INTEGER);", name))
	}
	{
		f := filepath.Join(root, "down.sql")
		slog.Debug("create", slog.String("file", f))
		fd, err := os.Create(f)
		if err != nil {
			return err
		}
		defer fd.Close()
		fd.WriteString(fmt.Sprintf("DROP TABLE IF EXISTS %s;", name))
	}

	return nil
}

func Create() string {
	return `
PostgreSql:
CREATE USER user-name WITH PASSWORD 'change-me';
CREATE DATABASE db-name WITH ENCODING = 'UTF8' OWNER user-name;

DM8:
CREATE TABLESPACE "demo" DATAFILE '/var/lib/dm8/demo.dbf' SIZE 10240;
CREATE USER "www" IDENTIFIED BY "change-ME.2024" HASH WITH SHA512 SALT ENCRYPT BY "123456" DEFAULT TABLESPACE "demo" DEFAULT INDEX TABLESPACE "demo";
GRANT "DBA" TO "www";
`
}

func Drop() string {
	return `
PostgreSql/MySql:
DROP DATABASE db-name;

DM8:
DROP TABLESPACE "demo";
`
}

func Dump(url string, schema_file string) error {
	// TODO
	return nil
}

func Load(url string, schema_file string) error {
	// TODO
	return nil
}

func Migrate(url string, migrations_dir string, migrations_table string) error {
	driver, db, err := Open(url)
	if err != nil {
		return err
	}

	tx, err := db.Begin()
	if err != nil {
		return err
	}
	if err = migrate(tx, driver, migrations_dir, migrations_table); err != nil {
		return err
	}
	if err = tx.Commit(); err != nil {
		return err
	}
	slog.Info("done.")
	return nil
}

func Rollback(url string, migrations_table string) error {
	driver, db, err := Open(url)
	if err != nil {
		return err
	}

	tx, err := db.Begin()
	if err != nil {
		return err
	}
	item, err := select_latest(tx, driver, migrations_table)
	switch {
	case err == sql.ErrNoRows:
		slog.Warn("empty database")
		return nil
	case err != nil:
		return err
	default:
		if err = item.Rollback(tx, driver, migrations_table); err != nil {
			return err
		}
	}

	if err = tx.Commit(); err != nil {
		return err
	}
	slog.Info("done.")
	return nil
}
func Clear(url string, migrations_dir string, migrations_table string) error {
	driver, db, err := Open(url)
	if err != nil {
		return err
	}

	tx, err := db.Begin()
	if err != nil {
		return err
	}
	if err = clear(tx, driver, migrations_table); err != nil {
		return err
	}
	if err = tx.Commit(); err != nil {
		return err
	}
	slog.Info("done.")
	return nil
}
func Reset(url string, migrations_dir string, migrations_table string) error {
	driver, db, err := Open(url)
	if err != nil {
		return err
	}

	tx, err := db.Begin()
	if err != nil {
		return err
	}

	if err = clear(tx, driver, migrations_table); err != nil {
		return err
	}
	if err = migrate(tx, driver, migrations_dir, migrations_table); err != nil {
		return err
	}

	if err = tx.Commit(); err != nil {
		return err
	}
	slog.Info("done.")
	return nil
}

func Status(url string, migrations_dir string, migrations_table string) error {
	driver, db, err := Open(url)
	if err != nil {
		return err
	}

	{
		slog.Debug("ping database")
		if err = db.Ping(); err != nil {
			return err
		}
	}
	tx, err := db.Begin()
	if err != nil {
		return err
	}
	defer tx.Rollback()

	if err = check_migrations_table(tx, driver, migrations_table); err != nil {
		return err
	}
	if err = check_migrations_dir(tx, driver, migrations_dir, migrations_table); err != nil {
		return err
	}

	{
		tpl, err := template.New("").Parse(driver.Version())
		if err != nil {
			return err
		}
		var buf bytes.Buffer
		if err = tpl.Execute(&buf, map[string]interface{}{"name": migrations_table}); err != nil {
			return err
		}
		sq := buf.String()
		slog.Debug(sq)

		var version string
		if err = tx.QueryRow(sq).Scan(&version); err != nil {
			return err
		}
		fmt.Println(version)
	}

	items, err := select_all(tx, driver, migrations_table)
	if err != nil {
		return err
	}

	fmt.Printf("%-15s %-32s\t%s\n", "VERSION", "NAME", "RUN AT")
	for _, it := range items {
		run_at := "n/a"
		if it.RunAt != nil {
			run_at = it.RunAt.Format(time.UnixDate)
		}
		fmt.Printf("%-15s %-32s\t%s\n", it.Version, it.Name, run_at)
	}
	if err = tx.Commit(); err != nil {
		return err
	}
	return nil
}

func Open(url string) (Database, *sql.DB, error) {
	if dsn, ok := strings.CutPrefix(url, "sqlite3://"); ok {
		slog.Debug("open sqlite3", slog.String("dsn", dsn))
		db, err := sql.Open("sqlite3", dsn)
		if err != nil {
			return nil, nil, err
		}
		return &Sqlite3{}, db, nil
	}
	if dsn, ok := strings.CutPrefix(url, "mysql://"); ok {
		slog.Debug("open mysql", slog.String("dsn", dsn))
		db, err := sql.Open("mysql", dsn)
		if err != nil {
			return nil, nil, err
		}
		return &MySql{}, db, nil
	}
	if strings.HasPrefix(url, "postgres://") {
		slog.Debug("open postgresql", slog.String("dsn", url))
		db, err := sql.Open("postgres", url)
		if err != nil {
			return nil, nil, err
		}
		return &PostgreSql{}, db, nil
	}
	if strings.HasPrefix(url, "sqlserver://") {
		slog.Debug("open sqlserver", slog.String("dsn", url))
		db, err := sql.Open("sqlserver", url)
		if err != nil {
			return nil, nil, err
		}
		return &SqlServer{}, db, nil
	}
	if strings.HasPrefix(url, "oracle://") {
		slog.Debug("open oracle", slog.String("dsn", url))
		// TODO
		db, err := sql.Open("oracle", url)
		if err != nil {
			return nil, nil, err
		}
		return &Oracle{}, db, nil
	}
	if strings.HasPrefix(url, "dm://") {
		slog.Debug("open dm8", slog.String("dsn", url))
		db, err := sql.Open("dm", url)
		if err != nil {
			return nil, nil, err
		}
		return &DM8{}, db, nil
	}
	return nil, nil, fmt.Errorf("unsupported %s", url)
}

func init() {
	validate = validator.New(validator.WithRequiredStructEnabled())
}

func (p *Migration) Rollback(tx *sql.Tx, driver Database, migrations_table string) error {
	slog.Info("rollback", slog.String("version", p.Version), slog.String("name", p.Name))
	slog.Debug(p.Down)
	st, err := tx.Prepare(p.Down)
	if err != nil {
		return err
	}
	if _, err := st.Exec(); err != nil {
		return err
	}

	{
		tpl, err := template.New("").Parse(driver.Down())
		if err != nil {
			return err
		}
		var buf bytes.Buffer
		if err = tpl.Execute(&buf, map[string]interface{}{"name": migrations_table}); err != nil {
			return err
		}
		sq := buf.String()
		slog.Debug(sq)
		st, err := tx.Prepare(sq)
		if err != nil {
			return err
		}
		if _, err = st.Exec(p.Version); err != nil {
			return err
		}
	}
	return nil
}
func (p *Migration) Migrate(tx *sql.Tx, driver Database, migrations_table string) error {
	slog.Info("migrate", slog.String("version", p.Version), slog.String("name", p.Name))
	slog.Debug(p.Up)
	st, err := tx.Prepare(p.Up)
	if err != nil {
		return err
	}
	if _, err := st.Exec(); err != nil {
		return err
	}

	{
		tpl, err := template.New("").Parse(driver.Up())
		if err != nil {
			return err
		}
		var buf bytes.Buffer
		if err = tpl.Execute(&buf, map[string]interface{}{"name": migrations_table}); err != nil {
			return err
		}
		sq := buf.String()
		slog.Debug(sq)
		st, err := tx.Prepare(sq)
		if err != nil {
			return err
		}
		if _, err = st.Exec(p.Version); err != nil {
			return err
		}
	}
	return nil
}

func check_hibernate_sequence(driver Database, migrations_dir string) error {
	slog.Debug("check hibernate sequence")
	root := filepath.Join(migrations_dir, "00000000000000-CreateHibernateSequence")
	{
		fi, err := os.Stat(root)
		switch {
		case errors.Is(err, os.ErrNotExist):
			slog.Debug("create", slog.String("folder", root))
			if err = os.MkdirAll(root, 0700); err != nil {
				return err
			}
		case err == nil:
			if !fi.IsDir() {
				return fmt.Errorf("%s isn't a folder", root)
			}
		default:
			return err
		}
	}
	up, down := driver.HibernateSequence()
	{
		file := filepath.Join(root, "up.sql")
		fi, err := os.Stat(file)
		switch {
		case errors.Is(err, os.ErrNotExist):
			slog.Info("generate", slog.String("file", file))
			fd, err := os.Create(file)
			if err != nil {
				return err
			}
			if _, err = fd.WriteString(up); err != nil {
				return err
			}
		case err == nil:
			if fi.IsDir() {
				return fmt.Errorf("%s is a folder", file)
			}
		default:
			return err
		}
	}
	{
		file := filepath.Join(root, "down.sql")
		fi, err := os.Stat(file)
		switch {
		case errors.Is(err, os.ErrNotExist):
			slog.Info("generate", slog.String("file", file))
			fd, err := os.Create(file)
			if err != nil {
				return err
			}
			if _, err = fd.WriteString(down); err != nil {
				return err
			}
		case err == nil:
			if fi.IsDir() {
				return fmt.Errorf("%s is a folder", file)
			}
		default:
			return err
		}
	}
	return nil
}
func check_migrations_dir(tx *sql.Tx, driver Database, migrations_dir string, migrations_table string) error {
	if err := check_hibernate_sequence(driver, migrations_dir); err != nil {
		return err
	}

	items, err := os.ReadDir(migrations_dir)
	if err != nil {
		return err
	}
	for _, it := range items {
		if !it.IsDir() {
			return fmt.Errorf("%s isn't a folder", it.Name())
		}
		ss := strings.Split(it.Name(), "-")
		if len(ss) != 2 {
			return fmt.Errorf("bad migration name: %s", it.Name())
		}
		version := ss[0]
		name := ss[1]
		slog.Info("find migration", slog.String("version", version), slog.String("name", name))

		up_b, err := os.ReadFile(filepath.Join(migrations_dir, it.Name(), "up.sql"))
		if err != nil {
			return err
		}
		up := string(up_b)
		down_b, err := os.ReadFile(filepath.Join(migrations_dir, it.Name(), "down.sql"))
		if err != nil {
			return err
		}
		down := string(down_b)

		item, err := select_by_version(tx, driver, migrations_table, version)
		switch {
		case err == sql.ErrNoRows:
			slog.Debug("insert migrations", slog.String("version", version), slog.String("name", name))
			tpl, err := template.New("").Parse(driver.Insert())
			if err != nil {
				return err
			}
			var buf bytes.Buffer
			if err = tpl.Execute(&buf, map[string]interface{}{"name": migrations_table}); err != nil {
				return err
			}
			sq := buf.String()
			slog.Debug(sq)
			st, err := tx.Prepare(sq)
			if err != nil {
				return err
			}
			if _, err = st.Exec(version, name, up, down); err != nil {
				return err
			}
		case err != nil:
			return err
		default:
			if item.Name != name || item.Up != up || item.Down != down {
				return fmt.Errorf("migration %s-%s is dirty", version, name)
			}
		}

	}
	return nil
}

func select_all(tx *sql.Tx, driver Database, migrations_table string) ([]Migration, error) {
	tpl, err := template.New("").Parse(driver.All())
	if err != nil {
		return nil, err
	}
	var buf bytes.Buffer
	if err = tpl.Execute(&buf, map[string]interface{}{"name": migrations_table}); err != nil {
		return nil, err
	}
	sq := buf.String()
	slog.Debug(sq)
	rows, err := tx.Query(sq)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var items []Migration

	for rows.Next() {
		var it Migration
		if err = rows.Scan(&it.Version, &it.Name, &it.Up, &it.Down, &it.RunAt); err != nil {
			return nil, err
		}
		items = append(items, it)
	}
	return items, nil
}
func select_latest(tx *sql.Tx, driver Database, migrations_table string) (*Migration, error) {
	tpl, err := template.New("").Parse(driver.Latest())
	if err != nil {
		return nil, err
	}
	var buf bytes.Buffer
	if err = tpl.Execute(&buf, map[string]interface{}{"name": migrations_table}); err != nil {
		return nil, err
	}
	sq := buf.String()
	slog.Debug(sq)
	var it Migration

	if err = tx.QueryRow(sq).Scan(&it.Version, &it.Name, &it.Up, &it.Down, &it.RunAt); err != nil {
		return nil, err
	}
	return &it, nil
}
func select_by_version(tx *sql.Tx, driver Database, migrations_table string, version string) (*Migration, error) {
	tpl, err := template.New("").Parse(driver.ByVersion())
	if err != nil {
		return nil, err
	}
	var buf bytes.Buffer
	if err = tpl.Execute(&buf, map[string]interface{}{"name": migrations_table}); err != nil {
		return nil, err
	}
	sq := buf.String()
	slog.Debug(sq)
	var it Migration

	if err = tx.QueryRow(sq, version).Scan(&it.Version, &it.Name, &it.Up, &it.Down, &it.RunAt); err != nil {
		return nil, err
	}
	return &it, nil
}

func check_migrations_table(tx *sql.Tx, driver Database, migrations_table string) error {

	slog.Debug("check migrations table", slog.String("name", migrations_table))
	tpl, err := template.New("").Parse(driver.CreateSchemaMigrationsTable())
	if err != nil {
		return err
	}
	var buf bytes.Buffer
	if err = tpl.Execute(&buf, map[string]interface{}{"name": migrations_table}); err != nil {
		return err
	}
	sq := buf.String()

	slog.Debug(sq)
	st, err := tx.Prepare(sq)
	if err != nil {
		return err
	}
	if _, err = st.Exec(); err != nil {
		return err
	}

	return nil
}

func clear(tx *sql.Tx, driver Database, migrations_table string) error {
	{
		slog.Warn("clear the database schema")
	L:
		for {
			item, err := select_latest(tx, driver, migrations_table)
			switch {
			case err == sql.ErrNoRows:
				break L
			case err != nil:
				return err
			default:
				if err = item.Rollback(tx, driver, migrations_table); err != nil {
					return err
				}
			}

		}
	}

	{
		slog.Debug("drop the migrations table", slog.String("name", migrations_table))
		tpl, err := template.New("").Parse(driver.DropSchemaMigrationsTable())
		if err != nil {
			return err
		}
		var buf bytes.Buffer
		if err = tpl.Execute(&buf, map[string]interface{}{"name": migrations_table}); err != nil {
			return err
		}
		sq := buf.String()
		slog.Debug(sq)
		st, err := tx.Prepare(sq)
		if err != nil {
			return err
		}
		if _, err = st.Exec(); err != nil {
			return err
		}
	}

	return nil
}

func migrate(tx *sql.Tx, driver Database, migrations_dir string, migrations_table string) error {
	if err := check_migrations_table(tx, driver, migrations_table); err != nil {
		return err
	}
	if err := check_migrations_dir(tx, driver, migrations_dir, migrations_table); err != nil {
		return err
	}
	{
		items, err := select_all(tx, driver, migrations_table)
		if err != nil {
			return err
		}
		for _, it := range items {
			if it.RunAt == nil {
				if err = it.Migrate(tx, driver, migrations_table); err != nil {
					return err
				}
			}
		}
	}
	return nil
}
