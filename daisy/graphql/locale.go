package graphql

import (
	"context"
	"strings"

	"github.com/graph-gophers/graphql-go"
	"golang.org/x/text/language"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/models"
)

func (p *Mutation) SetLocale(ctx context.Context, args struct {
	Lang    string
	Code    string
	Message string
}) (*Ok, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err = ss.User.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	lang, err := language.Parse(args.Lang)
	if err != nil {
		return nil, err
	}

	form := setLocaleForm{Lang: lang.String(), Code: strings.ToLower(strings.TrimSpace(args.Code)), Message: args.Message}
	if err := gl_validate.Struct(&form); err != nil {
		return nil, err
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		return models.SetLocale(tx, form.Lang, form.Code, form.Message)
	}); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}

type setLocaleForm struct {
	Lang    string `validate:"required,gte=1,lte=15"`
	Code    string `validate:"required,gte=2,lte=255"`
	Message string `validate:"required,gte=1"`
}

func (p *Mutation) DestroyLocale(ctx context.Context, args struct {
	Id graphql.ID
}) (*Ok, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := ss.User.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	id, err := FromId(args.Id)
	if err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if err := tx.Where("id = ?", id).Delete(&models.Locale{}).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}

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
