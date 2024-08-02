package services

import (
	"context"
	"errors"
	"log/slog"
	"time"

	"github.com/casbin/casbin/v2"
	"google.golang.org/protobuf/types/known/emptypb"
	"google.golang.org/protobuf/types/known/timestamppb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/balsam/models"
	pb "github.com/saturn-xiv/palm/atropa/balsam/services/v2"
	"github.com/saturn-xiv/palm/atropa/env"
	"github.com/saturn-xiv/palm/atropa/env/crypto"
	rbac_pb "github.com/saturn-xiv/palm/atropa/rbac/services/v2"
)

func NewUserService(db *gorm.DB) *UserService {
	return &UserService{db: db}
}

type UserService struct {
	pb.UnimplementedUserServer

	db *gorm.DB
}

func (p *UserService) Disable(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if err := tx.Delete(&models.User{}, req.Id).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *UserService) Enable(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var it models.User
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
func (p *UserService) Lock(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var it models.User
		if err := tx.First(&it, req.Id).Error; err != nil {
			return err
		}
		if err := tx.Model(&it).Updates(map[string]interface{}{
			"locked_at": time.Now(),
		}).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *UserService) Unlock(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var it models.User
		if err := tx.First(&it, req.Id).Error; err != nil {
			return err
		}
		if err := tx.Model(&it).Updates(map[string]interface{}{
			"locked_at": nil,
		}).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *UserService) Logs(ctx context.Context, req *pb.UserLogsRequest) (*pb.UserLogsResponse, error) {
	var total int64
	if req.User == nil {
		if err := p.db.Model(&models.Log{}).Count(&total).Error; err != nil {
			return nil, nil
		}
	} else {
		if err := p.db.Model(&models.Log{}).Where("user_id = ?", *req.User).Count(&total).Error; err != nil {
			return nil, nil
		}
	}
	pagination := pb.NewPagination(req.Pager, total)
	var items []models.Log
	if req.User == nil {
		if err := p.db.Limit(pagination.Limit()).Offset(pagination.Offset()).Order("created_at DESC").Find(&items).Error; err != nil {
			return nil, err
		}
	} else {
		if err := p.db.Where("user_id = ?", *req.User).Limit(pagination.Limit()).Offset(pagination.Offset()).Order("created_at DESC").Find(&items).Error; err != nil {
			return nil, err
		}
	}
	res := pb.UserLogsResponse{
		Items:      make([]*pb.UserLogsResponse_Item, 0),
		Pagination: pagination,
	}
	for _, it := range items {
		tmp, err := p.new_log_response_item(&it)
		if err != nil {
			return nil, err
		}
		res.Items = append(res.Items, tmp)
	}
	return &res, nil
}

func (p *UserService) SignOut(ctx context.Context, req *pb.UserSignOutRequest) (*emptypb.Empty, error) {
	ip := ClientIP(ctx).String()
	lang := Locale(ctx).String()
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var it models.User
		if err := tx.First(&it, req.Id).Error; err != nil {
			return err
		}
		if err := models.CreateLog(
			tx, req.Id, lang, ip, pb.UserLogsResponse_Item_Info,
			(*UserService)(nil), (*models.User)(nil), nil,
			"user.logs.sign-out", map[string]interface{}{},
		); err != nil {
			return err
		}
		if err := tx.Model(&it).Updates(map[string]interface{}{
			"current_sign_at": nil,
			"current_sign_ip": nil,
			"last_sign_at":    it.LastSignInAt,
			"last_sign_ip":    it.LastSignInIP,
		}).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *UserService) UpdateProfile(ctx context.Context, req *pb.UserUpdateProfileRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var it models.User
		if err := tx.First(&it, req.Id).Error; err != nil {
			return err
		}
		if err := tx.Model(&it).Updates(map[string]interface{}{
			"lang":     req.Lang,
			"timezone": req.Timezone,
		}).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *UserService) Index(ctx context.Context, req *pb.Pager) (*pb.UserIndexResponse, error) {
	var total int64
	if err := p.db.Model(&models.User{}).Count(&total).Error; err != nil {
		return nil, nil
	}
	pagination := pb.NewPagination(req, total)
	var items []models.User
	if err := p.db.Limit(pagination.Limit()).Offset(pagination.Offset()).Find(&items).Error; err != nil {
		return nil, err
	}
	res := pb.UserIndexResponse{
		Items:      make([]*pb.UserIndexResponse_Item, 0),
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
func (p *UserService) ById(ctx context.Context, req *pb.IdRequest) (*pb.UserIndexResponse_Item, error) {
	var it models.User
	if err := p.db.First(&it, req.Id).Error; err != nil {
		return nil, err
	}
	tmp, err := p.new_response_item(&it)
	if err != nil {
		return nil, err
	}
	return tmp, nil
}
func (p *UserService) ByUid(ctx context.Context, req *pb.UidRequest) (*pb.UserIndexResponse_Item, error) {
	var it models.User
	if err := p.db.First(&it, "uid = ?", req.Uid).Error; err != nil {
		return nil, err
	}
	tmp, err := p.new_response_item(&it)
	if err != nil {
		return nil, err
	}
	return tmp, nil
}

func (p *UserService) new_response_item(it *models.User) (*pb.UserIndexResponse_Item, error) {
	tmp := pb.UserIndexResponse_Item{
		Id:              it.ID,
		Uid:             it.UID,
		Lang:            it.Lang,
		Timezone:        it.Timezone,
		SignInCount:     it.SignInCount,
		CurrentSignInIp: it.CurrentSignInIP,
		LastSignInIp:    it.LastSignInIP,
		UpdatedAt:       timestamppb.New(it.UpdatedAt),
	}
	if it.CurrentSignInAt != nil {
		tmp.CurrentSignInAt = timestamppb.New(*it.CurrentSignInAt)
	}
	if it.LastSignInAt != nil {
		tmp.LastSignInAt = timestamppb.New(*it.LastSignInAt)
	}
	if it.LockedAt != nil {
		tmp.LockedaAt = timestamppb.New(*it.LockedAt)
	}
	if it.DeletedAt != nil {
		tmp.DeletedAt = timestamppb.New(*it.DeletedAt)
	}

	return &tmp, nil
}

func (p *UserService) new_log_response_item(it *models.Log) (*pb.UserLogsResponse_Item, error) {
	tmp := pb.UserLogsResponse_Item{
		Id:           it.ID,
		Plugin:       it.Plugin,
		Ip:           it.IP,
		Message:      it.Message,
		Level:        pb.UserLogsResponse_Item_Level(pb.UserLogsResponse_Item_Level_value[it.Level]),
		ResourceType: it.ResourceType,
		ResourceId:   it.ResourceID,
		CreatedAt:    timestamppb.New(it.CreatedAt),
	}

	return &tmp, nil
}

func create_user_sign_in_response(ctx context.Context, resource any, db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer, user_id uint64, detail *pb.UserSignInResponse_Detail, ttl time.Duration) (*pb.UserSignInResponse, error) {
	lang := Locale(ctx).String()
	client_ip := ClientIP(ctx).String()
	var user models.User
	if err := db.First(&user, user_id).Error; err != nil {
		return nil, err
	}
	if user.DeletedAt != nil {
		return nil, errors.New("user is disabled")
	}
	if user.LockedAt != nil {
		return nil, errors.New("user is locked")
	}
	now := time.Now()
	exp := now.Add(ttl)
	token, err := jwt.Sign(env.JWT_ISSUER, user.UID, []string{gl_sign_in_audience}, map[string]interface{}{}, &now, &exp)
	if err != nil {
		return nil, err
	}
	if err := db.Transaction(func(tx *gorm.DB) error {
		if err := models.CreateLog(tx, user_id, lang, client_ip, pb.UserLogsResponse_Item_Info, (*UserService)(nil), resource, nil, "user.logs.sign-in", map[string]interface{}{}); err != nil {
			return err
		}
		if err := tx.Model(&models.User{}).Where("id = ?", user_id).Updates(map[string]interface{}{
			"current_sign_at": time.Now(),
			"current_sign_ip": client_ip,
			"last_sign_at":    user.CurrentSignInAt,
			"last_sign_ip":    user.CurrentSignInIP,
			"sign_in_count":   user.SignInCount + 1,
		}).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	{
		// TODO
		detail.HasPhone = false
	}
	{
		// TODO
		detail.HasFacebookOauth2 = false
	}
	{
		var c int64
		if err := db.Model(&models.EmailUser{}).Count(&c).Error; err != nil {
			return nil, err
		}
		detail.HasEmail = c > 0
	}
	{
		var c int64
		if err := db.Model(&models.GoogleOauth2User{}).Count(&c).Error; err != nil {
			return nil, err
		}
		detail.HasGoogleOauth2 = c > 0
	}
	{
		var c int64
		if err := db.Model(&models.WechatMiniProgramUser{}).Count(&c).Error; err != nil {
			return nil, err
		}
		detail.HasWechatMiniProgram = c > 0
	}
	{
		var c int64
		if err := db.Model(&models.WechatOauth2User{}).Count(&c).Error; err != nil {
			return nil, err
		}
		detail.HasWechatOauth2 = c > 0
	}
	res := pb.UserSignInResponse{
		Detail:      detail,
		Token:       token,
		Roles:       make([]string, 0),
		Permissions: make([]*pb.UserSignInResponse_Permission, 0),
		Menus:       make([]*pb.UserSignInResponse_Menu, 0),
	}

	{
		pu := rbac_pb.PolicyUsersResponse_Item{
			Id: &rbac_pb.PolicyUsersResponse_Item_I{I: user.ID},
		}
		subject, err := pu.Code()
		if err != nil {
			return nil, err
		}
		{
			roles, err := enforcer.GetImplicitRolesForUser(subject)
			if err != nil {
				return nil, err
			}
			for _, it := range roles {
				role, err := rbac_pb.NewPolicyRoleFromCode(it)
				if err != nil {
					return nil, err
				}
				switch by := role.By.(type) {
				case *rbac_pb.PolicyRolesResponse_Item_Administrator_:
					res.IsAdministrator = true
				case *rbac_pb.PolicyRolesResponse_Item_Root_:
					res.IsRoot = true
				case *rbac_pb.PolicyRolesResponse_Item_Code:
					res.Roles = append(res.Roles, by.Code)
				}
			}
		}
		{
			permissions, err := enforcer.GetImplicitPermissionsForUser(subject)
			if err != nil {
				return nil, err
			}
			for _, rule := range permissions {
				permission, err := rbac_pb.NewPolicyPermissionFromRule(rule)
				if err != nil {
					return nil, err
				}
				it := pb.UserSignInResponse_Permission{
					Operation:    permission.Operation.String(),
					ResourceType: permission.Resource.Type,
				}
				if permission.Resource.Id.By != nil {
					switch by := permission.Resource.Id.By.(type) {
					case *rbac_pb.PolicyPermissionsResponse_Item_Resource_Id_I:
						it.ResourceId = &by.I
					case *rbac_pb.PolicyPermissionsResponse_Item_Resource_Id_S:
						slog.Warn("unsupported resource id", slog.String("s", by.S))
						continue
					}
				}
				res.Permissions = append(res.Permissions, &it)
			}
		}
	}
	{
		// TODO menus
	}

	return &res, nil
}
