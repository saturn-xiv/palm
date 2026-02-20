package portal

import (
	"context"
	"errors"
	"strings"

	"github.com/casbin/casbin/v3"
	"github.com/minio/minio-go/v7"
	"google.golang.org/protobuf/types/known/emptypb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/crypto"
	"github.com/saturn-xiv/palm/daisy/models"
	v2 "github.com/saturn-xiv/palm/daisy/portal/v2"
	"github.com/saturn-xiv/palm/daisy/rbac"
)

type EmailUserServer struct {
	db       *gorm.DB
	enforcer *casbin.Enforcer
	jwt      *crypto.Jwt
	hmac     *crypto.Hmac
	s3       *minio.Client
}

func NewEmailUserServer(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer, s3 *minio.Client, hmac *crypto.Hmac) *EmailUserServer {
	return &EmailUserServer{db: db, s3: s3, enforcer: enforcer, jwt: jwt, hmac: hmac}
}

type setEmailUserNameForm struct {
	Name string `validate:"required,min=2,max=31"`
}

func (p *EmailUserServer) SetName(ctx context.Context, req *v2.EmailUserSetNameRequest) (*emptypb.Empty, error) {
	form := setEmailUserNameForm{Name: strings.TrimSpace(req.Name)}
	if err := gl_validate.Struct(&form); err != nil {
		return nil, err
	}
	ss, err := rbac.CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if ss.Subject.Type != v2.Session_EMAIL {
		return nil, errors.New("not an email user")
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var it models.EmailUser
		if err := tx.Where("email = ?", ss.Subject.Sn).First(&it).Error; err != nil {
			return err
		}
		if err = tx.Model(&it).Updates(map[string]interface{}{
			"name": form.Name,
		}).Error; err != nil {
			return err
		}

		return models.CreateLog(tx, uint(ss.User.Id), v2.Plugin(), ss.ClientIp, v2.UserIndexLogResponse_Item_INFO, "update name")
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *EmailUserServer) SetAvatar(ctx context.Context, req *v2.EmailUserSetAvatarRequest) (*emptypb.Empty, error) {
	ss, err := rbac.CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if ss.Subject.Type != v2.Session_EMAIL {
		return nil, errors.New("not an email user")
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var it models.EmailUser
		if err := tx.Where("email = ?", ss.Subject.Sn).First(&it).Error; err != nil {
			return err
		}
		// TODO
		return models.CreateLog(tx, uint(ss.User.Id), v2.Plugin(), ss.ClientIp, v2.UserIndexLogResponse_Item_WARNING, "change password")
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *EmailUserServer) ChangePassword(ctx context.Context, req *v2.EmailUserChangePasswordRequest) (*emptypb.Empty, error) {
	ss, err := rbac.CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if ss.Subject.Type != v2.Session_EMAIL {
		return nil, errors.New("not an email user")
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var it models.EmailUser
		if err := tx.Where("email = ?", ss.Subject.Sn).First(&it).Error; err != nil {
			return err
		}
		if err := it.VerifyPassword(p.hmac, req.CurrentPassword); err != nil {
			return err
		}
		form := models.NewSetPasswordForEmailUserForm(req.NewPassword)
		if err := form.Execute(p.db, &it, p.hmac); err != nil {
			return err
		}
		return models.CreateLog(tx, uint(ss.User.Id), v2.Plugin(), ss.ClientIp, v2.UserIndexLogResponse_Item_WARNING, "change password")
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
