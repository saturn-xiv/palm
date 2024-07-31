package services

import (
	"context"

	"google.golang.org/protobuf/types/known/emptypb"
	"google.golang.org/protobuf/types/known/timestamppb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/balsam/models"
	pb "github.com/saturn-xiv/palm/atropa/balsam/services/v2"
)

func NewGoogleOauth2Service(db *gorm.DB) *GoogleOauth2Service {
	return &GoogleOauth2Service{db: db}
}

type GoogleOauth2Service struct {
	pb.UnimplementedGoogleOauth2UserServer

	db *gorm.DB
}

func (p *GoogleOauth2Service) Disable(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if err := tx.Delete(&models.GoogleOauth2User{}, req.Id).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *GoogleOauth2Service) Enable(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var it models.GoogleOauth2User
		if err := tx.First(&it, req.Id).Error; err != nil {
			return err
		}
		if err := tx.Model(&it).Updates(map[string]interface{}{
			"deleted_at": nil,
		}).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *GoogleOauth2Service) Index(ctx context.Context, req *pb.Pager) (*pb.GoogleOauth2UserIndexResponse, error) {
	var total int64
	if err := p.db.Model(&models.GoogleOauth2User{}).Count(&total).Error; err != nil {
		return nil, nil
	}
	pagination := pb.NewPagination(req, total)
	var items []models.GoogleOauth2User
	if err := p.db.Limit(pagination.Limit()).Offset(pagination.Offset()).Find(&items).Error; err != nil {
		return nil, err
	}
	res := pb.GoogleOauth2UserIndexResponse{
		Items:      make([]*pb.GoogleOauth2UserIndexResponse_Item, 0),
		Pagination: pagination,
	}
	for _, it := range items {
		tmp, err := p.new_response_item(&it)
		if err != nil {
			return nil, err
		}
		res.Items = append(res.Items, tmp)
	}
	return &res, nil
}

func (p *GoogleOauth2Service) ById(ctx context.Context, req *pb.IdRequest) (*pb.GoogleOauth2UserIndexResponse_Item, error) {
	var it models.GoogleOauth2User
	if err := p.db.First(&it, req.Id).Error; err != nil {
		return nil, err
	}
	tmp, err := p.new_response_item(&it)
	if err != nil {
		return nil, err
	}
	return tmp, nil
}
func (p *GoogleOauth2Service) BySubject(ctx context.Context, req *pb.GoogleOauth2UserBySubjectRequest) (*pb.GoogleOauth2UserIndexResponse_Item, error) {
	var it models.GoogleOauth2User
	if err := p.db.First(&it, "subject = ?", req.Subject).Error; err != nil {
		return nil, err
	}
	tmp, err := p.new_response_item(&it)
	if err != nil {
		return nil, err
	}
	return tmp, nil
}

func (p *GoogleOauth2Service) new_response_item(it *models.GoogleOauth2User) (*pb.GoogleOauth2UserIndexResponse_Item, error) {
	tmp := pb.GoogleOauth2UserIndexResponse_Item{
		Id:            it.ID,
		UserId:        it.UserID,
		Subject:       it.Subject,
		Email:         it.Email,
		EmailVerified: it.EmailVerified,
		Name:          it.Name,
		Picture:       it.Picture,
		Locale:        it.Locale,
		UpdatedAt:     timestamppb.New(it.UpdatedAt),
	}
	if it.DeletedAt != nil {
		tmp.DeletedAt = timestamppb.New(*it.DeletedAt)
	}

	return &tmp, nil
}
