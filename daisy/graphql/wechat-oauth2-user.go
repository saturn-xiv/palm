package graphql

import (
	"context"

	"github.com/graph-gophers/graphql-go"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/models"
)

func (p *Query) IndexWechatOauth2User(ctx context.Context, args struct {
	Page Page
}) (*IndexWechatOauth2UserResponse, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := ss.User.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	var total int64
	if err := p.db.Model(&models.WechatOauth2User{}).Count(&total).Error; err != nil {
		return nil, err
	}
	pagination := NewPagination(&args.Page, uint(total))
	var items []models.WechatOauth2User
	if err := p.db.Offset(int(pagination.current.Offset())).Limit(int(pagination.current.Size)).Order("updated_at DESC").Find(&items).Error; err != nil {
		return nil, err
	}
	return &IndexWechatOauth2UserResponse{db: p.db, items: items, pagination: pagination}, nil
}

type IndexWechatOauth2UserResponse struct {
	db         *gorm.DB
	items      []models.WechatOauth2User
	pagination *Pagination
}

func (p *IndexWechatOauth2UserResponse) Items() []*WechatOauth2User {
	var items []*WechatOauth2User
	for _, it := range p.items {
		items = append(items, &WechatOauth2User{item: &it, db: p.db})
	}
	return items
}
func (p *IndexWechatOauth2UserResponse) Pagination() *Pagination {
	return p.pagination
}

type WechatOauth2User struct {
	item *models.WechatOauth2User
	db   *gorm.DB
}

func (p *WechatOauth2User) Id() graphql.ID {
	return ToId(p.item.ID)
}
func (p *WechatOauth2User) CreatedAt() graphql.Time {
	return graphql.Time{Time: p.item.CreatedAt}
}
func (p *WechatOauth2User) UpdatedAt() graphql.Time {
	return graphql.Time{Time: p.item.UpdatedAt}
}
func (p *WechatOauth2User) User() (*UserDetails, error) {
	var it models.User
	if err := p.db.Unscoped().First(&it, p.item.UserID).Error; err != nil {
		return nil, err
	}
	return &UserDetails{item: &it}, nil
}
func (p *WechatOauth2User) Nickname() string {
	return p.item.Nickname
}
func (p *WechatOauth2User) Sex() int32 {
	return int32(p.item.Sex)
}
func (p *WechatOauth2User) City() string {
	return p.item.City
}
func (p *WechatOauth2User) Province() string {
	return p.item.Province
}
func (p *WechatOauth2User) Country() string {
	return p.item.Country
}
func (p *WechatOauth2User) HeadImgUrl() *string {
	return p.item.HeadImgUrl
}
func (p *WechatOauth2User) Lang() string {
	return p.item.Lang
}
