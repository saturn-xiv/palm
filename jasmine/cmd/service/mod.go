package service

import (
	"fmt"
	"log/slog"
	"os"
	"text/template"
)

func SystemdConf(name string, port uint16) error {
	tpl, err := template.New("").Parse(`
[Unit]
Description=A minio service.
Wants=network-online.target
After=network-online.target

[Service]
Type=notify
User=root
Group=root
ExecStart=/usr/bin/jasmine rpc -p {{ .port }}
WorkingDirectory=/var/lib/{{ .name }}
Restart=always

[Install]
WantedBy=multi-user.target
`)
	if err != nil {
		return err
	}

	file := fmt.Sprintf("%s.conf", name)
	slog.Info("generate", slog.String("file", file))
	fd, err := os.Create(file)
	if err != nil {
		return err
	}
	defer fd.Close()
	if err = tpl.Execute(fd, map[string]interface{}{"name": name, "port": port}); err != nil {
		return err
	}
	slog.Info(fmt.Sprintf("please copy %s => /usr/lib/systemd/system/%s.conf", file, name))
	return nil
}
