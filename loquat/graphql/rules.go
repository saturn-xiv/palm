package graphql

import (
	"context"
	"errors"
	"fmt"
	"reflect"
	"slices"
	"time"

	graphql "github.com/graph-gophers/graphql-go"
	"google.golang.org/protobuf/proto"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/loquat/models"
	v2 "github.com/saturn-xiv/palm/loquat/router/v2"
)

func (p *Mutation) AllowPing(ctx context.Context, args struct {
	Id        *graphql.ID
	Device    string
	SortOrder int32
	Memo      string
}) (*Ok, error) {
	if err := p.save_firewall_rule(ctx, args.Id, &v2.FirewallRule_Ping{
		Device: args.Device,
	}, int(args.SortOrder), args.Memo); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}

func (p *Mutation) AllowInput(ctx context.Context, args struct {
	Id        *graphql.ID
	Device    string
	Tcp       bool
	Port      int32
	SortOrder int32
	Memo      string
}) (*Ok, error) {
	protocol := v2.FirewallRule_Tcp
	if !args.Tcp {
		protocol = v2.FirewallRule_Udp
	}
	if err := p.save_firewall_rule(ctx, args.Id, &v2.FirewallRule_Input{
		Device:   args.Device,
		Protocol: protocol,
		Port:     uint32(args.Port),
	}, int(args.SortOrder), args.Memo); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}

func (p *Mutation) AllowNat(ctx context.Context, args struct {
	Id              *graphql.ID
	Device          string
	Tcp             bool
	Port            int32
	DestinationIp   string
	DestinationPort int32
	SortOrder       int32
	Memo            string
}) (*Ok, error) {
	protocol := v2.FirewallRule_Tcp
	if !args.Tcp {
		protocol = v2.FirewallRule_Udp
	}
	if err := p.save_firewall_rule(ctx, args.Id, &v2.FirewallRule_Nat{
		Device:   args.Device,
		Protocol: protocol,
		Port:     uint32(args.Port),
		Destination: &v2.FirewallRule_Nat_Destination{
			Ip:   args.DestinationIp,
			Port: uint32(args.DestinationPort),
		},
	}, int(args.SortOrder), args.Memo); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}

func (p *Mutation) DenyOutput(ctx context.Context, args struct {
	Id        *graphql.ID
	Address   string
	Weekdays  []string
	BeginTime string
	EndTime   string
	SortOrder int32
	Memo      string
}) (*Ok, error) {
	begin, err := v2.NewFirewallRuleTime(args.BeginTime)
	if err != nil {
		return nil, err
	}
	end, err := v2.NewFirewallRuleTime(args.EndTime)
	if err != nil {
		return nil, err
	}
	var days []v2.FirewallRule_Week
	for _, it := range args.Weekdays {
		days = append(days, v2.FirewallRule_Week(v2.FirewallRule_Week_value[it]))

	}
	if err := p.save_firewall_rule(ctx, args.Id, &v2.FirewallRule_Output{
		Address: args.Address,
		Period: &v2.FirewallRule_Period{
			Begin: begin,
			End:   end,
			Days:  days,
		},
	}, int(args.SortOrder), args.Memo); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}

func (p *Mutation) LimitSpeed(ctx context.Context, args struct {
	Id        *graphql.ID
	Value     int32
	Weekdays  []string
	BeginTime string
	EndTime   string
	SortOrder int32
	Memo      string
}) (*Ok, error) {
	begin, err := v2.NewFirewallRuleTime(args.BeginTime)
	if err != nil {
		return nil, err
	}
	end, err := v2.NewFirewallRuleTime(args.EndTime)
	if err != nil {
		return nil, err
	}
	var days []v2.FirewallRule_Week
	for _, it := range args.Weekdays {
		days = append(days, v2.FirewallRule_Week(v2.FirewallRule_Week_value[it]))

	}
	if err := p.save_firewall_rule(ctx, args.Id, &v2.FirewallRule_SpeedLimit{
		Value: uint32(args.Value),
		Period: &v2.FirewallRule_Period{
			Begin: begin,
			End:   end,
			Days:  days,
		},
	}, int(args.SortOrder), args.Memo); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}

