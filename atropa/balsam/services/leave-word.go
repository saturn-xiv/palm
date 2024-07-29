package services

import (
	"context"
	"time"

	"google.golang.org/protobuf/types/known/emptypb"
	"google.golang.org/protobuf/types/known/timestamppb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/balsam/models"
	pb "github.com/saturn-xiv/palm/atropa/balsam/services/v2"
)

func NewLeaveWordService(db *gorm.DB) *LeaveWordService {
	return &LeaveWordService{db: db}
}

type LeaveWordService struct {
	pb.UnimplementedLeaveWordServer

	db *gorm.DB
}

func (p *LeaveWordService) Set(ctx context.Context, req *pb.LeaveWordCreateRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		return tx.Create(&models.LeaveWord{
			IP:     req.Ip,
			Lang:   req.Lang,
			Body:   req.Body,
			Editor: req.Editor.String(),
			Status: pb.LeaveWordIndexResponse_Item_Pending.String(),
		}).Error
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *LeaveWordService) Index(ctx context.Context, req *pb.Pager) (*pb.LeaveWordIndexResponse, error) {
	var total int64
	if err := p.db.Model(&models.LeaveWord{}).Count(&total).Error; err != nil {
		return nil, nil
	}
	pagination := pb.NewPagination(req, total)
	var items []models.LeaveWord
	if err := p.db.Limit(pagination.Limit()).Offset(pagination.Offset()).Find(&items).Error; err != nil {
		return nil, err
	}
	res := pb.LeaveWordIndexResponse{
		Items:      make([]*pb.LeaveWordIndexResponse_Item, 0),
		Pagination: pagination,
	}

	for _, it := range items {
		tmp := pb.LeaveWordIndexResponse_Item{
			Id:        it.ID,
			Lang:      it.Lang,
			Body:      it.Body,
			Ip:        it.IP,
			Status:    pb.LeaveWordIndexResponse_Item_Status(pb.LeaveWordIndexResponse_Item_Status_value[it.Status]),
			Editor:    pb.MediaEditor(pb.MediaEditor_value[it.Editor]),
			UpdatedAt: timestamppb.New(it.UpdatedAt),
		}
		if it.PublishedAt != nil {
			tmp.PublishedAt = timestamppb.New(*it.PublishedAt)
		}
		if it.DeletedAt != nil {
			tmp.DeletedAt = timestamppb.New(*it.DeletedAt)
		}
		res.Items = append(res.Items, &tmp)
	}
	return &res, nil
}
func (p *LeaveWordService) Publish(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var it models.LeaveWord
		if err := tx.First(&it, req.Id).Error; err != nil {
			return err
		}
		return tx.Model(&it).Updates(map[string]interface{}{
			"published_at": time.Now(),
			"version":      it.Version + 1,
		}).Error
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *LeaveWordService) Destroy(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var it models.LeaveWord
		if err := tx.First(&it, req.Id).Error; err != nil {
			return err
		}
		return tx.Model(&it).Updates(map[string]interface{}{
			"deleted_at": time.Now(),
			"version":    it.Version + 1,
		}).Error
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
