package etc

import (
	"fmt"
	"log/slog"
	"os"
	"text/template"
)

func RpcSystemdConf(domain string, port uint16) error {
	tpl, err := template.ParseFS(gl_templates_fs, "templates/systemd.conf")
	if err != nil {
		return err
	}

	file := fmt.Sprintf("rpc.atropa.%s.conf", domain)
	slog.Info("generate", slog.String("file", file))
	fd, err := os.Create(file)
	if err != nil {
		return err
	}
	defer fd.Close()
	if err = tpl.Execute(fd, map[string]interface{}{
		"domain":      domain,
		"description": "Atropa rpc service",
		"args":        fmt.Sprintf("rpc -p %d", port),
	}); err != nil {
		return err
	}
	slog.Info(fmt.Sprintf("please copy %s => /usr/lib/systemd/system/", file))
	return nil
}

func WwwSystemdConf(domain string, port uint16) error {
	tpl, err := template.ParseFS(gl_templates_fs, "templates/systemd.conf")
	if err != nil {
		return err
	}

	file := fmt.Sprintf("www.atropa.%s.conf", domain)
	slog.Info("generate", slog.String("file", file))
	fd, err := os.Create(file)
	if err != nil {
		return err
	}
	defer fd.Close()
	if err = tpl.Execute(fd, map[string]interface{}{
		"domain":      domain,
		"description": "Atropa http service",
		"args":        fmt.Sprintf("web -p %d", port),
	}); err != nil {
		return err
	}
	slog.Info(fmt.Sprintf("please copy %s => /usr/lib/systemd/system/", file))
	return nil
}
