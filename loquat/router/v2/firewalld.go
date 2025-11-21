package v2

import (
	_ "embed"
	"fmt"
)

//go:embed templates/firewalld.txt
var gl_firewalld_txt string

// ----------------------------------------------------------------------------
func (p *FirewallRule_Protocol) ToString() string {
	switch *p {
	case FirewallRule_Tcp:
		return "tcp"
	case FirewallRule_Udp:
		return "udp"
	default:
		return ""
	}
}

func firewall_add_port(zone string, protocol string, port uint32) string {
	return fmt.Sprintf("firewall-cmd --permanent --zone=%s --add-port=%d/%s", zone, port, protocol)
}
func firewall_add_service(zone string, service string) string {
	return fmt.Sprintf("firewall-cmd --permanent --zone=%s --add-service=%s", zone, service)
}
func firewall_reset(zone string) string {
	return fmt.Sprintf("firewall-cmd --permanent --load-zone-defaults=%s", zone)
}

func firewall_snat(wan string, lan string) []string {
	return []string{}

}
