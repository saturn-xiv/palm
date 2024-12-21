package hibiscus

import (
	"log/slog"
	"os/exec"
)

func Shell(dir string, command string, args ...string) error {
	cmd := exec.Command(command, args...)
	cmd.Dir = dir
	slog.Debug(cmd.String())
	out, err := cmd.CombinedOutput()
	if err != nil {
		slog.Error(string(out))
		return err
	}
	slog.Info(string(out))
	return nil
}
