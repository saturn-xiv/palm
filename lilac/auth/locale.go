package auth

import (
	"context"
	"errors"
	"fmt"
	"log/slog"
	"time"

	"github.com/casbin/casbin/v2"
	"google.golang.org/protobuf/types/known/emptypb"
	"google.golang.org/protobuf/types/known/timestamppb"
	"gorm.io/gorm"

	pb "github.com/saturn-xiv/palm/lilac/auth/v2"
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/i18n"
	"github.com/saturn-xiv/palm/lilac/models"
)

type LocaleService struct {
	pb.UnimplementedLocaleServer

	jwt      *crypto.Jwt
	db       *gorm.DB
	enforcer *casbin.Enforcer
	i18n     *i18n.I18n
}

func NewLocaleService(db *gorm.DB, jwt *crypto.Jwt, i18n *i18n.I18n, enforcer *casbin.Enforcer) *LocaleService {
	return &LocaleService{db: db, jwt: jwt, i18n: i18n, enforcer: enforcer}
}

func (p *LocaleService) ByLang(ctx context.Context, req *pb.LocaleByLangRequest) (*pb.LocaleByLangResponse, error) {
	var items []*pb.LocaleByLangResponse_Item

	tmp, ok := p.i18n.ByLang(req.Lang)
	if !ok {
		return nil, fmt.Errorf("couldn't find language %s", req.Lang)
	}
	for k, v := range tmp {
		items = append(items, &pb.LocaleByLangResponse_Item{
			Code:    k,
			Message: v,
		})
	}

	return &pb.LocaleByLangResponse{Items: items}, nil
}
func (p *LocaleService) Index(ctx context.Context, req *pb.Pager) (*pb.LocaleIndexResponse, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}

	tmp, pag, err := models.GetLocaleByPager(p.db, req)
	if err != nil {
		return nil, err
	}
	var items []*pb.LocaleIndexResponse_Item
	for _, it := range tmp {
		items = append(items, &pb.LocaleIndexResponse_Item{
			Id:        it.ID,
			Lang:      it.Lang,
			Code:      it.Code,
			Message:   it.Message,
			UpdatedAt: timestamppb.New(it.UpdatedAt),
		})
	}

	return &pb.LocaleIndexResponse{
		Items:      items,
		Pagination: pag,
	}, nil
}

func (p *LocaleService) Set(ctx context.Context, req *pb.LocaleSetRequest) (*emptypb.Empty, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		now := time.Now()

		it, err := models.GetLocaleByLangAndCode(tx, req.Lang, req.Code)
		if err == nil {
			slog.Debug(fmt.Sprintf("update locale %s.%s", req.Lang, req.Code))
			it.Message = req.Message
			it.UpdatedAt = now
			it.Version += 1
			return tx.Save(it).Error
		}
		if !errors.Is(err, gorm.ErrRecordNotFound) {
			return err
		}
		slog.Debug(fmt.Sprintf("create locale %s.%s", req.Lang, req.Code))
		if rst := tx.Create(&models.Locale{
			Lang:      req.Lang,
			Code:      req.Code,
			Message:   req.Message,
			Version:   0,
			UpdatedAt: now,
			CreatedAt: now,
		}); rst.Error != nil {
			return rst.Error
		}
		if err = p.i18n.Reload(tx); err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *LocaleService) Destroy(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		slog.Debug(fmt.Sprintf("delete locale %d", req.Id))
		if rst := tx.Delete(&models.Locale{}, req.Id); rst.Error != nil {
			return rst.Error
		}
		if err := p.i18n.Reload(tx); err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
