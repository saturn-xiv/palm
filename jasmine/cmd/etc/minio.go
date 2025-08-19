package etc

import (
	"crypto/rand"
	"fmt"
	"log/slog"
	"os"
	"text/template"

	"github.com/btcsuite/btcutil/base58"
	"github.com/saturn-xiv/palm/jasmine/env"
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
	slog.Info("please copy to /etc/nginx/sites-enabled/", slog.String("file", file))
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

	file := fmt.Sprintf("s3.%s.%s.conf", env.PLUGIN_NAME, domain)
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
	slog.Info("please copy to /usr/lib/systemd/system/", slog.String("file", file))
	return nil
}
