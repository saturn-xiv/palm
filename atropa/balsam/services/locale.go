package services

import (
	"context"

	"google.golang.org/protobuf/types/known/emptypb"
	"google.golang.org/protobuf/types/known/timestamppb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/balsam/models"
	pb "github.com/saturn-xiv/palm/atropa/balsam/services/v2"
)

func NewLocaleService(db *gorm.DB) *LocaleService {
	return &LocaleService{db: db}
}

type LocaleService struct {
	pb.UnimplementedLocaleServer

	db *gorm.DB
}

func (p *LocaleService) Set(ctx context.Context, req *pb.LocaleSetRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		return models.SetLocale(tx, req.Lang, req.Code, req.Message)
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *LocaleService) Index(ctx context.Context, req *pb.Pager) (*pb.LocaleIndexResponse, error) {
	var total int64
	if err := p.db.Model(&models.Locale{}).Count(&total).Error; err != nil {
		return nil, nil
	}
	pagination := pb.NewPagination(req, total)
	var items []models.Locale
	if err := p.db.Limit(pagination.Limit()).Offset(pagination.Offset()).Find(&items).Error; err != nil {
		return nil, err
	}
	res := pb.LocaleIndexResponse{
		Items:      make([]*pb.LocaleIndexResponse_Item, 0),
		Pagination: pagination,
	}

	for _, it := range items {
		res.Items = append(res.Items, &pb.LocaleIndexResponse_Item{
			Id:        it.ID,
			Code:      it.Code,
			Lang:      it.Lang,
			Message:   it.Message,
			UpdatedAt: timestamppb.New(it.UpdatedAt),
		})
	}

	return &res, nil
}
func (p *LocaleService) ByLang(ctx context.Context, req *pb.LocaleByLangRequest) (*pb.LocaleByLangResponse, error) {
	var items []models.Locale
	if err := p.db.Where("lang = ?", req.Lang).Find(&items).Error; err != nil {
		return nil, err
	}
	res := pb.LocaleByLangResponse{
		Items: make([]*pb.LocaleIndexResponse_Item, 0),
	}

	for _, it := range items {
		updated_at := timestamppb.New(it.UpdatedAt)
		res.Items = append(res.Items, &pb.LocaleIndexResponse_Item{
			Id:        it.ID,
			Code:      it.Code,
			Lang:      it.Lang,
			Message:   it.Message,
			UpdatedAt: updated_at,
		})
	}

	return &res, nil
}
