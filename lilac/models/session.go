package models

import (
	"errors"
	"fmt"
	"time"

	"gorm.io/gorm"

	"github.com/google/uuid"
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type Session struct {
	ID           uint32    `gorm:"primaryKey"`
	Uid          string    `gorm:"uniqueIndex;not null;size:36"`
	ProviderType string    `gorm:"index;index:,unique,composite:provider;not null;size:31"`
	ProviderId   uint32    `gorm:"index:,unique,composite:provider;not null"`
	Ip           string    `gorm:"index;not null;size:45"`
	ExpiredAt    time.Time `gorm:"not null"`
	CreatedAt    time.Time `gorm:"not null"`
}

func NewSession(db *gorm.DB, jwt *crypto.Jwt, issuer string, audience string, provider_type pb.UserIndexResponse_Item_ProviderType, provider_id uint32, ttl time.Duration) (string, error) {
	now := time.Now()
	uid := uuid.New().String()
	token, err := jwt.Sign(issuer, uid, audience, map[string]string{}, ttl)
	if err != nil {
		return "", err
	}
	if rst := db.Create(&Session{
		ProviderType: provider_type.String(),
		ProviderId:   provider_id,
		Uid:          uid,
		ExpiredAt:    now.Add(ttl),
		CreatedAt:    now,
	}); rst.Error != nil {
		return "", rst.Error
	}

	return token, nil

}

func UserFromSession(db *gorm.DB, ss *Session) (*User, *pb.UserIndexResponse_Item_Detail, error) {
	if ss.ExpiredAt.Before(time.Now()) {
		return nil, nil, errors.New("session is expired")
	}
	provider_type, err := pb.NewUserProviderType(ss.ProviderType)
	if err != nil {
		return nil, nil, err
	}

	var it User
	var ud *pb.UserIndexResponse_Item_Detail
	switch provider_type {
	case pb.UserIndexResponse_Item_Email:
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
		return nil, nil, fmt.Errorf("provider type %s isn't support yet", ss.ProviderType)
	}

	if it.DeletedAt != nil {
		return nil, nil, errors.New("user is disabled")
	}
	if it.LockedAt != nil {
		return nil, nil, errors.New("user is locked")
	}
	return &it, ud, nil
}
