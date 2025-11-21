package v2

import (
	"fmt"
)

// https://netplan.readthedocs.io/en/latest/netplan-yaml/#properties-for-device-type-ethernets
func (p *Internet) netplan(dev string) (string, error) {
	args := make(map[string]interface{})
	switch p.Ip.(type) {
	case *Internet_Dhcp_:
		args["dhcp4"] = true
	case *Internet_Static_:
		args["addresses"] = []string{p.GetStatic().Address}
		args["nameservers"] = map[string]interface{}{
			"addresses": p.GetStatic().Dns,
		}
	default:
		// https://netplan.readthedocs.io/en/latest/netplan-yaml/#properties-for-device-type-modems
		return "", fmt.Errorf("does not support mod %v yet", p.Ip)
	}
	return render_netplan_yaml("ethernets", dev, args)
}
