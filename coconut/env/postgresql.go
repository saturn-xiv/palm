package env

import (
	"errors"
	"fmt"
	"log/slog"
	"os/exec"
	"path/filepath"
	"strconv"
)

type PostgreSql struct {
	host     string
	port     uint16
	db_name  string
	user     string
	password string
}

func NewPostgreSql(host string, port uint16, db_name string, user string, password string) (*PostgreSql, error) {
	if host == "" {
		return nil, errors.New("empty host")
	}
	if db_name == "" {
		return nil, errors.New("empty db name")
	}

	return &PostgreSql{host: host, port: port, db_name: db_name, user: user, password: password}, nil
}
func (p *PostgreSql) Dump(target string) []*exec.Cmd {
	slog.Info("backup postgresql", slog.String("host", p.host), slog.Int("port", int(p.port)), slog.String("user", p.user), slog.String("db-name", p.db_name))
	url := fmt.Sprintf("postgresql://%s:%d@%s:%s/%s?sslmode=disable", p.host, p.port, p.user, p.password, p.db_name)
	return []*exec.Cmd{
		exec.Command("pg_dump", "-O", "-s", "-w", "-d", url, "-f", filepath.Join(target, fmt.Sprintf("%s-scheme.sql", p.db_name))),
		exec.Command("pg_dump", "-Fc", "-O", "-a", "-w", url, "-f", filepath.Join(target, fmt.Sprintf("%s-data.dump", p.db_name))),
	}
}
func (p *PostgreSql) Restore() []*exec.Cmd {
	return []*exec.Cmd{
		exec.Command("psql", "-h", p.host, "-p", strconv.Itoa(int(p.port)), "-U", p.user, "-d", p.db_name, "<", fmt.Sprintf("%s-scheme.sql", p.db_name)),
		exec.Command("pg_restore", "-Fc", "-h", p.host, "-p", strconv.Itoa(int(p.port)), "-U", p.user, "-d", p.db_name, "<", fmt.Sprintf("%s-data.dump", p.db_name)),
	}
}
