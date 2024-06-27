package env

import (
	"errors"
	"fmt"
	"os/exec"
	"path/filepath"
	"strconv"
)

type MySql struct {
	host     string
	port     uint16
	db_name  string
	user     string
	password string
}

func NewMySql(host string, port uint16, db_name string, user string, password string) (*MySql, error) {
	if host == "" {
		return nil, errors.New("empty host")
	}
	if db_name == "" {
		return nil, errors.New("empty db name")
	}
	if user == "" {
		return nil, errors.New("empty user")
	}

	return &MySql{host: host, port: port, db_name: db_name, user: user, password: password}, nil
}
func (p *MySql) Dump(target string) []*exec.Cmd {
	args := []string{"--no-create-db", "-h", p.host, "-P", strconv.Itoa(int(p.port)), "-u", p.user}
	if p.password != "" {
		args = append(args, fmt.Sprintf("-p%s", p.password))
	}
	args = append(args, "-r", filepath.Join(target, fmt.Sprintf("%s.sql", p.db_name)), p.db_name)
	return []*exec.Cmd{
		exec.Command("mysqldump", args...),
	}
}
func (p *MySql) Restore() []*exec.Cmd {
	return []*exec.Cmd{
		exec.Command("mysql", "-h", p.host, "-P", strconv.Itoa(int(p.port)), "-u", p.user, "-p", p.db_name, "<", fmt.Sprintf("%s.sql", p.db_name)),
	}
}
