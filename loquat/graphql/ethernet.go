package graphql

import (
	"context"
	"errors"
	"fmt"

	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/loquat/models"
	v2 "github.com/saturn-xiv/palm/loquat/router/v2"
)

func (p *Mutation) SetNetworkInterfacePublicStaticIp(ctx context.Context, args struct {
	Name    string
	Label   string
	Memo    string
	Address string
	Netmask string
	Gateway string
	Isp     string
	Dns     []string
}) (*Ok, error) {
	if _, _, err := current_user(ctx, p.db, p.secrets); err != nil {
		return nil, err
	}
	var profile v2.Internet
	err := models.GetProtobuf(p.db, networkInterfaceKey(args.Name), &profile)
	if err != nil && !errors.Is(err, gorm.ErrRecordNotFound) {
		return nil, err
	}

	profile.Label = args.Label
	profile.Memo = args.Memo
	profile.Isp = args.Isp
	profile.Ip = &v2.Internet_Static_{Static: &v2.Internet_Static{
		Dns:     args.Dns,
		Address: args.Address,
		Netmask: args.Netmask,
		Gateway: args.Gateway,
	}}
	if err = models.SetProtobuf(p.db, networkInterfaceKey(args.Name), &profile); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}
func (p *Mutation) SetNetworkInterfacePublicDhcp(ctx context.Context, args struct {
	Name  string
	Label string
	Isp   string
	Memo  string
}) (*Ok, error) {
	if _, _, err := current_user(ctx, p.db, p.secrets); err != nil {
		return nil, err
	}
	var profile v2.Internet
	err := models.GetProtobuf(p.db, networkInterfaceKey(args.Name), &profile)
	if err != nil && !errors.Is(err, gorm.ErrRecordNotFound) {
		return nil, err
	}

	profile.Label = args.Label
	profile.Memo = args.Memo
	profile.Isp = args.Isp
	profile.Ip = &v2.Internet_Dhcp_{Dhcp: &v2.Internet_Dhcp{}}
	if err = models.SetProtobuf(p.db, networkInterfaceKey(args.Name), &profile); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}

func (p *Query) GetNetworkInterface(ctx context.Context, args struct {
	Name string
}) (*NetworkInterfaceProfile, error) {
	if _, _, err := current_user(ctx, p.db, p.secrets); err != nil {
		return nil, err
	}
	var it NetworkInterfaceProfile
	if err := models.GetB(p.db, networkInterfaceKey(args.Name), it); err != nil {
		return nil, err
	}
	return &it, nil
}
func networkInterfaceKey(name string) string {
	return fmt.Sprintf("net.%s", name)
}

type StaticIp struct {
	label   string
	memo    string
	isp     string
	address string
	netmask string
	gateway string
	dns     []string
}

func (p *StaticIp) Label() string {
	return p.label
}

func (p *StaticIp) Dns() []string {
	return p.dns
}
func (p *StaticIp) Netmask() string {
	return p.netmask
}
func (p *StaticIp) Gateway() string {
	return p.gateway
}
func (p *StaticIp) Address() string {
	return p.address
}
func (p *StaticIp) Isp() string {
	return p.isp
}

func (p *StaticIp) Memo() string {
	return p.memo
}

type DynamicIp struct {
	label string
	memo  string
	isp   string
}

func (p *DynamicIp) Label() string {
	return p.label
}

func (p *DynamicIp) Isp() string {
	return p.isp
}

func (p *DynamicIp) Memo() string {
	return p.memo
}

type NetworkInterfaceProfile struct {
	item *v2.Internet
}

func (p *NetworkInterfaceProfile) ToStaticIp() (*StaticIp, error) {
	switch p.item.Ip.(type) {
	case *v2.Internet_Static_:
		return &StaticIp{
			isp:     p.item.Isp,
			label:   p.item.Label,
			memo:    p.item.Memo,
			address: p.item.GetStatic().Address,
			netmask: p.item.GetStatic().Netmask,
			gateway: p.item.GetStatic().Gateway,
			dns:     p.item.GetStatic().Dns,
		}, nil
	default:
		return nil, errors.New("not a static ip")
	}

}
func (p *NetworkInterfaceProfile) ToDynamicIp() (*DynamicIp, error) {
	switch p.item.Ip.(type) {
	case *v2.Internet_Dhcp_:
		return &DynamicIp{
			isp:   p.item.Isp,
			label: p.item.Label,
			memo:  p.item.Memo,
		}, nil
	default:
		return nil, errors.New("not a dynamic ip")
	}
}

// func setNetworkInterface(db *gorm.DB, it *net.Interface, profile *v2.Ethernet) error {
// 	return models.SetProtobuf(db, networkInterfaceKey(it), profile)
// }

// func getNetworkInterface(db *gorm.DB, it *net.Interface) (*v2.Ethernet, error) {
// 	var profile v2.Ethernet
// 	if err := models.GetProtobuf(db, networkInterfaceKey(it), &profile); err != nil {
// 		return nil, err
// 	}
// 	return &profile, nil
// }

// func fetchNetworkInterface(db *gorm.DB, name string) (*net.Interface, *v2.Ethernet, error) {
// 	eth, err := net.InterfaceByName(name)
// 	if err != nil {
// 		return nil, nil, err
// 	}
// 	it, err := getNetworkInterface(db, eth)
// 	if err == nil {
// 		return eth, it, nil
// 	}
// 	if errors.Is(err, gorm.ErrRecordNotFound) {
// 		return eth, &v2.Ethernet{}, nil
// 	}
// 	return nil, nil, err
// }
