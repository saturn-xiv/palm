package graphql

import (
	"context"

	"github.com/graph-gophers/graphql-go"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/models"
)

func (p *Query) IndexSmsUser(ctx context.Context, args struct {
	Page Page
}) (*IndexSmsUserResponse, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := ss.User.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	var total int64
	if err := p.db.Model(&models.SmsUser{}).Count(&total).Error; err != nil {
		return nil, err
	}
	pagination := NewPagination(&args.Page, uint(total))
	var items []models.SmsUser
	if err := p.db.Offset(int(pagination.current.Offset())).Limit(int(pagination.current.Size)).Order("updated_at DESC").Find(&items).Error; err != nil {
		return nil, err
	}
	return &IndexSmsUserResponse{db: p.db, pagination: pagination, items: items}, nil
}

type IndexSmsUserResponse struct {
	db         *gorm.DB
	items      []models.SmsUser
	pagination *Pagination
}

func (p *IndexSmsUserResponse) Items() []*SmsUser {
	var items []*SmsUser
	for _, it := range p.items {
		items = append(items, &SmsUser{item: &it, db: p.db})
	}
	return items
}
func (p *IndexSmsUserResponse) Pagination() *Pagination {
	return p.pagination
}

type SmsUser struct {
	item *models.SmsUser
	db   *gorm.DB
}

func (p *SmsUser) Id() graphql.ID {
	return ToId(p.item.ID)
}
func (p *SmsUser) CreatedAt() graphql.Time {
	return graphql.Time{Time: p.item.CreatedAt}
}
func (p *SmsUser) UpdatedAt() graphql.Time {
	return graphql.Time{Time: p.item.UpdatedAt}
}
func (p *SmsUser) User() (*UserDetails, error) {
	var it models.User
	if err := p.db.Unscoped().First(&it, p.item.UserID).Error; err != nil {
		return nil, err
	}
	return &UserDetails{item: &it}, nil
}
func (p *SmsUser) Name() string {
	return p.item.Name
}
func (p *SmsUser) Phone() string {
	return p.item.Phone
}
func (p *SmsUser) Avatar() *string {
	return p.item.Avatar
}
