package services

import (
	"context"

	"google.golang.org/protobuf/types/known/emptypb"
	"google.golang.org/protobuf/types/known/timestamppb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/balsam/models"
	pb "github.com/saturn-xiv/palm/atropa/balsam/services/v2"
)

func NewSessionService(db *gorm.DB) *SessionService {
	return &SessionService{db: db}
}

type SessionService struct {
	pb.UnimplementedSessionServer

	db *gorm.DB
}

func (p *SessionService) Disable(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if err := tx.Delete(&models.Session{}, req.Id).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *SessionService) Index(ctx context.Context, req *pb.Pager) (*pb.SessionIndexResponse, error) {
	var total int64
	if err := p.db.Model(&models.Session{}).Count(&total).Error; err != nil {
		return nil, nil
	}
	pagination := pb.NewPagination(req, total)
	var items []models.Session
	if err := p.db.Limit(pagination.Limit()).Offset(pagination.Offset()).Find(&items).Error; err != nil {
		return nil, err
	}
	res := pb.SessionIndexResponse{
		Items:      make([]*pb.SessionIndexResponse_Item, 0),
		Pagination: pagination,
	}
	for _, it := range items {
		res.Items = append(res.Items, p.new_response_item(&it))
	}
	return &res, nil
}

func (p *SessionService) ByUser(ctx context.Context, req *pb.IdRequest) (*pb.SessionListResponse, error) {
	var items []models.Session

	if err := p.db.Where("user_id = ?", req.Id).Find(&items).Error; err != nil {
		return nil, err
	}

	var res pb.SessionListResponse
	for _, it := range items {
		res.Items = append(res.Items, p.new_response_item(&it))
	}
	return &res, nil
}

func (p *SessionService) new_response_item(it *models.Session) *pb.SessionIndexResponse_Item {
	tmp := pb.SessionIndexResponse_Item{
		Id:           it.ID,
		Uid:          it.UID,
		Ip:           it.IP,
		User:         it.UserID,
		ProviderType: pb.UserSignInResponse_Detail_ProviderType(pb.UserSignInResponse_Detail_ProviderType_value[it.ProviderType]),
		ProviderId:   it.ProviderID,
		ExpiresAt:    timestamppb.New(it.ExpiresAt),
		CreatedAt:    timestamppb.New(it.CreatedAt),
	}
	if it.DeletedAt != nil {
		tmp.DeletedAt = timestamppb.New(*it.DeletedAt)
	}
	return &tmp
}
