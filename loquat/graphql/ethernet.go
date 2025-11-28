package graphql

import (
	"context"
	"errors"
	"fmt"

	"google.golang.org/protobuf/types/known/emptypb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/loquat/models"
	v2 "github.com/saturn-xiv/palm/loquat/router/v2"
)

func (p *Mutation) DisableNetworkInterface(ctx context.Context, args struct {
	Name string
}) (*Ok, error) {
	user, ip, err := current_user(ctx, p.db, p.secrets)
	if err != nil {
		return nil, err
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if err := tx.Unscoped().Where("key = ?", networkInterfaceKey(args.Name)).Delete(&models.Setting{}).Error; err != nil {
			return err
		}
		return tx.Create(&models.Log{UserID: user.ID, Ip: ip, Message: fmt.Sprintf("disable %s", args.Name)}).Error
	}); err != nil {
		return nil, err
	}

	return &Ok{}, nil
}
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
	user, ip, err := current_user(ctx, p.db, p.secrets)
	if err != nil {
		return nil, err
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var profile v2.Internet
		err := models.GetProtobuf(tx, networkInterfaceKey(args.Name), &profile)
		if err != nil && !errors.Is(err, gorm.ErrRecordNotFound) {
			return err
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
		if err = models.SetProtobuf(tx, networkInterfaceKey(args.Name), &profile); err != nil {
			return err
		}

		return tx.Create(&models.Log{UserID: user.ID, Ip: ip, Message: fmt.Sprintf("set %s to %s", args.Name, args.Address)}).Error
	}); err != nil {
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
	user, ip, err := current_user(ctx, p.db, p.secrets)
	if err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var profile v2.Internet
		err := models.GetProtobuf(tx, networkInterfaceKey(args.Name), &profile)
		if err != nil && !errors.Is(err, gorm.ErrRecordNotFound) {
			return err
		}

		profile.Label = args.Label
		profile.Memo = args.Memo
		profile.Isp = args.Isp
		profile.Ip = &v2.Internet_Dhcp{Dhcp: &emptypb.Empty{}}
		if err = models.SetProtobuf(tx, networkInterfaceKey(args.Name), &profile); err != nil {
			return err
		}

		return tx.Create(&models.Log{UserID: user.ID, Ip: ip, Message: fmt.Sprintf("set %s to dhcp", args.Name)}).Error
	}); err != nil {
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
	case *v2.Internet_Dhcp:
		return &DynamicIp{
			isp:   p.item.Isp,
			label: p.item.Label,
			memo:  p.item.Memo,
		}, nil
	default:
		return nil, errors.New("not a dynamic ip")
	}
}
