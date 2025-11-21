package graphql

import (
	"context"
	"errors"
	"reflect"
	"time"

	graphql "github.com/graph-gophers/graphql-go"
	"google.golang.org/protobuf/proto"

	"github.com/saturn-xiv/palm/loquat/models"
	v2 "github.com/saturn-xiv/palm/loquat/router/v2"
)

// allowPing(device: String!, sortOrder: Int!, memo: String!): Ok!
//   allowInput(
//     device: String!
//     tcp: Boolean!
//     port: Int!
//     sortOrder: Int!
//     memo: String!
//   ): Ok!
//   allowNat(
//     device: String!
//     tcp: Boolean!
//     sourcePort: Int!
//     destinationIp: String!
//     destinationPort: Int!
//     sortOrder: Int!
//     memo: String!
//   ): Ok!
//   denyOutput(
//     address: String!
//     port: Int
//     weekdays: [Week!]!
//     beginTime: String!
//     endTime: String!
//     sortOrder: Int!
//     memo: String!
//   ): Ok!
//   limitSpeed(
//     speed: Int!
//     weekdays: [Week!]!
//     beginTime: String!
//     endTime: String!
//     sortOrder: Int!
//     memo: String!
//   ): Ok!
//   destroyFirewallRule(id: ID!): Ok!
//   associateRuleWithMember(rule: ID!, member: ID!): Ok!
//   dissociateRuleFromMember(rule: ID!, member: ID!): Ok!

func (p *Query) IndexFirewallRule(ctx context.Context) ([]*FirewallRule, error) {
	if _, _, err := current_user(ctx, p.db, p.secrets); err != nil {
		return nil, err
	}
	var rules []models.Rule
	if err := p.db.Order("updated_at DESC").Preload("Members").Find(&rules).Error; err != nil {
		return nil, err
	}
	var res []*FirewallRule
	for _, rule := range rules {
		it := FirewallRule{item: &rule}
		for _, member := range rule.Members {
			it.members = append(it.members, &Member{item: &member})
		}
		res = append(res, &it)
	}

	return res, nil
}

type Ping struct {
	item       *v2.FirewallRule_Ping
	id         uint
	sort_order int
	memo       string
	updated_at *time.Time
}

func (p *Ping) Id() graphql.ID {
	return ToId(p.id)
}
func (p *Ping) SortOrder() int32 {
	return int32(p.sort_order)
}
func (p *Ping) Memo() string {
	return p.memo
}
func (p *Ping) UpdatedAt() graphql.Time {
	return graphql.Time{Time: *p.updated_at}
}
func (p *Ping) Device() string {
	return p.item.Device
}

type SpeedLimit struct {
	item       *v2.FirewallRule_SpeedLimit
	id         uint
	sort_order int
	memo       string
	updated_at *time.Time
	members    []*Member
}

func (p *SpeedLimit) Id() graphql.ID {
	return ToId(p.id)
}
func (p *SpeedLimit) SortOrder() int32 {
	return int32(p.sort_order)
}
func (p *SpeedLimit) Memo() string {
	return p.memo
}
func (p *SpeedLimit) UpdatedAt() graphql.Time {
	return graphql.Time{Time: *p.updated_at}
}
func (p *SpeedLimit) Value() int32 {
	return int32(p.item.Value)
}
func (p *SpeedLimit) Weekdays() []string {
	var items []string
	for _, it := range p.item.Period.Days {
		items = append(items, it.String())
	}
	return items
}
func (p *SpeedLimit) BeginTime() string {
	return p.item.Period.Begin.ToString()
}
func (p *SpeedLimit) EndTime() string {
	return p.item.Period.End.ToString()
}
func (p *SpeedLimit) Members() []*Member {
	return p.members
}

type Nat struct {
	item       *v2.FirewallRule_Nat
	id         uint
	sort_order int
	memo       string
	updated_at *time.Time
}

func (p *Nat) Id() graphql.ID {
	return ToId(p.id)
}
func (p *Nat) SortOrder() int32 {
	return int32(p.sort_order)
}
func (p *Nat) Memo() string {
	return p.memo
}
func (p *Nat) UpdatedAt() graphql.Time {
	return graphql.Time{Time: *p.updated_at}
}
func (p *Nat) Device() string {
	return p.item.Device
}
func (p *Nat) Tcp() bool {
	return p.item.Protocol == v2.FirewallRule_Tcp
}
func (p *Nat) DestinationPort() int32 {
	return int32(p.item.Destination.Port)
}
func (p *Nat) DestinationIp() string {
	return p.item.Destination.Ip
}
func (p *Nat) Port() int32 {
	return int32(p.item.Port)
}

type Input struct {
	item       *v2.FirewallRule_Input
	id         uint
	sort_order int
	memo       string
	updated_at *time.Time
}

