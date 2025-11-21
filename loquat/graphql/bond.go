package graphql

import (
	"context"
	"errors"
	"fmt"

	"github.com/saturn-xiv/palm/loquat/models"
	v2 "github.com/saturn-xiv/palm/loquat/router/v2"
	"gorm.io/gorm"
)

func (p *Mutation) BondWan(ctx context.Context, args struct {
	Interfaces []string
	Enable     bool
}) (*Ok, error) {
	if _, _, err := current_user(ctx, p.db, p.secrets); err != nil {
		return nil, err
	}

	var bond InternetBond
	err := models.GetB(p.db, bondKey(v2.WAN), &bond)
	if err != nil && !errors.Is(err, gorm.ErrRecordNotFound) {
		return nil, err
	}
	bond.interfaces = args.Interfaces
	bond.enable = args.Enable
	if err = models.SetB(p.db, bondKey(v2.WAN), &bond); err != nil {
		return nil, err
	}

	return &Ok{}, nil
}

func (p *Mutation) BondLan(ctx context.Context, args struct {
	Interfaces []string
	Address    string
	Enable     bool
}) (*Ok, error) {
	return p.bond_intranet_bond(ctx, args, v2.LAN)
}

func (p *Mutation) bond_intranet_bond(ctx context.Context, args struct {
	Interfaces []string
	Address    string
	Enable     bool
}, id string) (*Ok, error) {
	if _, _, err := current_user(ctx, p.db, p.secrets); err != nil {
		return nil, err
	}

	var bond IntranetBond
	err := models.GetB(p.db, bondKey(id), &bond)
	if err != nil && !errors.Is(err, gorm.ErrRecordNotFound) {
		return nil, err
	}
	bond.interfaces = args.Interfaces
	bond.address = args.Address
	bond.enable = args.Enable
	if err = models.SetB(p.db, bondKey(id), &bond); err != nil {
		return nil, err
	}

	return &Ok{}, nil
}

func (p *Mutation) BondDmz(ctx context.Context, args struct {
	Interfaces []string
	Address    string
	Enable     bool
}) (*Ok, error) {
	return p.bond_intranet_bond(ctx, args, v2.DMZ)
}

func (p *Query) BondDmz(ctx context.Context) (*IntranetBond, error) {
	return p.bond_intranet(ctx, v2.DMZ)
}

func (p *Query) BondLan(ctx context.Context) (*IntranetBond, error) {
	return p.bond_intranet(ctx, v2.LAN)
}

func (p *Query) BondWan(ctx context.Context) (*InternetBond, error) {
	if _, _, err := current_user(ctx, p.db, p.secrets); err != nil {
		return nil, err
	}

	var bond InternetBond
	err := models.GetB(p.db, bondKey(v2.WAN), &bond)
	if err != nil && !errors.Is(err, gorm.ErrRecordNotFound) {
		return nil, err
	}

	return &bond, nil
}

func (p *Query) bond_intranet(ctx context.Context, id string) (*IntranetBond, error) {
	if _, _, err := current_user(ctx, p.db, p.secrets); err != nil {
		return nil, err
	}

	var bond IntranetBond
	err := models.GetB(p.db, bondKey(id), &bond)
	if err != nil && !errors.Is(err, gorm.ErrRecordNotFound) {
		return nil, err
	}

	return &bond, nil
}
func bondKey(label string) string {
	return fmt.Sprintf("bond.%s", label)
}

type InternetBond struct {
	interfaces []string
	enable     bool
}

func (p *InternetBond) Interfaces() []string {
	return p.interfaces
}

func (p *InternetBond) Enable() bool {
	return p.enable
}

type IntranetBond struct {
	interfaces []string
	address    string
	enable     bool
}

func (p *IntranetBond) Interfaces() []string {
	return p.interfaces
}
func (p *IntranetBond) Address() string {
	return p.address
}

func (p *IntranetBond) Enable() bool {
	return p.enable
}
