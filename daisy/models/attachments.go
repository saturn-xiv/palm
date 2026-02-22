package models

import (
	"time"

	"github.com/casbin/casbin/v3"

	portal_v2 "github.com/saturn-xiv/palm/daisy/portal/v2"
)

type Attachment struct {
	Model

	UserID          uint   `gorm:"not null"`
	Title           string `gorm:"index;not null;size:127"`
	Bucket          string `gorm:"uniqueIndex:idx_attachments_bucket_object;not null;size:63"`
	Object          string `gorm:"uniqueIndex:idx_attachments_bucket_object;not null;size:63"`
	ContentType     string `gorm:"index;not null;size:127"`
	Size            uint   `gorm:"not null"`
	Public          bool   `gorm:"not null;default:false"`
	UploadedAt      *time.Time
	ExpireAfterDays *uint

	Resources []*AttachmentResource
	User      *User
}

func (Attachment) TableName() string {
	return "attachments"
}

func (p *Attachment) CanManage(enforcer *casbin.Enforcer, ss *portal_v2.Session) error {
	if ss.User.Id == int64(p.UserID) {
		return nil
	}
	return ss.User.IsAdministrator(enforcer)
}

func (p *Attachment) Available() bool {
	if p.DeletedAt.Valid {
		return false
	}
	if p.UploadedAt == nil {
		return false
	}
	if p.ExpireAfterDays != nil {
		if time.Now().After(p.UpdatedAt.Add(time.Duration(*p.ExpireAfterDays) * 24 * time.Hour).Add(time.Minute * -1)) {
			return false
		}
	}
	return true
}

type AttachmentResource struct {
	ID           uint   `gorm:"primarykey"`
	AttachmentID uint   `gorm:"not null"`
	ResourceType string `gorm:"index;not null;size:127"`
	ResourceId   *uint
	CreatedAt    time.Time `gorm:"not null"`

	Attachment *Attachment
}

func (AttachmentResource) TableName() string {
	return "attachments_resources"
}
