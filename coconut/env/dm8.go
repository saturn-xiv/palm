package env

import (
	"errors"
	"fmt"
	"os/exec"
	"path/filepath"
)

// https://eco.dameng.com/document/dm/zh-cn/ops/logical-backup-restore.html
type Dm8 struct {
	home     string
	user     string
	password string
}

func NewDm8(home string, user string, password string) (*Dm8, error) {
	if home == "" {
		return nil, errors.New("empty database home")
	}
	if user == "" {
		return nil, errors.New("empty user")
	}
	if password == "" {
		return nil, errors.New("empty password")
	}
	return &Dm8{home: home, user: user, password: password}, nil
}

func (p *Dm8) Dump(target string) []*exec.Cmd {
	return []*exec.Cmd{
		exec.Command(
			filepath.Join(p.home, "bin", "dexp"),
			fmt.Sprintf("USERID=%s/%s", p.user, p.password),
			fmt.Sprintf("FILE=%s.dmp", p.user),
			fmt.Sprintf("LOG=%s.log", p.user),
			fmt.Sprintf("OWNER=%s", p.user),
			fmt.Sprintf("DIRECTORY=%s", target),
		),
	}
}
func (p *Dm8) Restore() []*exec.Cmd {
	return []*exec.Cmd{
		exec.Command(
			filepath.Join(p.home, "bin", "dimp"),
			fmt.Sprintf("USERID=%s/%s", p.user, p.password),
			fmt.Sprintf("FILE=%s.dmp", p.user),
			fmt.Sprintf("LOG=%s.log", p.user),
			fmt.Sprintf("OWNER=%s", p.user),
			"DIRECTORY=/tmp",
		),
	}
}
