package graphql

import (
	"context"
	"errors"
	"fmt"
	"net"

	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/loquat/models"
	v2 "github.com/saturn-xiv/palm/loquat/router/v2"
)

func (p *Mutation) BondWan(ctx context.Context, args struct{ Interfaces []string }) (*Ok, error) {
	if _, _, err := current_user(ctx, p.db, p.secrets); err != nil {
		return nil, err
	}
	bond := v2.Bond{
		Label: gl_bond_wan,
		Mode:  v2.Bond_BalanceAlb,
	}
	if err := checkPublicBond(p.db, &bond); err != nil {
		return nil, err
	}
	if err := setBond(p.db, &bond); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}

func (p *Mutation) BondLan(ctx context.Context, args struct {
	Interfaces []string
	Address    string
}) (*Ok, error) {
	if _, _, err := current_user(ctx, p.db, p.secrets); err != nil {
		return nil, err
	}

	bond := v2.Bond{
		Label:   gl_bond_lan,
		Mode:    v2.Bond_BalanceRr,
		Address: args.Address,
	}
	if err := checkPrivateBond(p.db, &bond); err != nil {
		return nil, err
	}
	if err := setBond(p.db, &bond); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}

func (p *Mutation) BondDmz(ctx context.Context, args struct {
	Interfaces []string
	Address    string
}) (*Ok, error) {
	if _, _, err := current_user(ctx, p.db, p.secrets); err != nil {
		return nil, err
	}

	bond := v2.Bond{
		Label:   gl_bond_dmz,
		Mode:    v2.Bond_BalanceRr,
		Address: args.Address,
	}
	if err := checkPrivateBond(p.db, &bond); err != nil {
		return nil, err
	}
	if err := setBond(p.db, &bond); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}

func (p *Query) BondDmz(ctx context.Context) (*PrivateBond, error) {
	if _, _, err := current_user(ctx, p.db, p.secrets); err != nil {
		return nil, err
	}
	bond, err := getBond(p.db, gl_bond_dmz)
	if err != nil {
		return nil, err
	}
	return &PrivateBond{item: bond}, nil
}

func (p *Query) BondLan(ctx context.Context) (*PrivateBond, error) {
	if _, _, err := current_user(ctx, p.db, p.secrets); err != nil {
		return nil, err
	}

	bond, err := getBond(p.db, gl_bond_lan)
	if err != nil {
		return nil, err
	}
	return &PrivateBond{item: bond}, nil
}

func (p *Query) BondWan(ctx context.Context) (*PublicBond, error) {
	if _, _, err := current_user(ctx, p.db, p.secrets); err != nil {
		return nil, err
	}

	bond, err := getBond(p.db, gl_bond_wan)
	if err != nil {
		return nil, err
	}
	return &PublicBond{item: bond}, nil
}

type PublicBond struct {
	item *v2.Bond
}

func (p *PublicBond) Interfaces() []string {
	return p.item.Interfaces
}

type PrivateBond struct {
	item *v2.Bond
}

func (p *PrivateBond) Address() string {
	return p.item.Address
}

func (p *PrivateBond) Interfaces() []string {
	return p.item.Interfaces
}

func bondKey(label string) string {
	return fmt.Sprintf("bond.%s", label)
}

func setBond(db *gorm.DB, it *v2.Bond) error {
	return models.SetProtobuf(db, bondKey(it.Label), it)
}

func getBond(db *gorm.DB, label string) (*v2.Bond, error) {
	var it v2.Bond
	if err := models.GetProtobuf(db, bondKey(label), &it); err != nil {
		return nil, err
	}
	return &it, nil
}

func checkPublicBond(db *gorm.DB, bond *v2.Bond) error {
	for _, name := range bond.Interfaces {
		iface, err := net.InterfaceByName(name)
		if err != nil {
			return err
		}
		if _, err = getNetworkInterface(db, iface); err != nil {
			return err
		}
	}
	return nil
}

func checkPrivateBond(db *gorm.DB, bond *v2.Bond) error {
	{
		ip, _, err := net.ParseCIDR(bond.Address)
		if err != nil {
			return err
		}
		if !ip.IsPrivate() {
			return fmt.Errorf("%s is not an private address", bond.Address)
		}
	}

	for _, name := range bond.Interfaces {
		iface, err := net.InterfaceByName(name)
		if err != nil {
			return err
		}
		_, err = getNetworkInterface(db, iface)
		if err == nil {
			return fmt.Errorf("interface %s is in use", name)
		}
		if !errors.Is(err, gorm.ErrRecordNotFound) {
			return err
		}
	}

	return nil
}

var (
	gl_bond_wan = "wan"
	gl_bond_lan = "lan"
	gl_bond_dmz = "dmz"
)
