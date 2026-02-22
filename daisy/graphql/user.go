package graphql

import (
	"context"
	"errors"
	"fmt"
	"time"

	"github.com/graph-gophers/graphql-go"
	"golang.org/x/text/language"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/crypto"
	"github.com/saturn-xiv/palm/daisy/models"
	portal_v2 "github.com/saturn-xiv/palm/daisy/portal/v2"
	"github.com/saturn-xiv/palm/daisy/rbac"
)

func (p *Mutation) DestroyUser(ctx context.Context, args struct {
	Id     graphql.ID
	Reason string
}) (*Ok, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err = ss.User.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}

	id, err := FromId(args.Id)
	if err != nil {
		return nil, err
	}

	{
		var it models.User
		if err := p.db.First(&it, id).Error; err != nil {
			return nil, err
		}
		if err := it.IsRoot(p.enforcer); err == nil {
			return nil, status.Error(codes.PermissionDenied, "it is a root user")
		}
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if err := tx.Delete(&models.User{}, id).Error; err != nil {
			return err
		}
		return models.CreateLog(tx, id, portal_v2.Plugin(), ss.ClientIp, portal_v2.UserIndexLogResponse_Item_WARNING, fmt.Sprintf("deleted by %s, reason: %s", ss.Name, args.Reason))

	}); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}

func (p *Mutation) UnlockUser(ctx context.Context, args struct {
	Id     graphql.ID
	Reason string
}) (*Ok, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err = ss.User.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}

	id, err := FromId(args.Id)
	if err != nil {
		return nil, err
	}

	{
		var it models.User
		if err := p.db.First(&it, id).Error; err != nil {
			return nil, err
		}
		if err := it.IsRoot(p.enforcer); err == nil {
			return nil, status.Error(codes.PermissionDenied, "it is a root user")
		}
		if it.LockedAt == nil {
			return nil, status.Error(codes.InvalidArgument, "user isn't locked")
		}
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if err := tx.Model(&models.User{}).Where("id = ?", id).Updates(map[string]interface{}{"locked_at": nil}).Error; err != nil {
			return err
		}
		return models.CreateLog(tx, id, portal_v2.Plugin(), ss.ClientIp, portal_v2.UserIndexLogResponse_Item_WARNING, fmt.Sprintf("unlock by %s, reason: %s", ss.Name, args.Reason))
	}); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}
func (p *Mutation) LockUser(ctx context.Context, args struct {
	Id     graphql.ID
	Reason string
}) (*Ok, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err = ss.User.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	id, err := FromId(args.Id)
	if err != nil {
		return nil, err
	}
	{
		var it models.User
		if err := p.db.First(&it, id).Error; err != nil {
			return nil, err
		}
		if err := it.IsRoot(p.enforcer); err == nil {
			return nil, status.Error(codes.PermissionDenied, "it is a root user")
		}
		if it.LockedAt != nil {
			return nil, status.Error(codes.InvalidArgument, "user is already locked")
		}
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if err := tx.Model(&models.User{}).Updates(map[string]interface{}{"locked_at": time.Now()}).Error; err != nil {
			return err
		}
		return models.CreateLog(tx, id, portal_v2.Plugin(), ss.ClientIp, portal_v2.UserIndexLogResponse_Item_WARNING, fmt.Sprintf("lock by %s, reason: %s", ss.Name, args.Reason))
	}); err != nil {
		return nil, err
	}

	return &Ok{}, nil
}

func (p *Mutation) SetUserLocation(ctx context.Context, args struct {
	Lang     string
	Timezone string
}) (*Ok, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	lang, err := language.Parse(args.Lang)
	if err != nil {
		return nil, err
	}
	tz, err := time.LoadLocation(args.Timezone)
	if err != nil {
		return nil, err
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if err := tx.Model(&models.User{}).Where("id = ?", ss.User.Id).Updates(map[string]interface{}{"lang": lang.String(), "timezone": tz.String()}).Error; err != nil {
			return err
		}
		return models.CreateLog(tx, uint(ss.User.Id), portal_v2.Plugin(), ss.ClientIp, portal_v2.UserIndexLogResponse_Item_INFO, "update location")
	}); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}

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

type UserSignInResponse struct{}

func newUserSignInResponse(db *gorm.DB, provider_type portal_v2.Session_ProviderType, provider_sn string, ttl uint) (*UserSignInResponse, error) {
	if ttl < 60 {
		return nil, fmt.Errorf("ttl shouldn't least than %d seconds", ttl)
	}
	// TODO
	return &UserSignInResponse{}, nil
}

func (p *UserSignInResponse) Token() (string, error) {
	// TODO
	return "", errors.New("todo")
}

func CurrentUser(ctx context.Context, db *gorm.DB, jwt *crypto.Jwt) (*portal_v2.Session, error) {
	auth, ok := ctx.Value(headerKey(rbac.Authorization)).(string)
	if !ok {
		return nil, errors.New("no authorization header")
	}
	return rbac.NewSessionByAuthorization(auth, db, jwt)
}