func (p *Mutation) DestroyFirewallRule(ctx context.Context, args struct {
	Id graphql.ID
}) (*Ok, error) {
	if _, _, err := current_user(ctx, p.db, p.secrets); err != nil {
		return nil, err
	}
	id, err := FromId(args.Id)
	if err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var it models.Rule
		if err := tx.First(&it, id).Error; err != nil {
			return err
		}
		if err := tx.Delete(&it).Error; err != nil {
			return err
		}
		if err := tx.Model(&it).Association("Members").Clear(); err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}

func (p *Mutation) AssociateRuleWithMember(ctx context.Context, args struct {
	Member graphql.ID
	Rule   graphql.ID
}) (*Ok, error) {
	if _, _, err := current_user(ctx, p.db, p.secrets); err != nil {
		return nil, err
	}
	mid, err := FromId(args.Member)
	if err != nil {
		return nil, err
	}
	rid, err := FromId(args.Rule)
	if err != nil {
		return nil, err
	}
	var member models.Member
	if err = p.db.Where(map[string]interface{}{"id": mid}).Take(&member).Error; err != nil {
		return nil, err
	}
	var rule models.Rule
	if err = p.db.Where(map[string]interface{}{"id": rid}).Take(&rule).Error; err != nil {
		return nil, err
	}
	{
		if !slices.Contains([]string{
			reflect.TypeOf((*v2.FirewallRule_Output)(nil)).Name(),
			reflect.TypeOf((*v2.FirewallRule_SpeedLimit)(nil)).Name(),
		}, rule.Type) {
			return nil, fmt.Errorf("deny for %s", rule.Type)
		}
	}
	if err = p.db.Model(&member).Association("Rules").Append(&rule); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}

func (p *Mutation) DissociateRuleFromMember(ctx context.Context, args struct {
	Member graphql.ID
	Rule   graphql.ID
}) (*Ok, error) {
	if _, _, err := current_user(ctx, p.db, p.secrets); err != nil {
		return nil, err
	}
	mid, err := FromId(args.Member)
	if err != nil {
		return nil, err
	}
	rid, err := FromId(args.Rule)
	if err != nil {
		return nil, err
	}
	var member models.Member
	if err = p.db.Where(map[string]interface{}{"id": mid}).Take(&member).Error; err != nil {
		return nil, err
	}
	var rule models.Rule
	if err = p.db.Where(map[string]interface{}{"id": rid}).Take(&rule).Error; err != nil {
		return nil, err
	}
	if err = p.db.Model(&member).Association("Rules").Delete(&rule); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}

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

func (p *Mutation) save_firewall_rule(ctx context.Context, id *graphql.ID, rule proto.Message, sort_order int, memo string) error {
	user, ip, err := current_user(ctx, p.db, p.secrets)
	if err != nil {
		return err
	}
	type_ := reflect.TypeOf(rule).Name()
	content, err := proto.Marshal(rule)
	if err != nil {
		return err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {

		if id == nil {
			if err := tx.Create(&models.Rule{
				Content:   content,
				Type:      type_,
				SortOrder: sort_order,
				Memo:      memo,
			}).Error; err != nil {
				return err
			}
			if err := tx.Create(&models.Log{UserID: user.ID, Ip: ip, Message: fmt.Sprintf("create rule %s(%s)", type_, memo)}).Error; err != nil {
				return err
			}
		} else {
			rid, err := FromId(*id)
			if err != nil {
				return err
			}
			var it models.Rule
			if err = tx.First(&it, rid).Error; err != nil {
				return err
			}
			if type_ != it.Type {
				return fmt.Errorf("couldn't change type %s=>%s", it.Type, type_)
			}
			if err = tx.Model(&it).Updates(map[string]interface{}{
				"content":    content,
				"sort_order": sort_order,
				"memo":       memo,
				"version":    it.Version + 1,
			}).Error; err != nil {
				return err
			}
			if err := tx.Create(&models.Log{UserID: user.ID, Ip: ip, Message: fmt.Sprintf("update rule %s(%s)", type_, memo)}).Error; err != nil {
				return err
			}
		}

		return nil
	}); err != nil {
		return err
	}
	return nil
}
