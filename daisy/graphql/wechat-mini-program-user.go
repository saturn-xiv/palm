package graphql

import (
	"context"

	"github.com/graph-gophers/graphql-go"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/models"
)

func (p *Query) IndexWechatMiniProgramUser(ctx context.Context, args struct {
	Page Page
}) (*IndexWechatMiniProgramUserResponse, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := ss.User.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	var total int64
	if err := p.db.Model(&models.WechatMiniProgramUser{}).Count(&total).Error; err != nil {
		return nil, err
	}
	pagination := NewPagination(&args.Page, uint(total))
	var items []models.WechatMiniProgramUser
	if err := p.db.Offset(int(pagination.current.Offset())).Limit(int(pagination.current.Size)).Order("updated_at DESC").Find(&items).Error; err != nil {
		return nil, err
	}
	return &IndexWechatMiniProgramUserResponse{db: p.db, items: items, pagination: pagination}, nil
}

type IndexWechatMiniProgramUserResponse struct {
	db         *gorm.DB
	items      []models.WechatMiniProgramUser
	pagination *Pagination
}

func (p *IndexWechatMiniProgramUserResponse) Items() []*WechatMiniProgramUser {
	var items []*WechatMiniProgramUser
	for _, it := range p.items {
		items = append(items, &WechatMiniProgramUser{item: &it, db: p.db})
	}
	return items
}

func (p *IndexWechatMiniProgramUserResponse) Pagination() *Pagination {
	return p.pagination
}

type WechatMiniProgramUser struct {
	item *models.WechatMiniProgramUser
	db   *gorm.DB
}

func (p *WechatMiniProgramUser) Id() graphql.ID {
	return ToId(p.item.ID)
}
func (p *WechatMiniProgramUser) CreatedAt() graphql.Time {
	return graphql.Time{Time: p.item.CreatedAt}
}
func (p *WechatMiniProgramUser) UpdatedAt() graphql.Time {
	return graphql.Time{Time: p.item.UpdatedAt}
}
func (p *WechatMiniProgramUser) User() (*UserDetails, error) {
	var it models.User
	if err := p.db.Unscoped().First(&it, p.item.UserID).Error; err != nil {
		return nil, err
	}
	return &UserDetails{item: &it}, nil
}
func (p *WechatMiniProgramUser) Nickname() *string {
	return p.item.Nickname
}
func (p *WechatMiniProgramUser) AvatarUrl() *string {
	return p.item.AvatarUrl
}
