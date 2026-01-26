package rbac

import (
	"context"
	"errors"
	"fmt"
	"slices"
	"strings"

	"github.com/casbin/casbin/v2"
	"github.com/go-playground/validator/v10"
	"google.golang.org/grpc/metadata"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/crypto"
	"github.com/saturn-xiv/palm/daisy/env"
	"github.com/saturn-xiv/palm/daisy/models"
	portal_v2 "github.com/saturn-xiv/palm/daisy/portal/v2"
	v2 "github.com/saturn-xiv/palm/daisy/rbac/v2"
)

var (
	ContentType   = "Content-Type"
	Authorization = "Authorization"
	Bearer        = "Bearer "
	XForwardedFor = "X-Forwarded-For"
	XRealIp       = "X-Real-IP"

	UserSignInAudience = "user.sign-in"
)

var gl_validate = validator.New(validator.WithRequiredStructEnabled())

func CurrentUser(ctx context.Context, db *gorm.DB, jwt *crypto.Jwt) (*models.User, *portal_v2.Session, error) {
	md, ok := metadata.FromIncomingContext(ctx)
	if !ok {
		return nil, nil, errors.New("empty metadata header")
	}

	if auth, ok := md[strings.ToLower(Authorization)]; ok {
		for _, it := range auth {
			if user, ss, err := NewCurrentUserByAuthorization(it, db, jwt); err == nil {
				return user, ss, nil
			}
		}
	}

	return nil, nil, errors.New("invalid authorization header")
}

func NewCurrentUserByAuthorization(auth string, db *gorm.DB, jwt *crypto.Jwt) (*models.User, *portal_v2.Session, error) {
	if !strings.HasPrefix(auth, Bearer) {
		return nil, nil, errors.New("not a jwt bearer token")
	}
	token := strings.TrimPrefix(auth, Bearer)
	_, sub, err := jwt.Verify(token, env.Plugin(), UserSignInAudience)
	if err != nil {
		return nil, nil, err
	}

	ss, err := portal_v2.NewSession(sub)
	if err != nil {
		return nil, nil, err
	}
	user, err := current_user(db, ss)
	if err != nil {
		return nil, nil, err
	}
	return user, ss, nil
}

func current_user(db *gorm.DB, ss *portal_v2.Session) (*models.User, error) {
	switch ss.Type {
	case portal_v2.User_GOOGLE_OAUTH2:
		var it models.GoogleOauth2User
		if err := db.Where("sn = ?", ss.Sn).Preload("User").First(&it).Error; err != nil {
			return nil, err
		}
		if it.User.LockedAt != nil {
			return nil, fmt.Errorf("user %s is locked", it.Name)
		}
		return it.User, nil
	case portal_v2.User_EMAIL:
		var it models.EmailUser
		if err := db.Where("email = ?", ss.Sn).Preload("User").First(&it).Error; err != nil {
			return nil, err
		}
		if it.ConfirmedAt == nil {
			return nil, fmt.Errorf("user %s isn't confirmed", it.Name)
		}
		if it.User.LockedAt != nil {
			return nil, fmt.Errorf("user %s is locked", it.Name)
		}
		return it.User, nil
	default:
		return nil, fmt.Errorf("unsupported %s yet", ss.Type.String())
	}

}

func HasRole(enforcer *casbin.Enforcer, user *models.User, role string) error {
	return has_role_(enforcer, user, &v2.Subject_Role{
		By: &v2.Subject_Role_Code{
			Code: role,
		},
	})
}

func IsAdministrator(enforcer *casbin.Enforcer, user *models.User) error {
	return has_role_(enforcer, user, &v2.Subject_Role{
		By: &v2.Subject_Role_Administrator_{
			Administrator: &v2.Subject_Role_Administrator{},
		},
	})
}

func IsRoot(enforcer *casbin.Enforcer, user *models.User) error {
	return has_role_(enforcer, user, &v2.Subject_Role{
		By: &v2.Subject_Role_Root_{
			Root: &v2.Subject_Role_Root{},
		},
	})
}

func has_role_(enforcer *casbin.Enforcer, user *models.User, role *v2.Subject_Role) error {
	return has_role(enforcer,
		&v2.Subject_User{
			By: &v2.Subject_User_Id{
				Id: int64(user.ID),
			},
		}, role)
}

func has_role(enforcer *casbin.Enforcer, user *v2.Subject_User, role *v2.Subject_Role) error {
	role_ := v2.Subject{By: &v2.Subject_Role_{Role: role}}
	role_s, err := role_.ToString()
	if err != nil {
		return err
	}

	user_ := v2.Subject{By: &v2.Subject_User_{User: user}}
	user_s, err := user_.ToString()
	if err != nil {
		return err
	}

	items, err := enforcer.GetImplicitRolesForUser(user_s)
	if err != nil {
		return err
	}
	if slices.Contains(items, role_s) {
		return nil
	}
	return errors.New("deny")
}
