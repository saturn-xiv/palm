package hibiscus

import (
	"embed"
	"io/fs"
	"log/slog"
)

func LoadTemplates(node *embed.FS, theme string) error {
	return fs.WalkDir(node, theme, func(path string, it fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if it.IsDir() {
			return nil
		}
		if !it.Type().IsRegular() {
			return nil
		}
		slog.Debug("load html template", slog.String("file", path))
		buf, err := fs.ReadFile(node, path)
		if err != nil {
			return err
		}
		if _, err = gl_html_template.Parse(string(buf)); err != nil {
			return err
		}
		return nil
	})
}
