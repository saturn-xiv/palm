package v2

import "fmt"

// ----------------------------------------------------------------------------
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

func firewall_snat(wan string, lan string) []string {
	return []string{}

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
