package v2

import (
	"fmt"
	"log/slog"
	"os"
	"os/exec"
	"path/filepath"
)

// https://www.tug.org/levels.html
// https://tug.ctan.org/macros/latex/contrib/beamer/doc/beameruserguide.pdf
// https://en.wikibooks.org/wiki/LaTeX/Document_Structure#Document_classes
func (p *Task) Execute(uploader func(f string, b string, o string) error) error {
	tmp, err := os.MkdirTemp("", "tex")
	if err != nil {
		return err
	}
	defer os.RemoveAll(tmp)
	entry := "main"
	{
		tex := fmt.Sprintf("%s.tex", entry)

		if err := save_file(tmp, tex, []byte(p.Entry)); err != nil {
			return err
		}

		for name, body := range p.Files {
			if err := save_file(tmp, name, body); err != nil {
				return err
			}
		}

		slog.Debug("building by TeX Live", "folder", tmp)
		for range 3 {
			if err := build_pdf(tmp, tex); err != nil {
				return err
			}
		}
	}

	pdf := filepath.Join(tmp, fmt.Sprintf("%s.pdf", entry))
	if err = uploader(pdf, p.Target.Bucket, p.Target.Object); err != nil {
		return err
	}
	return nil
}

func save_file(folder string, name string, body []byte) error {
	file := filepath.Join(folder, name)
	slog.Debug("create", "file", file)
	return os.WriteFile(file, body, 0444)
}
func build_pdf(folder string, entry string) error {
	cmd := exec.Command("lualatex", "--halt-on-error", entry)
	cmd.Dir = folder
	buf, err := cmd.Output()
	if err != nil {
		return err
	}
	slog.Debug(string(buf))
	return nil
}
