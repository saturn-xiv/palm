package services

import (
	"context"

	"google.golang.org/protobuf/types/known/emptypb"
	"google.golang.org/protobuf/types/known/timestamppb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/balsam/models"
	pb "github.com/saturn-xiv/palm/atropa/balsam/services/v2"
)

func NewWechatMiniProgramService(db *gorm.DB) *WechatMiniProgramService {
	return &WechatMiniProgramService{db: db}
}

type WechatMiniProgramService struct {
	pb.UnimplementedWechatMiniProgramUserServer

	db *gorm.DB
}

func (p *WechatMiniProgramService) Disable(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if err := tx.Delete(&models.WechatMiniProgramUser{}, req.Id).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *WechatMiniProgramService) Enable(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var it models.WechatMiniProgramUser
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

func (p *WechatMiniProgramService) Index(ctx context.Context, req *pb.Pager) (*pb.WechatMiniProgramUserIndexResponse, error) {
	var total int64
	if err := p.db.Model(&models.WechatMiniProgramUser{}).Count(&total).Error; err != nil {
		return nil, nil
	}
	pagination := pb.NewPagination(req, total)
	var items []models.WechatMiniProgramUser
	if err := p.db.Limit(pagination.Limit()).Offset(pagination.Offset()).Find(&items).Error; err != nil {
		return nil, err
	}
	res := pb.WechatMiniProgramUserIndexResponse{
		Items:      make([]*pb.WechatMiniProgramUserIndexResponse_Item, 0),
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

func (p *WechatMiniProgramService) ById(ctx context.Context, req *pb.IdRequest) (*pb.WechatMiniProgramUserIndexResponse_Item, error) {
	var it models.WechatMiniProgramUser
	if err := p.db.First(&it, req.Id).Error; err != nil {
		return nil, err
	}
	tmp, err := p.new_response_item(&it)
	if err != nil {
		return nil, err
	}
	return tmp, nil
}
func (p *WechatMiniProgramService) ByUnionId(ctx context.Context, req *pb.WechatByUnionIdRequest) (*pb.WechatMiniProgramUserIndexResponse_Item, error) {
	var it models.WechatMiniProgramUser
	if err := p.db.First(&it, "union_id = ?", req.UnionId).Error; err != nil {
		return nil, err
	}
	tmp, err := p.new_response_item(&it)
	if err != nil {
		return nil, err
	}
	return tmp, nil
}
func (p *WechatMiniProgramService) ByAppIdAndUnionId(ctx context.Context, req *pb.WechatByAppIdAndOpenIdRequest) (*pb.WechatMiniProgramUserIndexResponse_Item, error) {
	var it models.WechatMiniProgramUser
	if err := p.db.First(&it, "app_id = ? AND open_id = ?", req.AppId, req.OpenId).Error; err != nil {
		return nil, err
	}
	tmp, err := p.new_response_item(&it)
	if err != nil {
		return nil, err
	}
	return tmp, nil
}

func (p *WechatMiniProgramService) new_response_item(it *models.WechatMiniProgramUser) (*pb.WechatMiniProgramUserIndexResponse_Item, error) {
	tmp := pb.WechatMiniProgramUserIndexResponse_Item{
		Id:        it.ID,
		UserId:    it.UserID,
		AppId:     it.AppID,
		UnionId:   it.UnionID,
		OpenId:    it.OpenID,
		Nickname:  it.Nickname,
		AvatarUrl: it.AvatarURL,
		UpdatedAt: timestamppb.New(it.UpdatedAt),
	}
	if it.DeletedAt != nil {
		tmp.DeletedAt = timestamppb.New(*it.DeletedAt)
	}

	return &tmp, nil
}
