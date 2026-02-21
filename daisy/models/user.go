package models

import (
	"time"

	"github.com/casbin/casbin/v3"
	"github.com/google/uuid"
	"golang.org/x/text/language"
	"gorm.io/gorm"

	rbac_v2 "github.com/saturn-xiv/palm/daisy/rbac/v2"
)

type User struct {
	Model

	Sn                string `gorm:"uniqueIndex;not null;size:36"`
	Lang              string `gorm:"index;not null;size:15;default:'en-US'"`
	Timezone          string `gorm:"index;not null;size:31;default:'UTC'"`
	SignedInTotal     uint   `gorm:"not null;default:0"`
	CurrentSignedInAt *time.Time
	CurrentSignedInIp *string `gorm:"size:45"`
	LastSignedInAt    *time.Time
	LastSignedInIp    *string `gorm:"size:45"`
	LockedAt          *time.Time

	Logs        []*Log
	Attachments []*Attachment
}

func (User) TableName() string {
	return "users"
}

func (p *User) IsAdministrator(enforcer *casbin.Enforcer) error {
	return p.Has(enforcer, &rbac_v2.Subject_Role{
		By: &rbac_v2.Subject_Role_Administrator_{
			Administrator: &rbac_v2.Subject_Role_Administrator{},
		},
	})
}

func (p *User) IsRoot(enforcer *casbin.Enforcer) error {
	return p.Has(enforcer, &rbac_v2.Subject_Role{
		By: &rbac_v2.Subject_Role_Root_{
			Root: &rbac_v2.Subject_Role_Root{},
		},
	})
}

func (p *User) Has(enforcer *casbin.Enforcer, role *rbac_v2.Subject_Role) error {
	return rbac_v2.Has(enforcer,
		&rbac_v2.Subject_User{
			By: &rbac_v2.Subject_User_Id{
				Id: int64(p.ID),
			},
		}, role)
}

func CreateUser(db *gorm.DB, lang *language.Tag, timezone *time.Location) (*User, error) {
	sn := uuid.New().String()
	if err := db.Create(&User{Sn: sn, Lang: lang.String(), Timezone: timezone.String()}).Error; err != nil {
		return nil, err
	}
	var it User
	if err := db.Where("sn = ?", sn).First(&it).Error; err != nil {
		return nil, err
	}
	return &it, nil
}

func SignInUser(db *gorm.DB, user *User, ip string) error {
	now := time.Now()

	if err := db.Model(&user).Updates(map[string]interface{}{
		"current_signed_in_at": &now,
		"current_signed_in_ip": ip,
		"last_signed_in_at":    user.CurrentSignedInAt,
		"last_signed_in_ip":    user.CurrentSignedInIp,
		"signed_in_total":      user.SignedInTotal + 1,
		"version":              user.Version + 1,
	}).Error; err != nil {
		return err
	}
	return nil
}

func UserBySn(db *gorm.DB, sn string) (*User, error) {
	var user User
	if err := db.Where("sn = ?", sn).First(&user).Error; err != nil {
		return nil, err
	}

	return &user, nil
}

func UserById(db *gorm.DB, id uint) (*User, error) {
	var user User
	if err := db.Where("id = ?", id).First(&user).Error; err != nil {
		return nil, err
	}

	return &user, nil
}
