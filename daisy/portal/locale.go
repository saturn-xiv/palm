package portal

import (
	"context"

	"github.com/casbin/casbin/v3"
	"google.golang.org/protobuf/types/known/emptypb"
	"google.golang.org/protobuf/types/known/timestamppb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/crypto"
	"github.com/saturn-xiv/palm/daisy/models"
	v2 "github.com/saturn-xiv/palm/daisy/portal/v2"
	"github.com/saturn-xiv/palm/daisy/rbac"
)

type LocaleServer struct {
	v2.UnimplementedLocaleServer

	db       *gorm.DB
	enforcer *casbin.Enforcer
	jwt      *crypto.Jwt
}

func NewLocaleServer(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer) *LocaleServer {
	return &LocaleServer{db: db, jwt: jwt, enforcer: enforcer}
}

func (p *LocaleServer) Index(ctx context.Context, req *v2.Page) (*v2.LocaleIndexResponse, error) {
	{
		user, _, err := rbac.CurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err = rbac.IsAdministrator(p.enforcer, user); err != nil {
			return nil, err
		}
	}

	total, err := models.CountLocale(p.db)
	if err != nil {
		return nil, err
	}
	pagination := v2.NewPagination(req, total)
	var items []models.Locale
	if err := p.db.Model(&models.Locale{}).Order("updated_at DESC").Limit(int(pagination.Current.Size)).Offset(int(pagination.Current.Offset())).Find(&items).Error; err != nil {
		return nil, err
	}
	res := v2.LocaleIndexResponse{
		Items:      []*v2.LocaleIndexResponse_Item{},
		Pagination: pagination,
	}
	for _, it := range items {
		res.Items = append(res.Items, &v2.LocaleIndexResponse_Item{
			Id:        int64(it.ID),
			Lang:      it.Lang,
			Code:      it.Code,
			Message:   it.Message,
			UpdatedAt: timestamppb.New(it.UpdatedAt),
		})
	}
	return &res, nil
}
func (p *LocaleServer) Set(ctx context.Context, req *v2.LocaleSetRequest) (*emptypb.Empty, error) {
	{
		user, _, err := rbac.CurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err = rbac.IsAdministrator(p.enforcer, user); err != nil {
			return nil, err
		}
	}
	if err := models.SetLocale(p.db, req.Lang, req.Code, req.Message); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *LocaleServer) ByLang(ctx context.Context, req *v2.LocaleByLangRequest) (*v2.LocaleByLangResponse, error) {
	var items []models.Locale
	if err := p.db.Model(&models.Locale{}).Where("lang = ?", req.Lang).Order("code ASC").Find(&items).Error; err != nil {
		return nil, err
	}
	res := v2.LocaleByLangResponse{
		Items: []*v2.LocaleIndexResponse_Item{},
	}
	for _, it := range items {
		res.Items = append(res.Items, &v2.LocaleIndexResponse_Item{
			Id:        int64(it.ID),
			Lang:      it.Lang,
			Code:      it.Code,
			Message:   it.Message,
			UpdatedAt: timestamppb.New(it.UpdatedAt),
		})
	}
	return &res, nil
}
