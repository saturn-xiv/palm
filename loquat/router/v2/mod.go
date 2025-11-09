package v2

import (
	_ "embed"
	"encoding/binary"
	"fmt"
	"io"
	"log/slog"
	"net"
	"os"
	"os/exec"
	"path/filepath"
	"text/template"
	"time"
)

//go:embed templates/firewalld.txt
var gl_firewalld_txt string

//go:embed templates/netplan.txt
var gl_netplan_txt string

//go:embed templates/dnsmasq.txt
var gl_dnsmasq_txt string

//go:embed templates/header.txt
var gl_header_txt string

//go:embed templates/footer.txt
var gl_footer_txt string

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
func (p *Router) setup_netplan(wrt io.Writer) error {
	slog.Debug("setup netplan")
	// TODO
	return nil
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

func (p *Ethernet_Lan) DnsServers() []string {
	switch p.Dns.(type) {
	case *Ethernet_Lan_Ali_:
		// https://www.alidns.com/
		return []string{"223.5.5.5", "223.6.6.6"}
	case *Ethernet_Lan_Google_:
		// https://developers.google.com/speed/public-dns
		return []string{"8.8.8.8", "8.8.4.4"}
	case *Ethernet_Lan_Other_:
		return p.GetOther().Hosts
	default:
		return []string{}
	}
}

func (p *Ethernet_Lan) Gateway() (string, error) {
	first, _, err := p.ipv4_addresses()
	if err != nil {
		return "", err
	}
	return uint32_to_ipv4(first + 1), nil
}

func (p *Ethernet_Lan) Broadcast() (string, error) {
	_, last, err := p.ipv4_addresses()
	if err != nil {
		return "", err
	}
	return uint32_to_ipv4(last), nil
}

func (p *Ethernet_Lan) Network() (string, error) {
	_, net4, err := net.ParseCIDR(p.Address)
	if err != nil {
		return "", err
	}
	return net4.String(), nil
}

func (p *Ethernet_Lan) Addresses() ([]string, error) {
	first, last, err := p.ipv4_addresses()
	if err != nil {
		return nil, err
	}
	var items []string
	for i := first + 2; i <= last; i += 1 {
		items = append(items, uint32_to_ipv4(i))
	}
	return items, nil
}

func (p *Ethernet_Lan) ipv4_addresses() (uint32, uint32, error) {
	_, net4, err := net.ParseCIDR(p.Address)
	if err != nil {
		return 0, 0, err
	}
	mask := binary.BigEndian.Uint32(net4.Mask)
	first := binary.BigEndian.Uint32(net4.IP)
	last := (first & mask) | (mask ^ 0xffffffff)
	return first, last, nil
}

func uint32_to_ipv4(i uint32) string {
	ip := make(net.IP, 4)
	binary.BigEndian.PutUint32(ip, i)
	return ip.String()
}
