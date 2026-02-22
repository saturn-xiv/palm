package graphql

import (
	"context"

	"github.com/saturn-xiv/palm/daisy/iso4217"
)

func (p *Query) Currencies(ctx context.Context) ([]*Currency, error) {
	items, err := iso4217.Iso4217()
	if err != nil {
		return nil, err
	}
	var res []*Currency
	for _, it := range items.Items {
		res = append(res, &Currency{item: &it})
	}
	return res, nil
}

type Currency struct {
	item *iso4217.Item
}

func (p *Currency) Name() string {
	return p.item.Name
}
func (p *Currency) Country() string {
	return p.item.Country
}
func (p *Currency) Code() string {
	return p.item.Code
}
func (p *Currency) Number() int32 {
	return int32(p.item.Number)
}
func (p *Currency) IsFund() *bool {
	return p.item.Units.Fund
}
