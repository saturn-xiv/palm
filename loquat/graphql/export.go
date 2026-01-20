package graphql

import (
	"errors"
	"log/slog"
	"net"

	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/loquat/models"
	v2 "github.com/saturn-xiv/palm/loquat/router/v2"
)

func Export(db *gorm.DB) (*v2.Router, error) {
	var rt v2.Router
	{
		slog.Debug("load dmz profile")
		bond, err := load_intranet_bond(db, v2.DMZ)
		if err != nil {
			return nil, err
		}
		rt.Dmz = bond
	}
	{
		slog.Debug("load lan profile")
		bond, err := load_intranet_bond(db, v2.LAN)
		if err != nil {
			return nil, err
		}
		rt.Lan = bond
	}

	{
		slog.Debug("verify network interfaces")
		if err := rt.VerifyInterface(); err != nil {
			return nil, err
		}
	}
	return &rt, nil
}

func load_bond(db *gorm.DB, name string) (*bondProfile, error) {
	var it bondProfile
	err := models.GetB(db, bondKey(name), &it)
	if err == nil {
		return &it, nil
	}
	if errors.Is(err, gorm.ErrRecordNotFound) {
		return nil, nil
	}
	return nil, err
}

func load_intranet_bond(db *gorm.DB, name string) (*v2.IntranetBond, error) {
	bond, err := load_bond(db, name)
	if err != nil {
		return nil, err
	}
	if bond == nil || !bond.Enable {
		return nil, nil
	}
	res := v2.IntranetBond{
		Interfaces:         bond.Interfaces,
		MiiMonitorInterval: 100,
		Network: &v2.Intranet{
			Address: bond.Address,
		},
	}
	switch bond.Dns {
	case "Google":
		res.Network.Dns = &v2.Intranet_Google_{Google: &v2.Intranet_Google{}}
	default:
		res.Network.Dns = &v2.Intranet_Ali_{Ali: &v2.Intranet_Ali{}}
	}

	{
		_, net4, err := net.ParseCIDR(bond.Address)
		if err != nil {
			return nil, err
		}
		var hosts []models.Host
		if err = db.Where(map[string]interface{}{"network": net4.String(), "fixed": true}).Find(&hosts).Error; err != nil {
			return nil, err
		}
		for _, host := range hosts {
			res.Network.Hosts = append(res.Network.Hosts, &v2.Intranet_Host{
				Mac:  host.Mac,
				Ip:   host.Ip,
				Name: *host.Name,
			})
		}
	}
	return &res, nil
}
