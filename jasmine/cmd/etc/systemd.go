package etc

import (
	"fmt"
	"log/slog"
	"os"
	"text/template"

	"github.com/saturn-xiv/palm/jasmine/env"
)

func RpcSystemdConf(domain string, port uint16) error {
	tpl, err := template.ParseFS(gl_templates_fs, "templates/systemd.conf")
	if err != nil {
		return err
	}

	file := fmt.Sprintf("rpc.%s.%s.conf", env.PLUGIN_NAME, domain)
	slog.Info("generate", slog.String("file", file))
	fd, err := os.Create(file)
	if err != nil {
		return err
	}
	defer fd.Close()
	if err = tpl.Execute(fd, map[string]interface{}{
		"domain":      domain,
		"description": fmt.Sprintf("%s rpc service", env.PLUGIN_NAME),
		"args":        fmt.Sprintf("rpc -p %d", port),
	}); err != nil {
		return err
	}
	slog.Info("please copy to /usr/lib/systemd/system/", slog.String("file", file))
	return nil
}

func WwwSystemdConf(domain string, port uint16) error {
	tpl, err := template.ParseFS(gl_templates_fs, "templates/systemd.conf")
	if err != nil {
		return err
	}

	file := fmt.Sprintf("www.%s.%s.conf", env.PLUGIN_NAME, domain)
	slog.Info("generate", slog.String("file", file))
	fd, err := os.Create(file)
	if err != nil {
		return err
	}
	defer fd.Close()
	if err = tpl.Execute(fd, map[string]interface{}{
		"domain":      domain,
		"description": fmt.Sprintf("%s http service", env.PLUGIN_NAME),
		"args":        fmt.Sprintf("web -p %d", port),
	}); err != nil {
		return err
	}
	slog.Info("please copy to /usr/lib/systemd/system/", slog.String("from", file))
	return nil
}
