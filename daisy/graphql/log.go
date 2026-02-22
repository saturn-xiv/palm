package graphql

import (
	"context"

	"github.com/graph-gophers/graphql-go"

	"github.com/saturn-xiv/palm/daisy/models"
)

func (p *Query) IndexLog(ctx context.Context, args struct {
	Page Page
}) (*IndexLogResponse, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	var total int64
	if err := p.db.Model(&models.Log{}).Where("user_id = ?", ss.User.Id).Count(&total).Error; err != nil {
		return nil, err
	}
	pagination := NewPagination(&args.Page, uint(total))
	var items []models.Log
	if err := p.db.Where("user_id = ?", ss.User.Id).Offset(int(pagination.current.Offset())).Limit(int(pagination.current.Size)).Order("created_at DESC").Find(&items).Error; err != nil {
		return nil, err
	}
	return &IndexLogResponse{items, pagination}, nil
}

type Log struct {
	item *models.Log
}

func (p *Log) Id() graphql.ID {
	return ToId(p.item.ID)
}
func (p *Log) Level() int32 {
	return p.item.Level
}
func (p *Log) Ip() string {
	return p.item.Ip
}
func (p *Log) Plugin() string {
	return p.item.Plugin
}
func (p *Log) Message() string {
	return p.item.Message
}
func (p *Log) CreatedAt() graphql.Time {
	return graphql.Time{Time: p.item.CreatedAt}
}

type IndexLogResponse struct {
	items      []models.Log
	pagination *Pagination
}

func (p *IndexLogResponse) Pagination() *Pagination {
	return p.pagination
}
func (p *IndexLogResponse) Items() []*Log {
	var items []*Log
	for _, it := range p.items {
		items = append(items, &Log{item: &it})
	}
	return items
}
