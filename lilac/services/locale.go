package services

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

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/models"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type LocaleService struct {
	pb.UnimplementedLocaleServer

	jwt      *crypto.Jwt
	db       *gorm.DB
	enforcer *casbin.Enforcer
}

func NewLocaleService(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer) *LocaleService {
	return &LocaleService{db: db, jwt: jwt, enforcer: enforcer}
}

func (p *LocaleService) ByLang(ctx context.Context, req *pb.LocaleByLangRequest) (*pb.LocaleByLangResponse, error) {
	var items []*pb.LocaleByLangResponse_Item
	{
		tmp, err := models.GetLocaleByLang(p.db, req.Lang)
		if err != nil {
			return nil, err
		}
		for _, it := range tmp {
			items = append(items, &pb.LocaleByLangResponse_Item{
				Code:    it.Code,
				Message: it.Message,
			})
		}
	}
	return &pb.LocaleByLangResponse{Items: items}, nil
}
func (p *LocaleService) Index(ctx context.Context, req *pb.Pager) (*pb.LocaleIndexResponse, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := user.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}

	tmp, pag, err := models.GetLocaleByPager(p.db, req)
	if err != nil {
		return nil, err
	}
	var items []*pb.LocaleIndexResponse_Item
	for _, it := range tmp {
		items = append(items, &pb.LocaleIndexResponse_Item{
			Id:      it.ID,
			Lang:    it.Lang,
			Code:    it.Code,
			Message: it.Message,
			UpdatedAt: &timestamppb.Timestamp{
				Seconds: it.UpdatedAt.Unix(),
			},
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
	if err := user.IsAdministrator(p.enforcer); err != nil {
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
		return tx.Create(&models.Locale{
			Lang:      req.Lang,
			Code:      req.Code,
			Message:   req.Message,
			Version:   0,
			UpdatedAt: now,
			CreatedAt: now,
		}).Error
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
	if err := user.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		slog.Debug(fmt.Sprintf("delete locale %d", req.Id))
		return tx.Delete(&models.Locale{}, req.Id).Error
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
