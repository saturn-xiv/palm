package services

import (
	"context"
	"errors"
	"fmt"
	"strings"
	"time"

	"github.com/casbin/casbin/v2"
	"google.golang.org/protobuf/types/known/emptypb"
	"google.golang.org/protobuf/types/known/timestamppb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/balsam/models"
	pb "github.com/saturn-xiv/palm/atropa/balsam/services/v2"
	daisy_pb "github.com/saturn-xiv/palm/atropa/daisy/services/v2"
	"github.com/saturn-xiv/palm/atropa/env"
	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"github.com/saturn-xiv/palm/atropa/env/rabbitmq"
)

const (
	gl_reset_password_audience = "users.reset-password"
	gl_unlock_audience         = "users.unlock"
	gl_confirm_audience        = "users.confirm"
	gl_sign_in_audience        = "users.sign-in"
)

func NewEmailUserService(db *gorm.DB, rabbitmq *rabbitmq.Config, enforcer *casbin.Enforcer, hmac *crypto.HMac, jwt *crypto.Jwt) *EmailUserService {
	return &EmailUserService{db: db, hmac: hmac, jwt: jwt, enforcer: enforcer, rabbitmq: rabbitmq}
}

type EmailUserService struct {
	pb.UnimplementedEmailUserServer

	db       *gorm.DB
	hmac     *crypto.HMac
	jwt      *crypto.Jwt
	rabbitmq *rabbitmq.Config
	enforcer *casbin.Enforcer
}

type userSignUpByEmailForm struct {
	Email    string `validate:"required,email,min=5,max=127"`
	Nickname string `validate:"required,alphanum,min=2,max=63"`
	RealName string `validate:"required,min=2,max=127"`
	Password string `validate:"required,min=6,max=32"`
	Home     string `validate:"required,min=8,max=63"`
}

