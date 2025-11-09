package graphql

import "net"

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
