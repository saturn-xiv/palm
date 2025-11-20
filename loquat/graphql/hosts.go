package graphql

import (
	"context"

	graphql "github.com/graph-gophers/graphql-go"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/loquat/models"
)

func (p *Query) IndexHost(ctx context.Context) ([]*Host, error) {
	if _, _, err := current_user(ctx, p.db, p.jwt_key); err != nil {
		return nil, err
	}
	var hosts []models.Host
	if err := p.db.Order("updated_at DESC").Find(&hosts).Error; err != nil {
		return nil, err
	}
	var items []*Host
	for _, host := range hosts {
		it, err := NewHost(p.db, &host)
		if err != nil {
			return nil, err
		}
		items = append(items, it)
	}
	return items, nil
}
func (p *Query) ShowHost(ctx context.Context, args struct{ Id graphql.ID }) (*Host, error) {
	if _, _, err := current_user(ctx, p.db, p.jwt_key); err != nil {
		return nil, err
	}
	id, err := FromId(string(args.Id))
	if err != nil {
		return nil, err
	}
	var host models.Host
	if err = p.db.Where(map[string]interface{}{"id": id}).Take(&host).Error; err != nil {
		return nil, err
	}
	return NewHost(p.db, &host)
}

type Host struct {
	item   *models.Host
	member *Member
}

func NewHost(db *gorm.DB, host *models.Host) (*Host, error) {
	res := Host{
		item: host,
	}
	if host.MemberID != nil {
		var member models.Member
		if err := db.Where(map[string]interface{}{"id": *host.MemberID}).Take(&member).Error; err != nil {
			return nil, err
		}
		res.member = &Member{item: &member}
	}
	return &res, nil
}

func (p *Host) Id() graphql.ID {
	return ToId(p.item.ID)
}
func (p *Host) Name() *string {
	return p.item.Name
}

func (p *Host) Mac() string {
	return p.item.Mac
}

func (p *Host) Ip() string {
	return p.item.Ip
}

func (p *Host) Vendor() *string {
	return p.item.Vendor
}

func (p *Host) Network() string {
	return p.item.Network
}

func (p *Host) Fixed() bool {
	return p.item.Fixed
}

func (p *Host) UpdatedAt() graphql.Time {
	return graphql.Time{Time: p.item.UpdatedAt}
}

func (p *Host) Member() *Member {
	return p.member
}
