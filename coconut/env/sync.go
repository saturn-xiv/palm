package env

import (
	"errors"
	"fmt"
	"log/slog"
	"os"
	"os/exec"
)

type Sync struct {
	host     string
	port     uint16
	user     string
	password string
	key_file string
	source   string
}

func NewSync(host string, port uint16, user string, password string, key_file string, source string) (*Sync, error) {
	if source == "" {
		return nil, errors.New("empty source path")
	}
	if host != "" && password == "" {
		fi, err := os.Stat(key_file)
		if err != nil {
			return nil, err
		}
		if fi.IsDir() {
			return nil, errors.New("bad ssh private key file")
		}
	}
	return &Sync{host: host, port: port, user: user, password: password, key_file: key_file, source: source}, nil
}

func (p *Sync) Dump(target string, keep uint) []*exec.Cmd {
	if p.host == "" {
		slog.Info("backup ", slog.String("path", p.source))
		return []*exec.Cmd{exec.Command("cp", "-a", p.source, target)}
	}
	slog.Info("backup ", slog.String("user", p.user), slog.String("host", p.host), slog.Int("port", int(p.port)), slog.String("path", p.source))
	if p.password == "" {
		return []*exec.Cmd{
			exec.Command(
				"rsync", "-a", "-z", "-q", "-e",
				fmt.Sprintf("ssh -o StrictHostKeyChecking=yes -p %d -l %s -i %s", p.port, p.user, p.key_file),
				fmt.Sprintf("%s:%s", p.host, p.source),
			),
		}
	}
	return []*exec.Cmd{
		exec.Command(
			"sshpass", "-p", p.password, "rsync", "-a", "-z", "-q", "-e",
			fmt.Sprintf("ssh -o StrictHostKeyChecking=yes -p %d -l %s", p.port, p.user),
			fmt.Sprintf("%s:%s", p.host, p.source),
		),
	}
}
func (p *Sync) Restore() []*exec.Cmd {
	return []*exec.Cmd{}
}
