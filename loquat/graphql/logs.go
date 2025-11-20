package graphql

import (
	"context"
	"time"

	graphql "github.com/graph-gophers/graphql-go"

	"github.com/saturn-xiv/palm/loquat/models"
)

func (p *Query) IndexLog(ctx context.Context, args struct {
	Page Page
}) (*IndexLogResponse, error) {
	if _, _, err := current_user(ctx, p.db, p.jwt_key); err != nil {
		return nil, err
	}
	var total int64
	if err := p.db.Model(&models.Log{}).Count(&total).Error; err != nil {
		return nil, err
	}
	pagination := NewPagination(&args.Page, uint(total))
	var items []models.Log
	if err := p.db.Order("created_at DESC").Offset(int(pagination.current.Offset())).Limit(int(pagination.current.Size)).Find(&items).Error; err != nil {
		return nil, err
	}
	return &IndexLogResponse{items: items, pagination: pagination}, nil
}

type Log struct {
	item *models.Log
}

func (p *Log) Id() graphql.ID {
	return ToId(p.item.ID)
}

func (p *Log) Message() string {
	return p.item.Message
}
func (p *Log) Ip() string {
	return p.item.Ip
}
func (p *Log) CreatedAt() graphql.Time {
	now := time.Now()
	return graphql.Time{Time: now}
}

type IndexLogResponse struct {
	items      []models.Log
	pagination *Pagination
}

func (p *IndexLogResponse) Pagination() *Pagination {
	return p.pagination
}

func (p *IndexLogResponse) Items() ([]*Log, error) {
	var items []*Log
	for _, it := range p.items {
		items = append(items, &Log{item: &it})
	}
	return items, nil
}
