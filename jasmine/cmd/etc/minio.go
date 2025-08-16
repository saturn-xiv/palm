package etc

import (
	"crypto/rand"
	"fmt"
	"log/slog"
	"os"
	"text/template"

	"github.com/btcsuite/btcutil/base58"
)

func MinioNginxConf(domain string) error {
	tpl, err := template.ParseFS(gl_templates_fs, "templates/minio/nginx.conf")
	if err != nil {
		return err
	}

	file := fmt.Sprintf("assets.%s.conf", domain)
	slog.Info("generate", slog.String("file", file))
	fd, err := os.Create(file)
	if err != nil {
		return err
	}
	defer fd.Close()
	if err = tpl.Execute(fd, map[string]interface{}{
		"domain": domain,
		"nodes": []map[string]interface{}{
			{"host": "127.0.0.1", "port": 9000, "console_port": 9001},
		},
	}); err != nil {
		return err
	}
	slog.Info(fmt.Sprintf("please copy %s => /etc/nginx/sites-enabled/", file))
	return nil
}

func MinioSystemdConf(domain string) error {
	password := make([]byte, 32)
	if _, err := rand.Read(password); err != nil {
		return err
	}
	tpl, err := template.ParseFS(gl_templates_fs, "templates/minio/systemd.conf")
	if err != nil {
		return err
	}

	file := fmt.Sprintf("s3.atropa.%s.conf", domain)
	slog.Info("generate", slog.String("file", file))
	fd, err := os.Create(file)
	if err != nil {
		return err
	}
	defer fd.Close()
	if err = tpl.Execute(fd, map[string]interface{}{
		"domain":       domain,
		"port":         9000,
		"console_port": 9001,
		"user":         "root",
		"password":     base58.Encode(password),
	}); err != nil {
		return err
	}
	slog.Info(fmt.Sprintf("please copy %s => /usr/lib/systemd/system/", file))
	return nil
}
