package models

import (
	"errors"
	"fmt"
	"time"

	"github.com/google/uuid"
	"gorm.io/gorm"

	auth_pb "github.com/saturn-xiv/palm/lilac/auth/v2"
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	rbac_pb "github.com/saturn-xiv/palm/lilac/rbac/v2"
)

type Session struct {
	ID           uint32    `gorm:"primaryKey"`
	Uid          string    `gorm:"uniqueIndex;not null;size:36"`
	ProviderType int32     `gorm:"not null"`
	ProviderId   uint32    `gorm:"not null"`
	Ip           string    `gorm:"index;not null;size:45"`
	ExpiredAt    time.Time `gorm:"not null"`
	CreatedAt    time.Time `gorm:"not null"`
}

func NewSession(db *gorm.DB, jwt *crypto.Jwt, issuer string, audience string, provider_type rbac_pb.UserDetail_Provider_Type, provider_id uint32, ttl time.Duration) (string, error) {
	now := time.Now()
	uid := uuid.New().String()
	token, err := jwt.Sign(issuer, uid, audience, map[string]string{}, ttl)
	if err != nil {
		return "", err
	}
	if rst := db.Create(&Session{
		ProviderType: int32(provider_type),
		ProviderId:   provider_id,
		Uid:          uid,
		ExpiredAt:    now.Add(ttl),
		CreatedAt:    now,
	}); rst.Error != nil {
		return "", rst.Error
	}

	return token, nil

}

func UserFromSession(db *gorm.DB, ss *Session) (*User, *auth_pb.UserIndexResponse_Item_Detail, error) {
	if ss.ExpiredAt.Before(time.Now()) {
		return nil, nil, errors.New("session is expired")
	}

	var it User
	var ud *auth_pb.UserIndexResponse_Item_Detail
	switch rbac_pb.UserDetail_Provider_Type(ss.ProviderType) {
	case rbac_pb.UserDetail_Provider_Email:
		var mu EmailUser
		if rst := db.First(&mu, ss.ProviderId); rst.Error != nil {
			return nil, nil, rst.Error
		}
		if mu.ConfirmedAt == nil {
			return nil, nil, errors.New("user isn't verified yet")
		}
		if rst := db.First(&it, mu.UserID); rst.Error != nil {
			return nil, nil, rst.Error
		}
		ud = mu.Detail()
	default:
		return nil, nil, fmt.Errorf("provider type %d isn't support yet", ss.ProviderType)
	}

	if it.DeletedAt != nil {
		return nil, nil, errors.New("user is disabled")
	}
	if it.LockedAt != nil {
		return nil, nil, errors.New("user is locked")
	}
	return &it, ud, nil
}
