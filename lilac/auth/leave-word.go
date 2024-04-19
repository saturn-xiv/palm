package auth

import (
	"context"
	"time"

	"github.com/casbin/casbin/v2"
	"google.golang.org/protobuf/types/known/emptypb"
	"google.golang.org/protobuf/types/known/timestamppb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/models"
	pb "github.com/saturn-xiv/palm/lilac/auth/v2"
)

type LeaveWordService struct {
	pb.UnimplementedLeaveWordServer

	jwt      *crypto.Jwt
	db       *gorm.DB
	enforcer *casbin.Enforcer
}

func (p *LeaveWordService) Create(ctx context.Context, req *pb.LeaveWordCreateRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		now := time.Now()
		tx.Create(&models.LeaveWord{
			Lang:    req.Lang,
			Ip:      req.Ip,
			Content: req.Content,
			Editor:  int32(req.Editor),
			Model: models.Model{
				CreatedAt: now,
				UpdatedAt: now,
			},
		})
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *LeaveWordService) Index(ctx context.Context, req *pb.Pager) (*pb.LeaveWordIndexResponse, error) {
	{
		user, err := NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	var total int64
	if rst := p.db.Model(&models.LeaveWord{}).Count(&total); rst.Error != nil {
		return nil, rst.Error
	}
	var tmp []models.LeaveWord
	if rst := p.db.Order("created_at desc").Find(&tmp); rst.Error != nil {
		return nil, rst.Error
	}
	var items []*pb.LeaveWordIndexResponse_Item
	for _, lw := range tmp {
		it := pb.LeaveWordIndexResponse_Item{
			Id:        lw.ID,
			Ip:        lw.Ip,
			Lang:      lw.Lang,
			Content:   lw.Content,
			Editor:    pb.Editor(lw.Editor),
			CreatedAt: timestamppb.New(lw.CreatedAt),
		}
		if lw.DeletedAt != nil {
			it.DeletedAt = timestamppb.New(*lw.DeletedAt)
		}
		if lw.PublishedAt != nil {
			it.PublishedAt = timestamppb.New(*lw.PublishedAt)
		}
		items = append(items, &it)
	}
	return &pb.LeaveWordIndexResponse{
		Items:      items,
		Pagination: pb.NewPagination(req, total),
	}, nil
}

func (p *LeaveWordService) Delete(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
	{
		user, err := NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		now := time.Now()
		var it models.LeaveWord
		if rst := tx.First(&it, req.Id); rst.Error != nil {
			return rst.Error
		}
		it.DeletedAt = &now
		it.Version += 1
		it.UpdatedAt = now
		if rst := tx.Save(it); rst.Error != nil {
			return rst.Error
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func NewLeaveWordService(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer) *LeaveWordService {
	return &LeaveWordService{db: db, jwt: jwt, enforcer: enforcer}
}
