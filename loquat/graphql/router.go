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

func (p *Query) IndexNetworkInterface(ctx context.Context) ([]*NetworkInterface, error) {
	if _, _, err := current_user(ctx, p.db, p.jwt_key); err != nil {
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
	if _, _, err := current_user(ctx, p.db, p.jwt_key); err != nil {
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
	memo *string
}

func NewNetworkInterface(db *gorm.DB, iface *net.Interface) (*NetworkInterface, error) {
	profile, err := getNetworkInterface(db, iface)
	if err == nil {
		return &NetworkInterface{item: iface, memo: &profile.Label}, nil
	}

	if errors.Is(err, gorm.ErrRecordNotFound) {
		return &NetworkInterface{item: iface}, nil
	}
	return nil, err
}

func (p *NetworkInterface) Name() string {
	return p.item.Name
}

func (p *NetworkInterface) Memo() *string {
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

func networkInterfaceKey(it *net.Interface) string {
	return fmt.Sprintf("net.%s", it.Name)
}

func setNetworkInterface(db *gorm.DB, it *net.Interface, profile *v2.Ethernet) error {
	return models.SetProtobuf(db, networkInterfaceKey(it), profile)
}

func getNetworkInterface(db *gorm.DB, it *net.Interface) (*v2.Ethernet, error) {
	var profile v2.Ethernet
	if err := models.GetProtobuf(db, networkInterfaceKey(it), &profile); err != nil {
		return nil, err
	}
	return &profile, nil
}
