package env

import "time"

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
