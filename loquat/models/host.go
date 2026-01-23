package models

import (
	"log/slog"
	"net"

	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/loquat/router/nmap"
)

type Host struct {
	gorm.Model

	MemberID *uint
	Name     *string `gorm:"index;size:63"`
	Mac      string  `gorm:"index;not null;size:17"`
	Vendor   *string `gorm:"size:63"`
	Network  string  `gorm:"index;not null;size:39"`
	Ip       string  `gorm:"index;not null;size:39"`
	Fixed    bool    `gorm:"not null;default:false"`
	Version  uint    `gorm:"not null;default:0"`
	Rules    []Rule  `gorm:"many2many:hosts_rules;"`
	Member   *Member
}

func (Host) TableName() string {
	return "hosts"
}

func ScanHosts(dev string, network string) ([]Host, error) {
	res, err := nmap.Scan(dev, network)
	if err != nil {
		return nil, err
	}
	slog.Info("scan result", "summary", res.RunStats.Finished.Summary)
	var items []Host
	for _, host := range res.Hosts {
		var it Host
		for _, addr := range host.Addresses {
			if addr.AddrType == "mac" {
				it.Mac = addr.Addr
				it.Vendor = addr.Vendor
				continue
			}
			if addr.AddrType == "ipv4" {
				ip := net.ParseIP(addr.Addr)
				if ip == nil {
					slog.Error("not a valid ip address", "v4", addr.Addr)
				} else {
					_, net, err := net.ParseCIDR(network)
					if err != nil {
						return nil, err
					}
					it.Ip = ip.String()
					it.Network = net.String()
				}
				continue
			}
		}
		if it.Mac == "" || it.Ip == "" || it.Network == "" {
			slog.Warn("found empty host")
			continue
		}
		if len(host.Hostname) > 0 {
			it.Name = &host.Hostname[0]
		}
		slog.Debug("found host", "mac", it.Mac, "ip", it.Ip, "vendor", it.Vendor, "name", it.Name)
		items = append(items, it)
	}
	return items, nil
}