func (p *Input) Id() graphql.ID {
	return ToId(p.id)
}
func (p *Input) SortOrder() int32 {
	return int32(p.sort_order)
}
func (p *Input) Memo() string {
	return p.memo
}
func (p *Input) UpdatedAt() graphql.Time {
	return graphql.Time{Time: *p.updated_at}
}
func (p *Input) Device() string {
	return p.item.Device
}
func (p *Input) Tcp() bool {
	return p.item.Protocol == v2.FirewallRule_Tcp
}
func (p *Input) Port() int32 {
	return int32(p.item.Port)
}

type Output struct {
	item       *v2.FirewallRule_Output
	id         uint
	sort_order int
	memo       string
	updated_at *time.Time
	members    []*Member
}

func (p *Output) Id() graphql.ID {
	return ToId(p.id)
}
func (p *Output) SortOrder() int32 {
	return int32(p.sort_order)
}
func (p *Output) Memo() string {
	return p.memo
}
func (p *Output) UpdatedAt() graphql.Time {
	return graphql.Time{Time: *p.updated_at}
}
func (p *Output) Address() string {
	return p.item.Address
}
func (p *Output) Port() int32 {
	return int32(*p.item.Port)
}
func (p *Output) Weekdays() []string {
	var items []string
	for _, it := range p.item.Period.Days {
		items = append(items, it.String())
	}
	return items
}
func (p *Output) BeginTime() string {
	return p.item.Period.Begin.ToString()
}
func (p *Output) EndTime() string {
	return p.item.Period.End.ToString()
}
func (p *Output) Members() []*Member {
	return p.members
}

type FirewallRule struct {
	item    *models.Rule
	members []*Member
}

func (p *FirewallRule) ToOutput() (*Output, error) {
	switch p.item.Type {
	case reflect.TypeOf((*v2.FirewallRule_Output)(nil)).Name():
		var it v2.FirewallRule_Output
		if err := proto.Unmarshal(p.item.Content, &it); err != nil {
			return nil, err
		}
		return &Output{
			item:       &it,
			id:         p.item.ID,
			sort_order: p.item.SortOrder,
			memo:       p.item.Memo,
			updated_at: &p.item.UpdatedAt,
			members:    p.members,
		}, nil
	default:
		return nil, errors.New("not a output rule")
	}
}

func (p *FirewallRule) ToInput() (*Input, error) {
	switch p.item.Type {
	case reflect.TypeOf((*v2.FirewallRule_Input)(nil)).Name():
		var it v2.FirewallRule_Input
		if err := proto.Unmarshal(p.item.Content, &it); err != nil {
			return nil, err
		}
		return &Input{
			item:       &it,
			id:         p.item.ID,
			sort_order: p.item.SortOrder,
			memo:       p.item.Memo,
			updated_at: &p.item.UpdatedAt,
		}, nil
	default:
		return nil, errors.New("not a input rule")
	}
}

func (p *FirewallRule) ToNat() (*Nat, error) {
	switch p.item.Type {
	case reflect.TypeOf((*v2.FirewallRule_Nat)(nil)).Name():
		var it v2.FirewallRule_Nat
		if err := proto.Unmarshal(p.item.Content, &it); err != nil {
			return nil, err
		}
		return &Nat{
			item:       &it,
			id:         p.item.ID,
			sort_order: p.item.SortOrder,
			memo:       p.item.Memo,
			updated_at: &p.item.UpdatedAt,
		}, nil
	default:
		return nil, errors.New("not a nat rule")
	}
}

func (p *FirewallRule) ToPing() (*Ping, error) {
	switch p.item.Type {
	case reflect.TypeOf((*v2.FirewallRule_Ping)(nil)).Name():
		var it v2.FirewallRule_Ping
		if err := proto.Unmarshal(p.item.Content, &it); err != nil {
			return nil, err
		}
		return &Ping{
			item:       &it,
			id:         p.item.ID,
			sort_order: p.item.SortOrder,
			memo:       p.item.Memo,
			updated_at: &p.item.UpdatedAt,
		}, nil
	default:
		return nil, errors.New("not a ping rule")
	}
}

func (p *FirewallRule) ToSpeedLimit() (*SpeedLimit, error) {
	switch p.item.Type {
	case reflect.TypeOf((*v2.FirewallRule_SpeedLimit)(nil)).Name():
		var it v2.FirewallRule_SpeedLimit
		if err := proto.Unmarshal(p.item.Content, &it); err != nil {
			return nil, err
		}
		return &SpeedLimit{
			item:       &it,
			id:         p.item.ID,
			sort_order: p.item.SortOrder,
			memo:       p.item.Memo,
			updated_at: &p.item.UpdatedAt,
			members:    p.members,
		}, nil
	default:
		return nil, errors.New("not a speed-limit rule")
	}
}
