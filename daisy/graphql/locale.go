package graphql

import (
	"context"

	"github.com/graph-gophers/graphql-go"

	"github.com/saturn-xiv/palm/daisy/models"
)

func (p *Query) IndexLocale(ctx context.Context, args struct {
	Page Page
}) (*IndexLocaleResponse, error) {
	total, err := models.CountLocale(p.db)
	if err != nil {
		return nil, err
	}
	pagination := NewPagination(&args.Page, uint(total))
	var items []models.Locale
	if err := p.db.Offset(int(pagination.current.Offset())).Limit(int(pagination.current.Size)).Order("updated_at DESC").Find(&items).Error; err != nil {
		return nil, err
	}
	return &IndexLocaleResponse{items, pagination}, nil
}

func (p *Query) GetLocaleByLang(ctx context.Context, args struct {
	Lang string
}) ([]*Locale, error) {
	var items []models.Locale
	if err := p.db.Where("lang = ?", args.Lang).Order("code ASC").Find(&items).Error; err != nil {
		return nil, err
	}
	var res []*Locale
	for _, it := range items {
		res = append(res, &Locale{item: &it})
	}
	return res, nil
}

type Locale struct {
	item *models.Locale
}

func (p *Locale) Id() graphql.ID {
	return ToId(p.item.ID)
}
func (p *Locale) Lang() string {
	return p.item.Lang
}
func (p *Locale) Code() string {
	return p.item.Code
}
func (p *Locale) Message() string {
	return p.item.Message
}
func (p *Locale) UpdatedAt() graphql.Time {
	return graphql.Time{Time: p.item.UpdatedAt}
}

type IndexLocaleResponse struct {
	items      []models.Locale
	pagination *Pagination
}

func (p *IndexLocaleResponse) Items() []*Locale {
	var items []*Locale
	for _, it := range p.items {
		items = append(items, &Locale{item: &it})
	}
	return items
}

func (p *IndexLocaleResponse) Pagination() *Pagination {
	return p.pagination
}
