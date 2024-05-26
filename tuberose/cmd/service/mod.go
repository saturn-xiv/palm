package service

import (
	"fmt"
	"log/slog"
	"os"
	"text/template"
)

func WebSystemdConf(name string, port uint16) error {
	name = "www." + name
	tpl, err := template.New("").Parse(`
[Unit]
Description=A sms callback handler(twilio).
Wants=network-online.target
After=network-online.target

[Service]
Type=notify
User=root
Group=root
ExecStart=/usr/bin/tuberose web -p {{ .port }}
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

func SendSmsWorkerSystemdConf(name string, queue string) error {
	name = "send-sms." + name
	tpl, err := template.New("").Parse(`
[Unit]
Description=A sms-send consumer worker(twilio).
Wants=network-online.target
After=network-online.target

[Service]
Type=notify
User=root
Group=root
ExecStart=/usr/bin/tuberose send-sms-worker -q {{ .queue }} -n {{ .name }}.consumer
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
	if err = tpl.Execute(fd, map[string]interface{}{"name": name, "queue": queue}); err != nil {
		return err
	}
	slog.Info(fmt.Sprintf("please copy %s => /usr/lib/systemd/system/%s.conf", file, name))
	return nil
}
