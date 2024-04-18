package services

import (
	"context"
	"errors"
	"fmt"
	"log/slog"
	"time"

	"github.com/casbin/casbin/v2"
	"github.com/minio/minio-go/v7"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"google.golang.org/protobuf/types/known/emptypb"
	"google.golang.org/protobuf/types/known/timestamppb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/env/rabbitmq"
	"github.com/saturn-xiv/palm/lilac/env/redis"
	"github.com/saturn-xiv/palm/lilac/i18n"
	"github.com/saturn-xiv/palm/lilac/models"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type UserService struct {
	pb.UnimplementedUserServer

	jwt      *crypto.Jwt
	mac      *crypto.HMac
	aes      *crypto.Aes
	enforcer *casbin.Enforcer
	db       *gorm.DB
	cache    *redis.Client
	queue    *rabbitmq.Config
	s3       *minio.Client
	i18n     *i18n.I18n
}

func (p *UserService) SignInByEmail(ctx context.Context, req *pb.UserSignInByEmailRequest) (*pb.UserSignInResponse, error) {
	// TODO
	return &pb.UserSignInResponse{}, nil
}
func (p *UserService) SignUpByEmail(ctx context.Context, req *pb.UserSignUpByEmailRequest) (*emptypb.Empty, error) {
	// TODO
	return &emptypb.Empty{}, nil
}

func (p *UserService) ConfirmByEmail(ctx context.Context, req *pb.UserByEmailRequest) (*emptypb.Empty, error) {
	user, eu, err := email_user_from_query(p.db, req.User)
	if err != nil {
		return nil, err
	}
	if user.DeletedAt != nil || eu.DeletedAt != nil {
		return nil, errors.New("user is disabled")
	}
	if user.LockedAt != nil {
		return nil, errors.New("user is locked")
	}
	if eu.ConfirmedAt != nil {
		return nil, errors.New("user is already verified")
	}
	if err := p.send_email(ctx, eu, req.Home, user.Lang, gl_jwt_audience_user_confirm); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *UserService) ConfirmByToken(ctx context.Context, req *pb.UserConfirmByTokenRequest) (*emptypb.Empty, error) {
	_, nickname, _, err := p.jwt.Verify(req.Token, gl_jwt_issuer, gl_jwt_audience_user_confirm)
	if err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		now := time.Now()
		var user models.User
		{
			var eu models.EmailUser
			if rst := tx.Where(&models.EmailUser{
				Nickname: nickname,
			}).First(&eu); rst.Error != nil {
				return rst.Error
			}
			if eu.DeletedAt != nil {
				return errors.New("email user is disabled")
			}
			if eu.ConfirmedAt == nil {
				return errors.New("user is already verified")
			}
			if rst := tx.First(&user, eu.UserID); rst.Error != nil {
				return rst.Error
			}
			if user.LockedAt != nil {
				return errors.New("user is locked")
			}
			if user.DeletedAt == nil {
				return errors.New("user is disabled")
			}

			eu.ConfirmedAt = &now
			eu.Version += 1
			eu.UpdatedAt = now
			if rst := tx.Save(&eu); rst.Error != nil {
				return rst.Error
			}
		}

		if rst := tx.Create(&models.Log{
			Level:     int32(pb.UserLogsResponse_Item_Warn),
			UserID:    user.ID,
			Message:   "Verify by email.",
			CreatedAt: now,
		}); rst.Error != nil {
			return rst.Error
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *UserService) UnlockByEmail(ctx context.Context, req *pb.UserByEmailRequest) (*emptypb.Empty, error) {
	user, eu, err := email_user_from_query(p.db, req.User)
	if err != nil {
		return nil, err
	}
	if user.DeletedAt != nil || eu.DeletedAt != nil {
		return nil, errors.New("user is disabled")
	}
	if user.LockedAt == nil {
		return nil, errors.New("user is unlocked")
	}
	if eu.ConfirmedAt == nil {
		return nil, errors.New("user isn't verified yet")
	}
	if err := p.send_email(ctx, eu, req.Home, user.Lang, gl_jwt_audience_user_unlock); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *UserService) UnlockByToken(ctx context.Context, req *pb.UserUnlockByTokenRequest) (*emptypb.Empty, error) {
	_, nickname, _, err := p.jwt.Verify(req.Token, gl_jwt_issuer, gl_jwt_audience_user_unlock)
	if err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		now := time.Now()
		var user models.User
		{
			var eu models.EmailUser
			if rst := tx.Where(&models.EmailUser{
				Nickname: nickname,
			}).First(&eu); rst.Error != nil {
				return rst.Error
			}
			if eu.DeletedAt != nil {
				return errors.New("email user is disabled")
			}
			if eu.ConfirmedAt != nil {
				return errors.New("user isn't verified yet")
			}
			if rst := tx.First(&user, eu.UserID); rst.Error != nil {
				return rst.Error
			}
			if user.LockedAt == nil {
				return errors.New("user isn't locked")
			}
			if user.DeletedAt == nil {
				return errors.New("user is disabled")
			}

			user.LockedAt = nil
			user.Version += 1
			user.UpdatedAt = now

			if rst := tx.Save(&user); rst.Error != nil {
				return rst.Error
			}
		}

		if rst := tx.Create(&models.Log{
			Level:     int32(pb.UserLogsResponse_Item_Warn),
			UserID:    user.ID,
			Message:   "Unlock.",
			CreatedAt: now,
		}); rst.Error != nil {
			return rst.Error
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *UserService) ForgotPassword(ctx context.Context, req *pb.UserByEmailRequest) (*emptypb.Empty, error) {
	user, eu, err := email_user_from_query(p.db, req.User)
	if err != nil {
		return nil, err
	}
	if user.DeletedAt != nil || eu.DeletedAt != nil {
		return nil, errors.New("user is disabled")
	}
	if user.LockedAt != nil {
		return nil, errors.New("user is locked")
	}
	if eu.ConfirmedAt == nil {
		return nil, errors.New("user isn't verified yet")
	}
	if err := p.send_email(ctx, eu, req.Home, user.Lang, gl_jwt_audience_user_reset_password); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *UserService) ResetPassword(ctx context.Context, req *pb.UserResetPasswordRequest) (*emptypb.Empty, error) {
	if err := gl_validate.Var(req.Password, gl_password_validator_tag); err != nil {
		return nil, err
	}
	_, nickname, _, err := p.jwt.Verify(req.Token, gl_jwt_issuer, gl_jwt_audience_user_reset_password)
	if err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		now := time.Now()
		var user models.User
		{
			var eu models.EmailUser
			if rst := tx.Where(&models.EmailUser{
				Nickname: nickname,
			}).First(&eu); rst.Error != nil {
				return rst.Error
			}
			if eu.DeletedAt != nil {
				return errors.New("user is disabled")
			}
			if eu.ConfirmedAt != nil {
				return errors.New("user isn't verified yet")
			}

			if rst := tx.First(&user, eu.UserID); rst.Error != nil {
				return rst.Error
			}
			if user.LockedAt != nil {
				return errors.New("user is locked")
			}
			if user.DeletedAt == nil {
				return errors.New("user is disabled")
			}

			password, salt, err := models.ComputePassword(p.mac, req.Password, 8)
			if err != nil {
				return err
			}
			eu.Password = password
			eu.Salt = salt
			eu.Version += 1
			eu.UpdatedAt = now
			if rst := tx.Save(eu); rst.Error != nil {
				return rst.Error
			}
		}
		if rst := tx.Create(&models.Log{
			Level:     int32(pb.UserLogsResponse_Item_Warn),
			UserID:    user.ID,
			Message:   "Reset password.",
			CreatedAt: now,
		}); rst.Error != nil {
			return rst.Error
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *UserService) Attachments(ctx context.Context, req *pb.Pager) (*pb.UserAttachmentsResponse, error) {
	su, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	var total int64
	if rst := p.db.Where(&models.Attachment{UserID: su.Payload.ID}).Count(&total); rst.Error != nil {
		return nil, rst.Error
	}
	var tmp []models.Attachment
	if rst := p.db.Where(&models.Attachment{UserID: su.Payload.ID}).Offset(req.Offset(total)).Limit(req.Size_()).Find(&tmp); rst.Error != nil {
		return nil, rst.Error
	}
	var items []*pb.UserAttachmentsResponse_Item
	for _, ia := range tmp {
		it := pb.UserAttachmentsResponse_Item{
			Id:          ia.ID,
			Bucket:      ia.Bucket,
			Name:        ia.Name,
			Size:        ia.Size,
			Title:       ia.Title,
			ContentType: ia.ContentType,
			UpdatedAt:   timestamppb.New(ia.UpdatedAt),
		}
		if ia.DeletedAt != nil {
			it.DeletedAt = timestamppb.New(*ia.DeletedAt)
		}
		items = append(items, &it)
	}

	return &pb.UserAttachmentsResponse{
		Items:      items,
		Pagination: pb.NewPagination(req, total),
	}, nil
}
func (p *UserService) Logs(ctx context.Context, req *pb.Pager) (*pb.UserLogsResponse, error) {
	su, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	var total int64
	if rst := p.db.Where(&models.Log{UserID: su.Payload.ID}).Count(&total); rst.Error != nil {
		return nil, rst.Error
	}
	var tmp []models.Log
	if rst := p.db.Where(&models.Log{UserID: su.Payload.ID}).Order("created_at DESC").Offset(req.Offset(total)).Limit(req.Size_()).Find(&tmp); rst.Error != nil {
		return nil, rst.Error
	}
	var items []*pb.UserLogsResponse_Item
	for _, it := range tmp {
		items = append(items, &pb.UserLogsResponse_Item{
			Id:     it.ID,
			Level:  pb.UserLogsResponse_Item_Level(it.Level),
			Ip:     it.Ip,
			Plugin: it.Plugin,
			Resource: &pb.PolicyPermissionsResponse_Item_Resource{
				Type: it.ResourceType,
				Id:   it.ResourceID,
			},
			Message:   it.Message,
			CreatedAt: timestamppb.New(it.CreatedAt),
		})
	}
	return &pb.UserLogsResponse{
		Items:      items,
		Pagination: pb.NewPagination(req, total),
	}, nil
}
func (p *UserService) UpdateProfile(ctx context.Context, req *pb.UserUpdateProfileRequest) (*emptypb.Empty, error) {
	su, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		now := time.Now()
		if su.ProviderType == pb.UserIndexResponse_Item_Email {
			avatar, err := IsUrl(req.Avatar)
			if err != nil {
				return err
			}
			real_name, err := IsUsername(req.RealName)
			if err != nil {
				return err
			}

			var eu models.EmailUser
			if rst := tx.First(&eu, su.ProviderId); rst.Error != nil {
				return rst.Error
			}
			eu.Avatar = avatar
			eu.RealName = real_name
			eu.Version += 1
			eu.UpdatedAt = now
			if rst := tx.Save(eu); rst.Error != nil {
				return err
			}
		}
		{
			var user models.User
			if rst := tx.First(&user, su.Payload.ID); rst.Error != nil {
				return rst.Error
			}
			user.Lang = req.Lang
			user.Timezone = req.Timezone
			user.Version += 1
			user.UpdatedAt = now
			if rst := tx.Save(&user); rst.Error != nil {
				return rst.Error
			}
		}
		if rst := tx.Create(&models.Log{
			Level:     int32(pb.UserLogsResponse_Item_Warn),
			UserID:    su.Payload.ID,
			Message:   "Update profile.",
			CreatedAt: now,
		}); rst.Error != nil {
			return rst.Error
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *UserService) ChangePassword(ctx context.Context, req *pb.UserChangePasswordRequest) (*emptypb.Empty, error) {
	su, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if su.ProviderType != pb.UserIndexResponse_Item_Email {
		return nil, errors.New("can't change password for current user")
	}
	if err = gl_validate.Var(req.NewPassword, gl_password_validator_tag); err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		now := time.Now()
		{
			var eu models.EmailUser
			if rst := tx.First(&eu, su.ProviderId); rst.Error != nil {
				return rst.Error
			}
			if err := models.VerifyPassword(p.mac, req.CurrentPassword, eu.Password, eu.Salt); err != nil {
				return err
			}
			password, salt, err := models.ComputePassword(p.mac, req.NewPassword, 8)
			if err != nil {
				return err
			}
			eu.Password = password
			eu.Salt = salt
			eu.Version += 1
			eu.UpdatedAt = now
			if rst := tx.Save(eu); rst.Error != nil {
				return rst.Error
			}
		}
		if rst := tx.Create(&models.Log{
			Level:     int32(pb.UserLogsResponse_Item_Warn),
			UserID:    su.Payload.ID,
			Message:   "Change password.",
			CreatedAt: now,
		}); rst.Error != nil {
			return rst.Error
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *UserService) SignOut(ctx context.Context, req *emptypb.Empty) (*emptypb.Empty, error) {
	su, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		now := time.Now()
		if rst := tx.Where(&models.Session{Uid: su.Session}).Delete(&models.Session{}); rst.Error != nil {
			return rst.Error
		}
		if rst := tx.Create(&models.Log{
			Level:     int32(pb.UserLogsResponse_Item_Info),
			UserID:    su.Payload.ID,
			Message:   "Sign out.",
			CreatedAt: now,
		}); rst.Error != nil {
			return rst.Error
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *UserService) Set(ctx context.Context, req *pb.KvSetRequest) (*emptypb.Empty, error) {
	su, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		return models.Set(tx, p.aes, su.Payload.ID, req.Key, req.Value, req.Encrypt)
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *UserService) Get(ctx context.Context, req *pb.KvGetRequest) (*pb.KvGetResponse, error) {
	su, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	val, err := models.Get(p.db, p.aes, su.Payload.ID, req.Key)
	if err != nil {
		return nil, err
	}
	return &pb.KvGetResponse{Value: val}, nil
}
func (p *UserService) Refresh(ctx context.Context, req *pb.UserRefreshRequest) (*pb.UserRefreshResponse, error) {
	su, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	var res pb.UserRefreshResponse
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		now := time.Now()
		token, err := models.NewSession(tx, p.jwt, gl_jwt_issuer, gl_jwt_audience_user_sign_in, su.ProviderType, su.ProviderId, req.Ttl.AsDuration())
		if err != nil {
			return err
		}
		{
			var it models.Session
			if rst := tx.Where(&models.Session{Uid: su.Session}).First(&it); rst.Error != nil {
				return rst.Error
			}
			it.ExpiredAt = now.Add(time.Second * 5)
			tx.Save(it)
		}
		tx.Create(&models.Log{
			UserID:    su.Payload.ID,
			Level:     int32(pb.UserLogsResponse_Item_Info),
			Message:   fmt.Sprintf("expire token %s", su.Session),
			CreatedAt: now,
		})
		res.Token = token
		return nil
	}); err != nil {
		return nil, err
	}
	return &res, nil
}

func (p *UserService) Create(ctx context.Context, req *pb.UserSignUpByEmailRequest) (*emptypb.Empty, error) {
	{
		user, err := NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}

	email, err := IsEmail(req.Email)
	if err != nil {
		return nil, err
	}
	nickname, err := IsCode(req.Nickname)
	if err != nil {
		return nil, err
	}
	real_name, err := IsUsername(req.RealName)
	if err != nil {
		return nil, err
	}
	if err = gl_validate.Var(req.Password, gl_password_validator_tag); err != nil {
		return nil, err
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		now := time.Now()
		if err := models.CreateUserByEmail(tx, p.mac, real_name, nickname, real_name, req.Password, req.Lang, req.Timezone); err != nil {
			return err
		}
		var eu models.EmailUser
		{
			if rst := tx.Where(&models.EmailUser{Email: email}).First(&eu); rst.Error != nil {
				return rst.Error
			}
			eu.ConfirmedAt = &now
			eu.UpdatedAt = now
			tx.Save(eu)
		}
		if rst := tx.Create(&models.Log{
			Level:     int32(pb.UserLogsResponse_Item_Info),
			UserID:    eu.UserID,
			CreatedAt: now,
		}); rst.Error != nil {
			return rst.Error
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *UserService) Index(ctx context.Context, req *pb.UserIndexRequest) (*pb.UserIndexResponse, error) {
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
	var items []*pb.UserIndexResponse_Item

	switch req.ProviderType {
	case pb.UserIndexResponse_Item_Email:
		if rst := p.db.Model(&models.EmailUser{}).Count(&total); rst.Error != nil {
			return nil, rst.Error
		}
		var tmp []models.EmailUser
		if rst := p.db.Offset(req.Pager.Offset(total)).Limit(req.Pager.Size_()).Find(&tmp); rst.Error != nil {
			return nil, rst.Error
		}
		for _, eu := range tmp {
			var it models.User
			if rst := p.db.First(&it, eu.UserID); rst.Error != nil {
				return nil, rst.Error
			}
			items = append(items, new_user_index_response_item(&it, eu.Detail()))
		}
	case pb.UserIndexResponse_Item_Google:
		if rst := p.db.Model(&models.GoogleUser{}).Count(&total); rst.Error != nil {
			return nil, rst.Error
		}
		var tmp []models.GoogleUser
		if rst := p.db.Offset(req.Pager.Offset(total)).Limit(req.Pager.Size_()).Find(&tmp); rst.Error != nil {
			return nil, rst.Error
		}
		for _, eu := range tmp {
			var it models.User
			if rst := p.db.First(&it, eu.UserID); rst.Error != nil {
				return nil, rst.Error
			}
			items = append(items, new_user_index_response_item(&it, eu.Detail()))
		}
	case pb.UserIndexResponse_Item_WeChatOauth:
		if rst := p.db.Model(&models.WechatOauth2User{}).Count(&total); rst.Error != nil {
			return nil, rst.Error
		}
		var tmp []models.WechatOauth2User
		if rst := p.db.Offset(req.Pager.Offset(total)).Limit(req.Pager.Size_()).Find(&tmp); rst.Error != nil {
			return nil, rst.Error
		}
		for _, eu := range tmp {
			var it models.User
			if rst := p.db.First(&it, eu.UserID); rst.Error != nil {
				return nil, rst.Error
			}
			items = append(items, new_user_index_response_item(&it, eu.Detail()))
		}
	case pb.UserIndexResponse_Item_WeChatMiniProgram:
		if rst := p.db.Model(&models.WechatMiniProgramUser{}).Count(&total); rst.Error != nil {
			return nil, rst.Error
		}
		var tmp []models.WechatMiniProgramUser
		if rst := p.db.Offset(req.Pager.Offset(total)).Limit(req.Pager.Size_()).Find(&tmp); rst.Error != nil {
			return nil, rst.Error
		}
		for _, eu := range tmp {
			var it models.User
			if rst := p.db.First(&it, eu.UserID); rst.Error != nil {
				return nil, rst.Error
			}
			items = append(items, new_user_index_response_item(&it, eu.Detail()))
		}
	default:
		return nil, fmt.Errorf("not support %s yet", req.ProviderType.String())
	}

	return &pb.UserIndexResponse{Items: items,
		Pagination: pb.NewPagination(req.Pager, total)}, nil
}
func (p *UserService) Sessions(ctx context.Context, req *pb.Pager) (*pb.UserSessionsResponse, error) {
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
	if rst := p.db.Model(&models.Session{}).Count(&total); rst.Error != nil {
		return nil, rst.Error
	}
	var tmp []models.Session
	if rst := p.db.Offset(req.Offset(total)).Limit(req.Size_()).Find(&tmp); rst.Error != nil {
		return nil, rst.Error
	}
	var items []*pb.UserSessionsResponse_Item
	for _, it := range tmp {
		user, detail, err := models.UserFromSession(p.db, &it)
		if err != nil {
			return nil, err
		}
		items = append(items, &pb.UserSessionsResponse_Item{
			User:      new_user_index_response_item(user, detail),
			Uid:       it.Uid,
			Ip:        it.Ip,
			ExpiredAt: timestamppb.New(it.ExpiredAt),
			CreatedAt: timestamppb.New(it.CreatedAt),
		})
	}
	return &pb.UserSessionsResponse{
		Items:      items,
		Pagination: pb.NewPagination(req, total),
	}, nil
}
func (p *UserService) SetPassword(ctx context.Context, req *pb.UserSetPasswordRequest) (*emptypb.Empty, error) {
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
		var eu models.EmailUser
		if rst := tx.First(&eu, req.User); rst.Error != nil {
			return rst.Error
		}
		var it models.User
		{
			if rst := tx.First(&it, eu.UserID); rst.Error != nil {
				return rst.Error
			}
			if it.IsRoot(p.enforcer) == nil {
				return errors.New("user has root role")
			}
		}

		slog.Debug(fmt.Sprintf("set user password %s", eu.Email))
		{
			password, salt, err := models.ComputePassword(p.mac, req.Password, 8)
			if err != nil {
				return err
			}
			eu.Password = password
			eu.Salt = salt
		}
		eu.UpdatedAt = now
		eu.Version += 1
		if rst := tx.Save(eu); rst.Error != nil {
			return rst.Error
		}
		return tx.Create(&models.Log{
			UserID:    it.ID,
			Level:     int32(pb.UserLogsResponse_Item_Warn),
			Message:   "Confirmed by administrator",
			CreatedAt: now,
		}).Error
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *UserService) Confirm(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
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
		var eu models.EmailUser
		if rst := tx.First(&eu, req.Id); rst.Error != nil {
			return rst.Error
		}
		var it models.User
		{
			if rst := tx.First(&it, eu.UserID); rst.Error != nil {
				return rst.Error
			}
			if it.IsRoot(p.enforcer) == nil {
				return errors.New("user has root role")
			}
		}
		if eu.ConfirmedAt != nil {
			return errors.New("user is already confirmed")
		}
		slog.Debug(fmt.Sprintf("confirm user %s", eu.Email))
		eu.ConfirmedAt = &now
		eu.UpdatedAt = now
		eu.Version += 1
		if rst := tx.Save(eu); rst.Error != nil {
			return rst.Error
		}
		return tx.Create(&models.Log{
			UserID:    it.ID,
			Level:     int32(pb.UserLogsResponse_Item_Warn),
			Message:   "Confirmed by administrator",
			CreatedAt: now,
		}).Error
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *UserService) Disable(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
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
		var it models.User
		if rst := tx.First(&it, req.Id); rst.Error != nil {
			return rst.Error
		}
		if it.IsRoot(p.enforcer) == nil {
			return errors.New("user has root role")
		}
		if it.DeletedAt != nil {
			return errors.New("user is already disabled")
		}
		slog.Debug(fmt.Sprintf("disable user %d", it.ID))
		it.DeletedAt = &now
		it.UpdatedAt = now
		it.Version += 1
		if rst := tx.Save(it); rst.Error != nil {
			return rst.Error
		}
		return tx.Create(&models.Log{
			UserID:    it.ID,
			Level:     int32(pb.UserLogsResponse_Item_Warn),
			Message:   "Disabled by administrator",
			CreatedAt: now,
		}).Error
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *UserService) Enable(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
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
		var it models.User
		if rst := tx.First(&it, req.Id); rst.Error != nil {
			return rst.Error
		}
		if it.IsRoot(p.enforcer) == nil {
			return errors.New("user has root role")
		}
		if it.DeletedAt == nil {
			return errors.New("user is already enabled")
		}
		slog.Debug(fmt.Sprintf("enable user %d", it.ID))
		it.DeletedAt = nil
		it.UpdatedAt = now
		it.Version += 1
		if rst := tx.Save(it); rst.Error != nil {
			return rst.Error
		}
		return tx.Create(&models.Log{
			UserID:    it.ID,
			Level:     int32(pb.UserLogsResponse_Item_Warn),
			Message:   "Enabled by administrator",
			CreatedAt: now,
		}).Error
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *UserService) Lock(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
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
		var it models.User
		if rst := tx.First(&it, req.Id); rst.Error != nil {
			return rst.Error
		}
		if it.IsRoot(p.enforcer) == nil {
			return errors.New("user has root role")
		}
		if it.LockedAt != nil {
			return errors.New("user is already locked")
		}
		slog.Debug(fmt.Sprintf("lock user %d", it.ID))
		it.LockedAt = &now
		it.UpdatedAt = now
		it.Version += 1
		if rst := tx.Save(it); rst.Error != nil {
			return rst.Error
		}
		return tx.Create(&models.Log{
			UserID:    it.ID,
			Level:     int32(pb.UserLogsResponse_Item_Warn),
			Message:   "Locked by administrator",
			CreatedAt: now,
		}).Error
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *UserService) Unlock(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
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
		var it models.User
		if rst := tx.First(&it, req.Id); rst.Error != nil {
			return rst.Error
		}
		if it.IsRoot(p.enforcer) == nil {
			return errors.New("user has root role")
		}
		if it.LockedAt == nil {
			return errors.New("user isn't locked")
		}
		slog.Debug(fmt.Sprintf("unlock user %d", it.ID))
		it.LockedAt = nil
		it.UpdatedAt = now
		it.Version += 1
		if rst := tx.Save(it); rst.Error != nil {
			return rst.Error
		}
		return tx.Create(&models.Log{
			UserID:    it.ID,
			Level:     int32(pb.UserLogsResponse_Item_Warn),
			Message:   "Unlocked by administrator",
			CreatedAt: now,
		}).Error
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func NewUserService(db *gorm.DB, cache *redis.Client, aes *crypto.Aes, mac *crypto.HMac, jwt *crypto.Jwt, enforcer *casbin.Enforcer, i18n *i18n.I18n, queue *rabbitmq.Config, s3 *minio.Client) *UserService {
	return &UserService{db: db, cache: cache, jwt: jwt, aes: aes, mac: mac, enforcer: enforcer, i18n: i18n, queue: queue, s3: s3}
}

func (p *UserService) send_email(ctx context.Context, eu *models.EmailUser, home string, lang string, action string) error {
	token, err := p.jwt.Sign(gl_jwt_issuer, eu.Nickname, action, map[string]string{}, time.Hour*1)
	if err != nil {
		return err
	}
	subject := p.i18n.Tr(lang, fmt.Sprintf("mailer.%s.subject", action), map[string]interface{}{
		"username": eu.RealName,
	})
	body := p.i18n.Tr(lang, fmt.Sprintf("mailer.%s.subject", action), map[string]interface{}{
		"username": eu.RealName,
		"token":    token,
		"home":     home,
	})
	return p.queue.Produce(ctx, pb.TaskQueueName((*pb.EmailSendRequest)(nil)), &pb.EmailSendRequest{
		To:      &pb.EmailSendRequest_Address{Name: eu.RealName, Email: eu.Email},
		Subject: subject,
		Body: &pb.EmailSendRequest_Body{
			Html:    true,
			Payload: body,
		},
	})
}

func email_user_from_query(db *gorm.DB, query *pb.EmailUserQuery) (*models.User, *models.EmailUser, error) {
	var eu models.EmailUser
	switch v := query.By.(type) {
	case *pb.EmailUserQuery_Nickname:
		if rst := db.Where(&models.EmailUser{Nickname: v.Nickname}).First(&eu); rst.Error != nil {
			return nil, nil, rst.Error
		}
	case *pb.EmailUserQuery_Email:
		if rst := db.Where(&models.EmailUser{Email: v.Email}).First(&eu); rst.Error != nil {
			return nil, nil, rst.Error
		}
	default:
		return nil, nil, status.Error(codes.NotFound, "")
	}

	var it models.User
	if rst := db.First(&it, eu.UserID); rst.Error != nil {
		return nil, nil, rst.Error
	}
	return &it, &eu, nil
}

func new_user_index_response_item(user *models.User, detail *pb.UserIndexResponse_Item_Detail) *pb.UserIndexResponse_Item {
	it := pb.UserIndexResponse_Item{
		Detail:      detail,
		Lang:        user.Lang,
		Timezone:    user.Timezone,
		SignInCount: user.SignInCount,
	}
	if user.LastSignedInAt != nil {
		it.LastSignedInAt = timestamppb.New(*user.LastSignedInAt)
	}
	if user.LastSignedInIp != nil {
		it.LastSignedInIp = *user.LastSignedInIp
	}
	if user.CurrentSignedInAt != nil {
		it.CurrentSignedInAt = timestamppb.New(*user.CurrentSignedInAt)
	}
	if user.CurrentSignedInIp != nil {
		it.CurrentSignedInIp = *user.CurrentSignedInIp
	}
	return &it
}
