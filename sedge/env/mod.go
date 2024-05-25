package env

import (
	"log/slog"
	"path/filepath"
	"time"
)

type Migration struct {
	Up      string
	Down    string
	Name    string
	Version time.Time
	RunAt   *time.Time
}

type Database interface {
	Name() string
	Execute(sql string) error
	Up(version string) error
	Down(version string) error
	Status() ([]Migration, error)
	Latest() (*Migration, error)
}

func New(migrations_dir string, name string) error {
	root := filepath.Join(migrations_dir, name)
	slog.Info("create migration", slog.String("path", root))
	return nil
}
