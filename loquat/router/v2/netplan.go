package v2

import (
	_ "embed"
	"io"
	"text/template"

	"github.com/goccy/go-yaml"
)

//go:embed templates/netplan.txt
var gl_netplan_txt string

// https://netplan.readthedocs.io/en/stable/examples/#
func (p *Router) setup_netplan(wrt io.Writer) error {
	items := make(map[string]interface{})
	if p.Wan != nil {
		for dev, it := range p.Wan.Interfaces {
			buf, err := it.netplan(dev)
			if err != nil {
				return err
			}
			items[dev] = map[string]string{"label": it.Label, "content": buf}
		}

		{
			buf, err := p.Wan.netplan(WAN)
			if err != nil {
				return err
			}
			items[WAN] = map[string]string{"label": "", "content": buf}
		}
	}
	if p.Dmz != nil {
		buf, err := p.Dmz.netplan(DMZ)
		if err != nil {
			return err
		}
		items[DMZ] = map[string]string{"label": "", "content": buf}
	}
	if p.Lan != nil {
		buf, err := p.Lan.netplan(LAN)
		if err != nil {
			return err
		}
		items[LAN] = map[string]string{"label": "", "content": buf}
	}

	tpl, err := template.New("").Parse(gl_netplan_txt)
	if err != nil {
		return err
	}
	return tpl.Execute(wrt, map[string]interface{}{"items": items})
}

func render_netplan_yaml(category string, dev string, args map[string]interface{}) (string, error) {
	var data = map[string]interface{}{
		"network": map[string]interface{}{
			"version":  2,
			"renderer": "networkd",
		},
	}
	data[category] = map[string]interface{}{dev: args}
	buf, err := yaml.Marshal(data)
	if err != nil {
		return "", err
	}
	return string(buf), nil
}
