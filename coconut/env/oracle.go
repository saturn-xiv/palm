package env

import (
	"errors"
	"fmt"
	"log/slog"
	"os"
	"os/exec"
	"path/filepath"
)

type Oracle struct {
	home           string
	sid            string
	user           string
	password       string
	directory_path string
}

func NewOracle(home string, sid string, user string, password string, directory_path string) (*Oracle, error) {
	{
		fi, err := os.Stat(filepath.Join(home, "bin", "impdp"))
		if err != nil {
			return nil, err
		}
		if fi.IsDir() {
			return nil, errors.New("bad oracle home")
		}
	}
	if sid == "" {
		return nil, errors.New("empty SID")
	}
	if user == "" {
		return nil, errors.New("empty user")
	}

	return &Oracle{home: home, sid: sid, directory_path: directory_path, user: user, password: password}, nil
}

func (p *Oracle) Dump(target string) []*exec.Cmd {
	slog.Info("backup oracle", slog.String("sid", p.sid), slog.String("user", p.user))
	slog.Warn(fmt.Sprintf(`
Please make sure:
create or replace directory datapump_dir as '%s';
grant read,write on directory datapump_dir to %s;
grant datapump_exp_full_database to %s;
grant datapump_imp_full_database to %s;
`, p.directory_path, p.user, p.user, p.user))

	exp := exec.Command(
		filepath.Join(p.home, "bin", "expdp"),
		fmt.Sprintf("%s/%s", p.user, p.password),
		"full=Y",
		fmt.Sprintf("dumpfile=%s.dmp", p.user),
		fmt.Sprintf("logfile=%s-exp.log", p.user),
	)
	exp.Env = append(os.Environ(),
		fmt.Sprintf("ORACLE_HOME=%s", p.home),
		fmt.Sprintf("ORACLE_SID=%s", p.sid),
		"NLS_LANG=AMERICAN_AMERICA.AL32UTF8",
		"LANG=en_US.utf8",
	)
	return []*exec.Cmd{
		exp,
		exec.Command("mv", filepath.Join(p.directory_path, fmt.Sprintf("%s.dmp", p.user)), filepath.Join(p.directory_path, fmt.Sprintf("%s-exp.log", p.user)), target),
	}
}
func (p *Oracle) Restore() []*exec.Cmd {
	return []*exec.Cmd{
		exec.Command("mv", fmt.Sprintf("%s.dmp", p.user), p.directory_path),
		exec.Command(
			filepath.Join(p.home, "bin", "impdp"),
			fmt.Sprintf("%s/CHANGE-ME", p.user),
			fmt.Sprintf("dumpfile=%s.dmp", p.user),
			fmt.Sprintf("logfile=%s.exp.log", p.user),
			fmt.Sprintf("schemas=%s", p.user),
			fmt.Sprintf("REMAP_SCHEMA=%s:NEW-USER", p.user),
		),
	}
}
