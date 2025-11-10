package v2

import (
	"encoding/binary"
	"errors"
	"net"
)

// https://netplan.readthedocs.io/en/latest/netplan-yaml/#properties-for-device-type-ethernets
func (p *Ethernet) netplan() (map[string]interface{}, error) {
	args := make(map[string]interface{})
	switch p.Ip.(type) {
	case *Ethernet_Dhcp_:
		args["dhcp4"] = true
	case *Ethernet_Static_:
		args["addresses"] = []string{p.GetStatic().Address}
		args["nameservers"] = map[string]interface{}{
			"addresses": p.GetStatic().Dns,
		}
	case *Ethernet_Pppoe_:
		// https://netplan.readthedocs.io/en/latest/netplan-yaml/#properties-for-device-type-modems
		return nil, errors.New("does not support modems yet")
	case *Ethernet_Lan_:
		args["addresses"] = []string{p.GetLan().Address}
	}
	return args, nil
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
