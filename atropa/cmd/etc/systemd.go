package etc

import (
	"fmt"
	"log/slog"
	"os"
	"text/template"
)

func RpcSystemdConf(domain string, port uint16) error {
	return systend_conf(fmt.Sprintf("rpc.atropa.%s.conf", domain), domain, "Atropa rpc service", fmt.Sprintf("rpc -p %d", port))
}

func WwwSystemdConf(domain string, port uint16) error {
	return systend_conf(fmt.Sprintf("www.atropa.%s.conf", domain), domain, "Atropa http service", fmt.Sprintf("web -p %d", port))
}

func SmsSendWorkerSystemdConf(domain string) error {
	return systend_conf(fmt.Sprintf("sms-send.atropa.%s.conf", domain), domain, "Atropa sms-send consumer worker", "sms-send-worker --queue sms")
}
func EmailSendWorkerSystemdConf(domain string) error {
	return systend_conf(fmt.Sprintf("email-send.atropa.%s.conf", domain), domain, "Atropa email-send consumer worker", "email-send-worker --queue sms")
}

func systend_conf(file string, domain string, description string, args string) error {
	tpl, err := template.ParseFS(gl_templates_fs, "templates/systemd.conf")
	if err != nil {
		return err
	}

	slog.Info("generate", slog.String("file", file))
	fd, err := os.Create(file)
	if err != nil {
		return err
	}
	defer fd.Close()
	if err = tpl.Execute(fd, map[string]interface{}{
		"domain":      domain,
		"description": description,
		"args":        args,
	}); err != nil {
		return err
	}
	slog.Info(fmt.Sprintf("please copy %s => /usr/lib/systemd/system/", file))
	return nil
}
