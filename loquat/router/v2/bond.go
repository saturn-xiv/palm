package v2

import (
	_ "embed"
)

// https://netplan.readthedocs.io/en/stable/examples/#how-to-configure-multiple-bonds
// https://netplan.readthedocs.io/en/latest/netplan-yaml/#properties-for-device-type-bonds
// https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/7/html/networking_guide/overview-of-bonding-modes-and-the-required-settings-on-the-switch

//go:embed templates/dnsmasq.txt
var gl_dnsmasq_txt string

//go:embed templates/dnsmasq-header.txt
var gl_dnsmasq_header_txt string

var (
	DMZ = "bond-dmz"
	LAN = "bond-lan"
	WAN = "bond-wan"
)

func (p *IntranetBond) netplan(dev string) (string, error) {

	items := map[string]interface{}{
		"interfaces": p.Interfaces,
		"addresses":  []string{p.Network.Address},
		"parameters": map[string]interface{}{
			"mode":                 "balance-xor",
			"mii-monitor-interval": p.MiiMonitorInterval,
			// layer2, layer3+4, layer2+3, encap2+3 and encap3+4.
			"transmit-hash-policy": "layer3+4",
		},
	}

	return render_netplan_yaml("bonds", dev, items)
}

func (p *InternetBond) netplan(dev string) (string, error) {
	var interfaces []string
	for dev := range p.Interfaces {
		interfaces = append(interfaces, dev)
	}

	items := map[string]interface{}{
		"interfaces": interfaces,
		"parameters": map[string]interface{}{
			"mode":                 "balance-alb",
			"mii-monitor-interval": p.MiiMonitorInterval,
		},
	}

	return render_netplan_yaml("bonds", dev, items)
}
