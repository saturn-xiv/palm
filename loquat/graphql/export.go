package graphql

import (
	"errors"
	"net"

	"github.com/saturn-xiv/palm/loquat/models"
	v2 "github.com/saturn-xiv/palm/loquat/router/v2"
	"gorm.io/gorm"
)

func Export(db *gorm.DB) (*v2.Router, error) {
	var rt v2.Router
	{
		var bond v2.InternetBond
		ok, err := load_internet_bond(db, &bond)
		if err != nil {
			return nil, err
		}
		if ok {
			rt.Wan = &bond
		}
	}
	{
		var bond v2.IntranetBond
		ok, err := load_intranet_bond(db, v2.DMZ, &bond)
		if err != nil {
			return nil, err
		}
		if ok {
			rt.Dmz = &bond
		}
	}
	{
		var bond v2.IntranetBond
		ok, err := load_intranet_bond(db, v2.LAN, &bond)
		if err != nil {
			return nil, err
		}
		if ok {
			rt.Lan = &bond
		}
	}

	if err := rt.VerifyInterface(); err != nil {
		return nil, err
	}
	return &rt, nil
}

func load_internet_bond(db *gorm.DB, bond *v2.InternetBond) (bool, error) {
	var it InternetBond
	err := models.GetB(db, bondKey(v2.WAN), &bond)
	if errors.Is(err, gorm.ErrRecordNotFound) {
		return false, nil
	}
	if err != nil {
		return false, err
	}
	if !it.enable {
		return false, nil
	}

	bond.MiiMonitorInterval = 1
	bond.Interfaces = make(map[string]*v2.Internet)

	for _, name := range it.interfaces {
		var profile v2.Internet
		if err = models.GetProtobuf(db, networkInterfaceKey(name), &profile); err != nil {
			return false, err
		}
		bond.Interfaces[name] = &profile
	}

	return true, nil
}

func load_intranet_bond(db *gorm.DB, name string, bond *v2.IntranetBond) (bool, error) {
	var it IntranetBond
	err := models.GetB(db, bondKey(name), &it)
	if errors.Is(err, gorm.ErrRecordNotFound) {
		return false, nil
	}
	if err != nil {
		return false, err
	}
	if !it.enable {
		return false, nil
	}

	_, net4, err := net.ParseCIDR(it.address)
	if err != nil {
		return false, err
	}

	bond.Interfaces = it.interfaces
	bond.Network = &v2.Intranet{
		Address: it.address,
	}
	{
		var hosts []models.Host
		if err = db.Where(map[string]interface{}{"network": net4.String(), "fixed": true}).Find(&hosts).Error; err != nil {
			return false, err
		}
		for _, host := range hosts {
			bond.Network.Hosts = append(bond.Network.Hosts, &v2.Intranet_Host{
				Mac:  host.Mac,
				Ip:   host.Ip,
				Name: *host.Name,
			})
		}
	}
	switch it.dns {
	case "Google":
		bond.Network.Dns = &v2.Intranet_Google_{Google: &v2.Intranet_Google{}}
	default:
		bond.Network.Dns = &v2.Intranet_Ali_{Ali: &v2.Intranet_Ali{}}
	}

	return true, nil
}