func (p *EmailUserService) SignUp(ctx context.Context, req *pb.UserSignUpByEmailRequest) (*emptypb.Empty, error) {
	// TODO
	// lang := Locale(ctx).String()
	// client_ip := ClientIP(ctx).String()
	form := userSignUpByEmailForm{
		Email:    strings.ToLower(strings.TrimSpace(req.Email)),
		Nickname: strings.ToLower(strings.TrimSpace(req.Nickname)),
		Password: req.Password,
	}
	if err := gl_validate.Struct(&form); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *EmailUserService) SignIn(ctx context.Context, req *pb.UserSignInByEmailRequest) (*pb.UserSignInResponse, error) {
	var it models.EmailUser

	switch user := req.User.(type) {
	case *pb.UserSignInByEmailRequest_Email:
		if err := p.db.First(&it, "email = ?", user.Email).Error; err != nil {
			return nil, err
		}
	case *pb.UserSignInByEmailRequest_Nickname:
		if err := p.db.First(&it, "nickname = ?", user.Nickname).Error; err != nil {
			return nil, err
		}
	}
	if it.DeletedAt != nil {
		return nil, errors.New("user is disabled")
	}
	if it.ConfirmedAt == nil {
		return nil, errors.New("user isn't confirmed")
	}
	return create_user_sign_in_response(
		ctx, (*models.EmailUser)(nil),
		p.db, p.jwt, p.enforcer, it.UserID,
		&pb.UserSignInResponse_Detail{
			ProviderType: pb.UserSignInResponse_Detail_Email,
			Name:         &it.RealName,
			Avatar:       &it.Avatar,
		},
		req.Ttl.AsDuration())

}
func (p *EmailUserService) ConfirmByEmail(ctx context.Context, req *pb.UserByEmailRequest) (*emptypb.Empty, error) {
	lang := Locale(ctx).String()
	user, err := p.from_request(p.db, req)
	if err != nil {
		return nil, err
	}
	if user.DeletedAt != nil {
		return nil, errors.New("user is disabled")
	}
	if user.ConfirmedAt != nil {
		return nil, errors.New("user already confirmed")
	}
	if err = p.send_email(ctx, req.Home, lang, user, gl_confirm_audience); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *EmailUserService) ConfirmByToken(ctx context.Context, req *pb.UserByTokenRequest) (*emptypb.Empty, error) {
	_, subject, _, err := p.jwt.Verify(req.Token, env.JWT_ISSUER, gl_confirm_audience)
	if err != nil {
		return nil, err
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var it models.EmailUser
		if err := tx.First(&it, "nickname = ?", subject).Error; err != nil {
			return err
		}
		if it.DeletedAt != nil {
			return errors.New("user is disabled")
		}
		if it.ConfirmedAt != nil {
			return errors.New("user is already confirmed")
		}
		if err := tx.Model(&it).Updates(map[string]interface{}{
			"confirmed_at": time.Now(),
		}).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *EmailUserService) UnlockByEmail(ctx context.Context, req *pb.UserByEmailRequest) (*emptypb.Empty, error) {
	lang := Locale(ctx).String()
	user, err := p.from_request(p.db, req)
	if err != nil {
		return nil, err
	}
	if user.DeletedAt != nil {
		return nil, errors.New("user is disabled")
	}
	{
		var it models.User
		if err = p.db.First(&it, user.UserID).Error; err != nil {
			return nil, err
		}
		if it.DeletedAt != nil {
			return nil, errors.New("user is disabled")
		}
		if it.LockedAt == nil {
			return nil, errors.New("user isn't locked")
		}
	}
	if err = p.send_email(ctx, req.Home, lang, user, gl_unlock_audience); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *EmailUserService) UnlockByToken(ctx context.Context, req *pb.UserByTokenRequest) (*emptypb.Empty, error) {
	_, subject, _, err := p.jwt.Verify(req.Token, env.JWT_ISSUER, gl_unlock_audience)
	if err != nil {
		return nil, err
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var it models.EmailUser
		if err := tx.First(&it, "nickname = ?", subject).Error; err != nil {
			return err
		}
		if it.DeletedAt != nil {
			return errors.New("user is disabled")
		}
		{
			var iu models.User
			if err = tx.First(&iu, it.UserID).Error; err != nil {
				return err
			}
			if iu.LockedAt == nil {
				return errors.New("user isn't locked")
			}
			if iu.DeletedAt != nil {
				return errors.New("user is disabled")
			}
			if err := tx.Model(&iu).Updates(map[string]interface{}{
				"locked_at": nil,
			}).Error; err != nil {
				return err
			}
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *EmailUserService) ForgotPassword(ctx context.Context, req *pb.UserByEmailRequest) (*emptypb.Empty, error) {
	lang := Locale(ctx).String()
	user, err := p.from_request(p.db, req)
	if err != nil {
		return nil, err
	}
	if user.DeletedAt != nil {
		return nil, errors.New("user is disabled")
	}
	if user.ConfirmedAt != nil {
		return nil, errors.New("user is already confirmed")
	}

	if err = p.send_email(ctx, req.Home, lang, user, gl_reset_password_audience); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *EmailUserService) ResetPassword(ctx context.Context, req *pb.UserResetPasswordRequest) (*emptypb.Empty, error) {
	_, subject, _, err := p.jwt.Verify(req.Token, env.JWT_ISSUER, gl_reset_password_audience)
	if err != nil {
		return nil, err
	}
	password, salt, err := p.hmac.Sign([]byte(req.Password))
	if err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var it models.EmailUser
		if err := tx.First(&it, "nickname = ?", subject).Error; err != nil {
			return err
		}
		if it.DeletedAt != nil {
			return errors.New("user is disabled")
		}
		if it.ConfirmedAt == nil {
			return errors.New("user isn't confirmed")
		}
		{
			var iu models.User
			if err = tx.First(&iu, it.UserID).Error; err != nil {
				return err
			}
			if iu.DeletedAt != nil {
				return errors.New("user is disabled")
			}
			if iu.LockedAt != nil {
				return errors.New("user is locked")
			}
		}

		if err := tx.Model(&it).Updates(map[string]interface{}{
			"password": password,
			"salt":     salt,
		}).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *EmailUserService) Disable(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if err := tx.Delete(&models.EmailUser{}, req.Id).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *EmailUserService) Enable(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var it models.EmailUser
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

func (p *EmailUserService) Confirm(ctx context.Context, req *pb.IdRequest) (*emptypb.Empty, error) {
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var it models.EmailUser
		if err := tx.First(&it, req.Id).Error; err != nil {
			return err
		}
		if err := tx.Model(&it).Updates(map[string]interface{}{
			"confirmed_at": nil,
		}).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *EmailUserService) Index(ctx context.Context, req *pb.Pager) (*pb.EmailUserIndexResponse, error) {
	var total int64
	if err := p.db.Model(&models.EmailUser{}).Count(&total).Error; err != nil {
		return nil, nil
	}
	pagination := pb.NewPagination(req, total)
	var items []models.EmailUser
	if err := p.db.Limit(pagination.Limit()).Offset(pagination.Offset()).Find(&items).Error; err != nil {
		return nil, err
	}
	res := pb.EmailUserIndexResponse{
		Items:      make([]*pb.EmailUserIndexResponse_Item, 0),
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
func (p *EmailUserService) ById(ctx context.Context, req *pb.IdRequest) (*pb.EmailUserIndexResponse_Item, error) {
	var it models.EmailUser
	if err := p.db.First(&it, req.Id).Error; err != nil {
		return nil, err
	}
	tmp, err := p.new_response_item(&it)
	if err != nil {
		return nil, err
	}
	return tmp, nil
}
func (p *EmailUserService) ByNickname(ctx context.Context, req *pb.EmailUserByNicknameRequest) (*pb.EmailUserIndexResponse_Item, error) {
	var it models.EmailUser
	if err := p.db.First(&it, "nickname = ?", req.Nickname).Error; err != nil {
		return nil, err
	}
	tmp, err := p.new_response_item(&it)
	if err != nil {
		return nil, err
	}
	return tmp, nil
}
func (p *EmailUserService) ByEmail(ctx context.Context, req *pb.EmailUserByEmailRequest) (*pb.EmailUserIndexResponse_Item, error) {
	var it models.EmailUser
	if err := p.db.First(&it, "email = ?", req.Email).Error; err != nil {
		return nil, err
	}
	tmp, err := p.new_response_item(&it)
	if err != nil {
		return nil, err
	}
	return tmp, nil
}

func (p *EmailUserService) from_request(db *gorm.DB, req *pb.UserByEmailRequest) (*models.EmailUser, error) {
	var it models.EmailUser

	switch user := req.User.(type) {
	case *pb.UserByEmailRequest_Email:
		if err := db.First(&it, "email = ?", user.Email).Error; err == nil {
			return &it, nil
		}
	case *pb.UserByEmailRequest_Nickname:
		if err := db.First(&it, "nickname = ?", user.Nickname).Error; err == nil {
			return &it, nil
		}

	}
	return nil, fmt.Errorf("couldn't find email user")
}
func (p *EmailUserService) new_response_item(it *models.EmailUser) (*pb.EmailUserIndexResponse_Item, error) {
	tmp := pb.EmailUserIndexResponse_Item{
		Id:        it.ID,
		RealName:  it.RealName,
		Nickname:  it.Nickname,
		Email:     it.Email,
		Avatar:    it.Avatar,
		UpdatedAt: timestamppb.New(it.UpdatedAt),
	}

	if it.ConfirmedAt != nil {
		tmp.ConfirmedAt = timestamppb.New(*it.ConfirmedAt)
	}
	if it.DeletedAt != nil {
		tmp.DeletedAt = timestamppb.New(*it.DeletedAt)
	}

	return &tmp, nil
}
func (p *EmailUserService) send_email(ctx context.Context, home string, lang string, ie *models.EmailUser, action string) error {
	now := time.Now()
	nbf := now.Add(time.Second * 3)
	exp := now.Add(time.Hour * 1)
	token, err := p.jwt.Sign(env.JWT_ISSUER, ie.Nickname, []string{action}, map[string]interface{}{}, &nbf, &exp)
	if err != nil {
		return err
	}

	task := daisy_pb.EmailTask{
		To: &daisy_pb.EmailTask_Address{
			Name:  ie.RealName,
			Email: ie.Email,
		},
		Subject: models.T(p.db, lang, fmt.Sprintf("%s.%s", action, "by-email.subject"), map[string]interface{}{"username": ie.RealName}),
		Body: &daisy_pb.EmailTask_Body{
			Text: models.T(p.db, lang, fmt.Sprintf("%s.%s", action, "by-email.body"), map[string]interface{}{"home": home, "username": ie.RealName, "token": token}),
			Html: true,
		},
	}
	return p.rabbitmq.ProducePB(ctx, &task)
}
