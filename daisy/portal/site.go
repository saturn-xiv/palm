package portal

import (
	"context"
	"slices"
	"strconv"

	"google.golang.org/protobuf/types/known/emptypb"
	"gorm.io/gorm"

	"github.com/casbin/casbin/v3"
	"github.com/saturn-xiv/palm/daisy/crypto"
	"github.com/saturn-xiv/palm/daisy/iso4217"
	v2 "github.com/saturn-xiv/palm/daisy/portal/v2"
)

type SiteServer struct {
	db       *gorm.DB
	enforcer *casbin.Enforcer
	jwt      *crypto.Jwt
	hmac     *crypto.Hmac
	aead     *crypto.Aead
}

func NewSiteServer(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer, hmac *crypto.Hmac, aead *crypto.Aead) *SiteServer {
	return &SiteServer{db: db, enforcer: enforcer, jwt: jwt, hmac: hmac, aead: aead}
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
