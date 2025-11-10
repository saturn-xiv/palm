package v2

import "fmt"

func (p *Firewall_Protocol) ToString() string {
	switch *p {
	case Firewall_Tcp:
		return "tcp"
	case Firewall_Udp:
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

// iptables -t nat -A PREROUTING -i eth0 -p tcp --dport 80 -j DNAT --to-destination 192.168.1.100:80
// iptables -t nat -A POSTROUTING -o eth0 -s 192.168.1.0/24 -j SNAT --to-source xxx.xxx.xx.xxx

func firewall_snat(wan string, lan string) []string {
	return []string{
		fmt.Sprintf("firewall-cmd --query-interface=%s", wan),
		fmt.Sprintf("firewall-cmd --query-interface=%s", lan),
		"firewall-cmd --get-active-zone",
		fmt.Sprintf("firewall-cmd --add-interface=%s --zone=external", wan),
		fmt.Sprintf("firewall-cmd --add-interface=%s --zone=internal", lan),
		"firewall-cmd --zone=external --add-masquerade --permanent",
		"firewall-cmd --reload",
		"firewall-cmd --zone=external --query-masquerade",
		"firewall-cmd --zone=internal --add-masquerade --permanent",
		"firewall-cmd --reload",
		"firewall-cmd --direct --add-rule ipv4 nat POSTROUTING 0 -o eth0 -j MASQUERADE",
		"firewall-cmd --direct --add-rule ipv4 filter FORWARD 0 -i eth0 -o eth1 -j ACCEPT",
		"firewall-cmd --direct --add-rule ipv4 filter FORWARD 0 -i eth0 -o eth1 -m state --state RELATED,ESTABLISHED -j ACCEPT",
		"firewall-cmd --reload",
	}

}

// https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/7/html/networking_guide/overview-of-bonding-modes-and-the-required-settings-on-the-switch
func (p *Bond_Mode) ToString() string {
	switch *p {
	case Bond_BalanceRr:
		return "balance-rr"
	case Bond_BalanceXor:
		return "balance-xor"
	case Bond_BalanceTlb:
		return "balance-tlb"
	case Bond_BalanceAlb:
		return "balance-alb"
	case Bond_AD802_3:
		return "802.3ad"
	case Bond_Broadcast:
		return "broadcast"
	case Bond_ActiveBackup:
		return "active-backup"
	default:
		return ""
	}
}
