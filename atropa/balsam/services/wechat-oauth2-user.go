package services

import (
	"context"
	"encoding/json"

	"google.golang.org/protobuf/types/known/emptypb"
	"google.golang.org/protobuf/types/known/timestamppb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/balsam/models"
	pb "github.com/saturn-xiv/palm/atropa/balsam/services/v2"
)

func NewWechatOauth2Service(db *gorm.DB) *WechatOauth2Service {
	return &WechatOauth2Service{db: db}
}

type WechatOauth2Service struct {
	pb.UnimplementedWechatOauth2UserServer

	db *gorm.DB
}

func (p *WechatOauth2Service) Disable(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if err := tx.Delete(&models.WechatOauth2User{}, req.Id).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *WechatOauth2Service) Enable(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var it models.WechatOauth2User
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

func (p *WechatOauth2Service) Index(ctx context.Context, req *pb.Pager) (*pb.WechatOauth2UserIndexResponse, error) {
	var total int64
	if err := p.db.Model(&models.WechatOauth2User{}).Count(&total).Error; err != nil {
		return nil, nil
	}
	pagination := pb.NewPagination(req, total)
	var items []models.WechatOauth2User
	if err := p.db.Limit(pagination.Limit()).Offset(pagination.Offset()).Find(&items).Error; err != nil {
		return nil, err
	}
	res := pb.WechatOauth2UserIndexResponse{
		Items:      make([]*pb.WechatOauth2UserIndexResponse_Item, 0),
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

func (p *WechatOauth2Service) ById(ctx context.Context, req *pb.IdRequest) (*pb.WechatOauth2UserIndexResponse_Item, error) {
	var it models.WechatOauth2User
	if err := p.db.First(&it, req.Id).Error; err != nil {
		return nil, err
	}
	tmp, err := p.new_response_item(&it)
	if err != nil {
		return nil, err
	}
	return tmp, nil
}
func (p *WechatOauth2Service) ByUnionId(ctx context.Context, req *pb.WechatByUnionIdRequest) (*pb.WechatOauth2UserIndexResponse_Item, error) {
	var it models.WechatOauth2User
	if err := p.db.First(&it, "union_id = ?", req.UnionId).Error; err != nil {
		return nil, err
	}
	tmp, err := p.new_response_item(&it)
	if err != nil {
		return nil, err
	}
	return tmp, nil
}
func (p *WechatOauth2Service) ByAppIdAndUnionId(ctx context.Context, req *pb.WechatByAppIdAndOpenIdRequest) (*pb.WechatOauth2UserIndexResponse_Item, error) {
	var it models.WechatOauth2User
	if err := p.db.First(&it, "app_id = ? AND open_id = ?", req.AppId, req.OpenId).Error; err != nil {
		return nil, err
	}
	tmp, err := p.new_response_item(&it)
	if err != nil {
		return nil, err
	}
	return tmp, nil
}

func (p *WechatOauth2Service) new_response_item(it *models.WechatOauth2User) (*pb.WechatOauth2UserIndexResponse_Item, error) {
	tmp := pb.WechatOauth2UserIndexResponse_Item{
		Id:         it.ID,
		UserId:     it.UserID,
		AppId:      it.AppID,
		UnionId:    it.UnionID,
		OpenId:     it.OpenID,
		Nickname:   it.Nickname,
		Sex:        pb.WechatOauth2UserIndexResponse_Item_Sex(it.Sex),
		City:       it.City,
		Country:    it.Country,
		Province:   it.Province,
		HeadImgUrl: it.HeadImgURL,
		Lang:       pb.WechatOauth2UserIndexResponse_Item_Lang(pb.WechatOauth2UserIndexResponse_Item_Lang_value[it.Lang]),
		UpdatedAt:  timestamppb.New(it.UpdatedAt),
	}
	if it.DeletedAt != nil {
		tmp.DeletedAt = timestamppb.New(*it.DeletedAt)
	}
	{
		var items []string
		if err := json.Unmarshal(it.Privilege, &items); err != nil {
			return nil, err
		}
		tmp.Privilege = items
	}
	return &tmp, nil
}
