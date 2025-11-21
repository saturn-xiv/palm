package graphql

import (
	"context"
	"errors"
	"log/slog"
	"net"
	"syscall"

	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/loquat/models"
	"github.com/saturn-xiv/palm/loquat/router"
	v2 "github.com/saturn-xiv/palm/loquat/router/v2"
)

func (p *Mutation) Apply(ctx context.Context, args struct{ Run bool }) (*Ok, error) {
	if _, _, err := current_user(ctx, p.db, p.secrets); err != nil {
		return nil, err
	}
	item, err := router.Export(p.db)
	if err != nil {
		return nil, err
	}
	if err = item.Apply(args.Run); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}

func (p *Mutation) Reboot(ctx context.Context) (*Ok, error) {
	if _, _, err := current_user(ctx, p.db, p.secrets); err != nil {
		return nil, err
	}
	go func() {
		if err := syscall.Reboot(syscall.LINUX_REBOOT_CMD_RESTART); err != nil {
			slog.Error(err.Error())
		}
	}()
	return &Ok{}, nil
}

func (p *Query) IndexNetworkInterface(ctx context.Context) ([]*NetworkInterface, error) {
	if _, _, err := current_user(ctx, p.db, p.secrets); err != nil {
		return nil, err
	}
	ifaces, err := net.Interfaces()
	if err != nil {
		return nil, err
	}
	var items []*NetworkInterface
	for _, iface := range ifaces {
		if iface.Name == "lo" {
			continue
		}
		it, err := NewNetworkInterface(p.db, &iface)
		if err != nil {
			return nil, err
		}
		items = append(items, it)
	}

	return items, nil
}
func (p *Query) ShowNetworkInterface(ctx context.Context, args struct{ Name string }) (*NetworkInterface, error) {
	if _, _, err := current_user(ctx, p.db, p.secrets); err != nil {
		return nil, err
	}
	iface, err := net.InterfaceByName(args.Name)
	if err != nil {
		return nil, err
	}
	return NewNetworkInterface(p.db, iface)
}

type NetworkInterface struct {
	item *net.Interface
	memo string
}

func NewNetworkInterface(db *gorm.DB, iface *net.Interface) (*NetworkInterface, error) {
	var profile v2.Internet
	err := models.GetProtobuf(db, networkInterfaceKey(iface.Name), &profile)
	if err != nil && !errors.Is(err, gorm.ErrRecordNotFound) {
		return nil, err
	}
	return &NetworkInterface{item: iface, memo: profile.Memo}, nil

}

func (p *NetworkInterface) Name() string {
	return p.item.Name
}

func (p *NetworkInterface) Memo() string {
	return p.memo
}

func (p *NetworkInterface) Mtu() int32 {
	return int32(p.item.MTU)
}
func (p *NetworkInterface) HardwareAddress() string {
	return p.item.HardwareAddr.String()
}
func (p *NetworkInterface) Addresses() ([]string, error) {
	addrs, err := p.item.Addrs()
	if err != nil {
		return nil, err
	}
	var items []string
	for _, addr := range addrs {
		items = append(items, addr.String())
	}
	return items, nil
}

func (p *NetworkInterface) MulticastAddresses() ([]string, error) {
	addrs, err := p.item.MulticastAddrs()
	if err != nil {
		return nil, err
	}
	var items []string
	for _, addr := range addrs {
		items = append(items, addr.String())
	}
	return items, nil
}
