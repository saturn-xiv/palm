package portal

import (
	"context"
	"slices"
	"strconv"

	"google.golang.org/protobuf/types/known/emptypb"

	"github.com/saturn-xiv/palm/daisy/iso4217"
	v2 "github.com/saturn-xiv/palm/daisy/portal/v2"
)

type SiteServer struct {
	v2.UnimplementedSiteServer
}

func NewSiteServer() *SiteServer {
	return &SiteServer{}
}

func (p *SiteServer) Currencies(ctx context.Context, req *emptypb.Empty) (*v2.CurrenciesResponse, error) {
	items, err := iso4217.Iso4217()
	if err != nil {
		return nil, err
	}
	var res v2.CurrenciesResponse
	for _, it := range items.Items {
		cur := v2.CurrenciesResponse_Item{
			Name:    it.Name,
			Country: it.Country,
			Code:    it.Code,
			IsFund:  it.Units.Fund,
			Number:  uint32(it.Number),
		}
		if !slices.Contains([]string{"", "N.A."}, it.Units.Value) {
			uns, err := strconv.ParseUint(it.Units.Value, 10, 32)
			if err != nil {
				return nil, err
			}
			val := uint32(uns)
			cur.Units = &val
		}
		res.Items = append(res.Items, &cur)
	}

	return &res, nil
}
