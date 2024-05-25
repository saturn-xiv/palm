package env

import (
	"bytes"
	"database/sql"
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
	Open(dsn string) (*sql.DB, error)
	Version() string
	Create() string
	ByVersion() string
	Insert() string
	Up() string
	Down() string
	All() string
	Latest() string
}

func New(migrations_dir string, name string) error {
	if err := validate.Var(name, "required,alphanum,min=2,max=63"); err != nil {
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

	slog.Info("done.")
	return nil
}

func Create(url string) (string, error) {
	// TODO
	return "", nil
}

func Drop(url string) (string, error) {
	// TODO
	return "", nil
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
	driver, err := Open(url)
	if err != nil {
		return err
	}
	db, err := driver.Open(url)
	if err != nil {
		return err
	}
	tx, err := db.Begin()
	if err != nil {
		return err
	}
	if err = check_migrations_table(tx, driver, migrations_table); err != nil {
		return err
	}
	if err = check_migrations_dir(tx, driver, migrations_dir, migrations_table); err != nil {
		return err
	}
	{
		items, err := select_all(tx, driver, migrations_table)
		if err != nil {
			return err
		}
		for _, it := range items {
			if it.RunAt == nil {
				it.Migrate(tx, driver, migrations_table)
			}
		}
	}

	if err = tx.Commit(); err != nil {
		return err
	}
	slog.Info("done.")
	return nil
}

func Rollback(url string, migrations_table string) error {
	driver, err := Open(url)
	if err != nil {
		return err
	}
	db, err := driver.Open(url)
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
	return nil
}

func Status(url string, migrations_table string) error {
	driver, err := Open(url)
	if err != nil {
		return err
	}
	db, err := driver.Open(url)
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
		return nil
	}
	fmt.Println("VERSION NAME\tRUN AT")
	for _, it := range items {
		run_at := "n/a"
		if it.RunAt != nil {
			run_at = it.RunAt.Format(time.UnixDate)
		}
		fmt.Printf("%s %s\t%s\n", it.Version, it.Name, run_at)
	}
	return nil
}

func Open(url string) (Database, error) {
	if strings.HasPrefix(url, "postgres://") {
		return &PostgreSql{}, nil
	}
	if strings.HasPrefix(url, "dm://") {
		return &DM8{}, nil
	}
	return nil, fmt.Errorf("unsupported %s", url)
}

func init() {
	validate = validator.New(validator.WithRequiredStructEnabled())
}

func (p *Migration) Rollback(tx *sql.Tx, driver Database, migrations_table string) error {
	slog.Info("rollback", slog.String("version", p.Version), slog.String("name", p.Name))
	slog.Debug(p.Down)
	if _, err := tx.Exec(p.Down); err != nil {
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
		if _, err = tx.Exec(sq, p.Version); err != nil {
			return err
		}
	}
	return nil
}
func (p *Migration) Migrate(tx *sql.Tx, driver Database, migrations_table string) error {
	slog.Info("migrate", slog.String("version", p.Version), slog.String("name", p.Name))
	slog.Debug(p.Up)
	if _, err := tx.Exec(p.Up); err != nil {
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
		if _, err = tx.Exec(sq, p.Version); err != nil {
			return err
		}
	}
	return nil
}

func check_migrations_dir(tx *sql.Tx, driver Database, migrations_dir string, migrations_table string) error {
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
			slog.Info("insert migrations", slog.String("version", version), slog.String("name", name))
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
			if _, err = tx.Exec(sq, version, name, up, down); err != nil {
				return err
			}
		case err != nil:
			return err
		default:
			if item.Name != name || item.Up != up || item.Down != down {
				return fmt.Errorf("migration %s is dirty", version)
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
	tpl, err := template.New("").Parse(driver.Create())
	if err != nil {
		return err
	}
	var buf bytes.Buffer
	if err = tpl.Execute(&buf, map[string]interface{}{"name": migrations_table}); err != nil {
		return err
	}
	sq := buf.String()
	slog.Debug(sq)
	if _, err = tx.Exec(sq); err != nil {
		return err
	}
	return nil
}
