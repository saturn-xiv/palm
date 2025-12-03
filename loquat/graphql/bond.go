package graphql

import (
	"context"
	"errors"
	"fmt"

	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/loquat/models"
)

func (p *Mutation) InternetBond(ctx context.Context, args struct {
	Name       string
	Interfaces []string
	Enable     bool
}) (*Ok, error) {
	user, ip, err := current_user(ctx, p.db, p.secrets)
	if err != nil {
		return nil, err
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var bond InternetBond
		err := models.GetB(tx, bondKey(args.Name), &bond)
		if err != nil && !errors.Is(err, gorm.ErrRecordNotFound) {
			return err
		}
		bond.interfaces = args.Interfaces
		bond.enable = args.Enable
		if err = models.SetB(tx, bondKey(args.Name), &bond); err != nil {
			return err
		}

		return tx.Create(&models.Log{UserID: user.ID, Ip: ip, Message: fmt.Sprintf("set %s to %v", args.Name, args.Interfaces)}).Error
	}); err != nil {
		return nil, err
	}

	return &Ok{}, nil
}

func (p *Mutation) IntranetBond(ctx context.Context, args struct {
	Name       string
	Interfaces []string
	Address    string
	Dns        string
	Enable     bool
}) (*Ok, error) {
	user, ip, err := current_user(ctx, p.db, p.secrets)
	if err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var bond IntranetBond
		err := models.GetB(tx, bondKey(args.Name), &bond)
		if err != nil && !errors.Is(err, gorm.ErrRecordNotFound) {
			return err
		}
		bond.interfaces = args.Interfaces
		bond.address = args.Address
		bond.enable = args.Enable
		bond.dns = args.Dns
		if err = models.SetB(tx, bondKey(args.Name), &bond); err != nil {
			return err
		}

		return tx.Create(&models.Log{UserID: user.ID, Ip: ip, Message: fmt.Sprintf("set %s to %v(%s)", args.Name, args.Interfaces, args.Address)}).Error
	}); err != nil {
		return nil, err
	}

	return &Ok{}, nil
}

func (p *Query) InternetBond(ctx context.Context, args struct{ Name string }) (*InternetBond, error) {
	if _, _, err := current_user(ctx, p.db, p.secrets); err != nil {
		return nil, err
	}

	var bond InternetBond
	err := models.GetB(p.db, bondKey(args.Name), &bond)
	if err != nil && !errors.Is(err, gorm.ErrRecordNotFound) {
		return nil, err
	}

	return &bond, nil
}

func (p *Query) IntranetBond(ctx context.Context, args struct{ Name string }) (*IntranetBond, error) {
	if _, _, err := current_user(ctx, p.db, p.secrets); err != nil {
		return nil, err
	}

	var bond IntranetBond
	err := models.GetB(p.db, bondKey(args.Name), &bond)
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
	dns        string
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

func (p *IntranetBond) Dns() string {
	return p.dns
}
