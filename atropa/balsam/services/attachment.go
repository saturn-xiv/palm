package services

import (
	"context"
	"log/slog"
	"time"

	"google.golang.org/protobuf/types/known/emptypb"
	"google.golang.org/protobuf/types/known/timestamppb"
	"gorm.io/gorm"

	pb "github.com/saturn-xiv/palm/atropa/balsam/services/v2"
	"github.com/saturn-xiv/palm/atropa/s3/models"
)

func NewAttachmentService(db *gorm.DB) *AttachmentService {
	return &AttachmentService{db: db}
}

type AttachmentService struct {
	pb.UnimplementedAttachmentServer

	db *gorm.DB
}

func (p *AttachmentService) Disable(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if err := tx.Delete(&models.Attachment{}, req.Id).Error; err != nil {
			return err
		}
		if err := tx.Where("attachment_id = ?", req.Id).Delete(&models.AttachmentResource{}).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *AttachmentService) SetTitle(ctx context.Context, req *pb.AttachmentSetTitleRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var it models.Attachment
		if err := tx.First(&it, req.Id).Error; err != nil {
			return err
		}
		if err := tx.Model(&it).Updates(map[string]interface{}{
			"title": req.Title,
		}).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *AttachmentService) ByUser(ctx context.Context, req *pb.IdRequest) (*pb.AttachmentListResponse, error) {
	var items []models.Attachment

	if err := p.db.Where("user_id = ?", req.Id).Find(&items).Error; err != nil {
		return nil, err
	}

	var res pb.AttachmentListResponse
	for _, it := range items {
		res.Items = append(res.Items, p.new_response_item(&it))
	}
	return &res, nil
}

func (p *AttachmentService) Clear(ctx context.Context, req *pb.AttachmentClearRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		ts := time.Now().AddDate(0, 0, int(0-req.DaysToKeep))
		slog.Warn("will clean the invalid attachment", slog.Time("before", ts))
		if err := tx.Where("uploaded_at IS NULL AND created_at < ?", ts).Delete(&models.Attachment{}).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *AttachmentService) Index(ctx context.Context, req *pb.Pager) (*pb.AttachmentIndexResponse, error) {
	var total int64
	if err := p.db.Model(&models.Attachment{}).Count(&total).Error; err != nil {
		return nil, nil
	}
	pagination := pb.NewPagination(req, total)
	var items []models.Attachment
	if err := p.db.Limit(pagination.Limit()).Offset(pagination.Offset()).Find(&items).Error; err != nil {
		return nil, err
	}
	res := pb.AttachmentIndexResponse{
		Items:      make([]*pb.AttachmentIndexResponse_Item, 0),
		Pagination: pagination,
	}
	for _, it := range items {
		res.Items = append(res.Items, p.new_response_item(&it))
	}
	return &res, nil
}
func (p *AttachmentService) ByResource(ctx context.Context, req *pb.ResourceRequest) (*pb.AttachmentListResponse, error) {
	var items []models.Attachment
	if req.Id == nil {
		if err := p.db.Where("resource_type = ? AND resource_id IS NULL", req.Type).Find(&items).Error; err != nil {
			return nil, err
		}
	} else {
		if err := p.db.Where("resource_type = ? AND resource_id = ?", req.Type, *req.Id).Find(&items).Error; err != nil {
			return nil, err
		}
	}
	var res pb.AttachmentListResponse
	for _, it := range items {
		res.Items = append(res.Items, p.new_response_item(&it))
	}
	return &res, nil
}
func (p *AttachmentService) Associate(ctx context.Context, req *pb.AttachmentResourceRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if err := tx.Create(&models.AttachmentResource{
			ResourceType: req.ResourceType,
			ResourceID:   req.ResourceId,
		}).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *AttachmentService) Dissociate(ctx context.Context, req *pb.AttachmentResourceRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if req.ResourceId == nil {
			if err := tx.Where("attachment_id = ? AND resource_type = ? AND resource_id IS NULL", req.Id, req.ResourceType).Delete(&models.AttachmentResource{}).Error; err != nil {
				return err
			}
		} else {
			if err := tx.Where("attachment_id = ? AND resource_type = ? AND resource_id = ?", req.Id, req.ResourceType, *req.ResourceId).Delete(&models.AttachmentResource{}).Error; err != nil {
				return err
			}
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *AttachmentService) Create(ctx context.Context, req *pb.AttachmentCreateRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if err := tx.Create(&models.Attachment{
			UserID:      req.User,
			Title:       req.Title,
			ContentType: req.ContentType,
			Bucket:      req.Bucket,
			Object:      req.Object,
			Size:        req.Size,
		}).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *AttachmentService) SetUploadedAt(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var it models.Attachment
		if err := tx.First(&it, req.Id).Error; err != nil {
			return err
		}
		if err := tx.Model(&it).Updates(map[string]interface{}{
			"uploaded_at": time.Now(),
		}).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *AttachmentService) new_response_item(it *models.Attachment) *pb.AttachmentIndexResponse_Item {
	tmp := pb.AttachmentIndexResponse_Item{
		Id:          it.ID,
		User:        it.UserID,
		Size:        it.Size,
		Bucket:      it.Bucket,
		Object:      it.Object,
		Title:       it.Title,
		ContentType: it.ContentType,
		UpdatedAt:   timestamppb.New(it.UpdatedAt),
	}
	if it.UploadedAt != nil {
		tmp.UploadedAt = timestamppb.New(*it.UploadedAt)
	}
	return &tmp
}
