package v2

import (
	"fmt"
	"io"
	"log/slog"
	"os"
	"os/exec"
	"path/filepath"
	"text/template"
	"time"

	"github.com/goccy/go-yaml"
)

func (p *Router) Render(wrt io.Writer) error {
	if _, err := fmt.Fprintf(wrt, "%s", gl_header_txt); err != nil {
		return err
	}
	if err := p.setup_netplan(wrt); err != nil {
		return err
	}
	if err := p.setup_dnsmasq(wrt); err != nil {
		return err
	}
	if err := p.setup_firewalld(wrt); err != nil {
		return err
	}
	_, err := fmt.Fprintf(wrt, "%s", gl_footer_txt)
	return err
}

// https://netplan.readthedocs.io/en/stable/examples/#
func (p *Router) setup_netplan(wrt io.Writer) error {
	slog.Debug("setup netplan")

	data := make(map[string]interface{})
	{
		items := make(map[string]interface{})
		for dev, it := range p.Ethernet {
			args, err := it.netplan()
			if err != nil {
				return err
			}
			buf, err := p.to_netplan_yaml("ethernets", dev, args)
			if err != nil {
				return err
			}
			items[dev] = map[string]string{"label": it.Label, "content": string(buf)}
		}
		for dev, it := range p.Bonds {
			args, err := it.netplan()
			if err != nil {
				return err
			}
			buf, err := p.to_netplan_yaml("bonds", dev, args)
			if err != nil {
				return err
			}
			items[dev] = map[string]string{"label": it.Label, "content": string(buf)}
		}
		data["items"] = items
	}

	tpl, err := template.New("").Parse(gl_netplan_txt)
	if err != nil {
		return err
	}
	return tpl.Execute(wrt, data)
}

func (p *Router) to_netplan_yaml(category string, dev string, args map[string]interface{}) ([]byte, error) {
	var data = map[string]interface{}{
		"network": map[string]interface{}{
			"version":  2,
			"renderer": "networkd",
		},
	}
	data[category] = map[string]interface{}{dev: args}
	return yaml.Marshal(data)

}
func (p *Router) setup_dnsmasq(wrt io.Writer) error {
	slog.Debug("setup dnsmasq")
	data := make(map[string]interface{})
	for device, eth := range p.Ethernet {
		switch eth.Ip.(type) {
		case *Ethernet_Lan_:
			var hosts []map[string]interface{}
			for _, it := range eth.GetLan().Hosts {
				hosts = append(hosts, map[string]interface{}{
					"mac":  it.Mac,
					"ip":   it.Ip,
					"name": it.Name,
				})
			}
			gateway, err := eth.GetLan().Gateway()
			if err != nil {
				return err
			}
			network, err := eth.GetLan().Network()
			if err != nil {
				return err
			}
			it := map[string]interface{}{
				"label":   eth.Label,
				"network": network,
				"gateway": gateway,
				"dns":     eth.GetLan().DnsServers(),
				"hosts":   hosts,
			}
			data[device] = it
		default:
			continue
		}
	}
	tpl, err := template.New("").Parse(gl_dnsmasq_txt)
	if err != nil {
		return err
	}
	return tpl.Execute(wrt, data)
}
func (p *Router) setup_firewalld(wrt io.Writer) error {
	slog.Debug("setup nftables")
	// TODO
	return nil
}

func (p *Router) Apply() error {
	tmp := filepath.Join(os.TempDir(), fmt.Sprintf("%s.sh", time.Now().Format("20060102150405")))
	if err := p.render_to_file(tmp); err != nil {
		return err
	}
	cmd := exec.Command("bash", tmp)
	return cmd.Run()
}

func (p *Router) render_to_file(name string) error {
	slog.Info("generate shell script", "file", name)
	file, err := os.Create(name)
	if err != nil {
		return err
	}
	defer file.Close()
	return p.Render(file)
}
