package v1

import (
	"log/slog"
	"os"
	"os/exec"
	"path/filepath"
)

func (p *Tex) BuildPdf(work_dir string) (string, error) {
	slog.Debug("build pdf", slog.String("work_dir", work_dir))

	job_name := "main"
	tex_file := job_name + ".tex"
	{
		file := filepath.Join(work_dir, tex_file)
		slog.Debug("create", slog.String("file", file))
		if err := os.WriteFile(file, p.Homepage, 0600); err != nil {
			return "", err
		}
	}
	for name, body := range p.Files {
		file := filepath.Join(work_dir, name)
		slog.Debug("create", slog.String("file", file))
		if err := os.WriteFile(file, body, 0600); err != nil {
			return "", err
		}
	}

	cmd := exec.Command("/usr/bin/xelatex", "-halt-on-error", "--job-name", job_name, tex_file)
	out, err := cmd.CombinedOutput()
	if err != nil {
		return "", err
	}
	slog.Info("output", slog.String("output", string(out)))
	return filepath.Join(work_dir, (job_name + ".pdf")), nil
}
