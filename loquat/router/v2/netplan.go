package v2

import (
	_ "embed"
	"io"
	"log/slog"
	"text/template"

	"github.com/goccy/go-yaml"
)

//go:embed templates/netplan.txt
var gl_netplan_txt string

func (p *Router) setup_netplan(wrt io.Writer) error {
	slog.Debug("setup netplan")
	items := make(map[string]interface{})
	if p.Dmz != nil {
		buf, err := p.Dmz.netplan(DMZ)
		if err != nil {
			return err
		}
		items[DMZ] = map[string]string{"label": DMZ, "content": buf}
	}
	if p.Lan != nil {
		buf, err := p.Lan.netplan(LAN)
		if err != nil {
			return err
		}
		items[LAN] = map[string]string{"label": LAN, "content": buf}
	}

	tpl, err := template.New("").Parse(gl_netplan_txt)
	if err != nil {
		return err
	}
	return tpl.Execute(wrt, map[string]interface{}{"items": items})
}

func render_netplan_yaml(category string, dev string, args map[string]interface{}) (string, error) {
	buf, err := yaml.Marshal(map[string]interface{}{
		"network": map[string]interface{}{
			"version":  2,
			"renderer": "networkd",
			category:   map[string]interface{}{dev: args},
		},
	})
	if err != nil {
		return "", err
	}
	return string(buf), nil
}
