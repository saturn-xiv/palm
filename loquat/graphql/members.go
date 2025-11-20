package graphql

import (
	"context"

	graphql "github.com/graph-gophers/graphql-go"

	"github.com/saturn-xiv/palm/loquat/models"
)

func (p *Query) IndexMember(ctx context.Context) ([]*Member, error) {
	if _, _, err := current_user(ctx, p.db, p.jwt_key); err != nil {
		return nil, err
	}
	var members []models.Member
	if err := p.db.Order("updated_at DESC").Find(&members).Error; err != nil {
		return nil, err
	}
	var items []*Member
	for _, member := range members {
		items = append(items, &Member{item: &member})
	}
	return items, nil
}
func (p *Query) ShowMember(ctx context.Context, args struct{ Id graphql.ID }) (*Member, error) {
	if _, _, err := current_user(ctx, p.db, p.jwt_key); err != nil {
		return nil, err
	}
	id, err := FromId(string(args.Id))
	if err != nil {
		return nil, err
	}
	var member models.Member
	if err = p.db.Where(map[string]interface{}{"id": id}).Take(&member).Error; err != nil {
		return nil, err
	}
	return &Member{item: &member}, nil
}

type Member struct {
	item *models.Member
}

func (p *Member) Id() graphql.ID {
	return ToId(p.item.ID)
}

func (p *Member) UpdatedAt() graphql.Time {
	return graphql.Time{Time: p.item.UpdatedAt}
}

func (p *Member) Name() string {
	return p.item.Name
}

func (p *Member) Sn() string {
	return p.item.Sn
}

func (p *Member) Memo() string {
	return p.item.Memo
}
