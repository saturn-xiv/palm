package graphql

import (
	"fmt"
	"net"

	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/loquat/models"
	v2 "github.com/saturn-xiv/palm/loquat/router/v2"
)

func (p *Query) Interfaces() ([]string, error) {
	ifs, err := net.Interfaces()
	if err != nil {
		return nil, err
	}
	var items []string
	for _, it := range ifs {
		items = append(items, it.Name)
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
