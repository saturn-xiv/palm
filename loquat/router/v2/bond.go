package v2

// https://netplan.readthedocs.io/en/stable/examples/#how-to-configure-multiple-bonds
// https://netplan.readthedocs.io/en/latest/netplan-yaml/#properties-for-device-type-bonds
func (p *Bond) netplan() (map[string]interface{}, error) {

	data := map[string]interface{}{
		"interfaces": p.Interfaces,
	}
	params := map[string]interface{}{
		"mode":                 p.Mode.ToString(),
		"mii-monitor-interval": p.MiiMonitorInterval,
	}
	switch p.Mode {
	case Bond_ActiveBackup:
		data["addresses"] = []string{p.Address}
		data["nameservers"] = map[string]interface{}{
			"addresses": p.Dns,
		}
		params["gratuitous-arp"] = p.GratuitousArp
	}
	data["parameters"] = params
	return data, nil
}
