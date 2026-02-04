package rbac

import (
	"context"
	"errors"
	"fmt"
	"strings"

	"github.com/go-playground/validator/v10"
	"google.golang.org/grpc/metadata"
	"google.golang.org/grpc/peer"
	"google.golang.org/protobuf/types/known/timestamppb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/crypto"
	"github.com/saturn-xiv/palm/daisy/env"
	"github.com/saturn-xiv/palm/daisy/models"
	portal_v2 "github.com/saturn-xiv/palm/daisy/portal/v2"
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

func CurrentUser(ctx context.Context, db *gorm.DB, jwt *crypto.Jwt) (*portal_v2.Session, error) {
	md, ok := metadata.FromIncomingContext(ctx)
	if !ok {
		return nil, errors.New("empty metadata header")
	}

	if auth, ok := md[strings.ToLower(Authorization)]; ok {
		for _, it := range auth {
			if user, ss, err := NewCurrentUserByAuthorization(it, db, jwt); err == nil {
				ss.User = NewUser(user)
				ss.ClientIp = client_ip(ctx)
				return ss, nil
			}
		}
	}

	return nil, errors.New("invalid authorization header")
}

func client_ip(ctx context.Context) string {
	if md, ok := metadata.FromIncomingContext(ctx); ok {
		for _, k := range []string{XForwardedFor, XRealIp} {
			if items, found := md[strings.ToLower(k)]; found && len(items) > 0 {
				return items[0]
			}
		}
	}
	if p, ok := peer.FromContext(ctx); ok {
		return p.Addr.String()
	}
	return "n/a"
}

func NewUser(it *models.User) *portal_v2.UserIndexResponse_Item {
	v := portal_v2.UserIndexResponse_Item{
		Id:                int64(it.ID),
		Sn:                it.Sn,
		Lang:              it.Lang,
		Timezone:          it.Timezone,
		SignedInTotal:     int64(it.SignedInTotal),
		UpdatedAt:         timestamppb.New(it.UpdatedAt),
		CurrentSignedInIp: it.CurrentSignedInIp,
		LastSignedInIp:    it.LastSignedInIp,
	}
	if it.DeletedAt.Valid {
		v.DeletedAt = timestamppb.New(it.DeletedAt.Time)
	}
	if it.CurrentSignedInAt != nil {
		v.CurrentSignedInAt = timestamppb.New(*it.CurrentSignedInAt)
	}
	if it.LastSignedInAt != nil {
		v.LastSignedInAt = timestamppb.New(*it.LastSignedInAt)
	}
	return &v
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

	var ss portal_v2.Session
	ss.Subject, err = portal_v2.NewSubject(sub)
	if err != nil {
		return nil, nil, err
	}
	user, err := current_user(db, &ss)
	if err != nil {
		return nil, nil, err
	}
	return user, &ss, nil
}

func current_user(db *gorm.DB, ss *portal_v2.Session) (*models.User, error) {
	switch ss.Subject.Type {
	case portal_v2.Session_GOOGLE_OAUTH2:
		var it models.GoogleOauth2User
		if err := db.Where("sn = ?", ss.Subject.Sn).Preload("User").First(&it).Error; err != nil {
			return nil, err
		}
		if it.User.LockedAt != nil {
			return nil, fmt.Errorf("user %s is locked", it.Name)
		}

		ss.Avatar = it.Picture
		return it.User, nil
	case portal_v2.Session_EMAIL:
		var it models.EmailUser
		if err := db.Where("email = ?", ss.Subject.Sn).Preload("User").First(&it).Error; err != nil {
			return nil, err
		}
		if it.ConfirmedAt == nil {
			return nil, fmt.Errorf("user %s isn't confirmed", it.Name)
		}
		if it.User.LockedAt != nil {
			return nil, fmt.Errorf("user %s is locked", it.Name)
		}

		if it.Avatar != nil {
			ss.Avatar = *it.Avatar
		}
		return it.User, nil
	default:
		return nil, fmt.Errorf("unsupported %s yet", ss.Subject.Type.String())
	}

}
