package models

import (
	"time"
)

type Locale struct {
	ID        uint
	Lang      string
	Code      string
	Message   string
	Version   int
	UpdatedAt time.Time
	CreatedAt time.Time
}

type Setting struct {
	ID        uint
	UserID    *uint
	Key       string
	Value     []byte
	Salt      *[]byte
	Version   int
	UpdatedAt time.Time
	CreatedAt time.Time
}

type User struct {
	ID              uint
	UID             string
	Name            *string
	Avatar          *string
	Lang            string
	Timezone        string
	SignInCount     int
	CurrentSignInAt *time.Time
	CurrentSignInIP *string
	LastSignInAt    *time.Time
	LastSignInIP    *string
	LockedAt        *time.Time
	DeletedAt       *time.Time
	Version         int
	UpdatedAt       time.Time
	CreatedAt       time.Time
}

type Log struct {
	ID           uint
	UserID       uint
	Plugin       string
	IP           string
	Level        string
	ResourceType string
	ResourceID   *uint
	Message      string
	CreatedAt    time.Time
}

type Session struct {
	ID           uint
	UserID       uint
	UID          string
	ProviderType string
	ProviderID   uint
	IP           string
	ExpiresAt    time.Time
	DeletedAt    *time.Time
	CreatedAt    time.Time
}

type EmailUser struct {
	ID          uint
	UserID      uint
	RealName    string
	Nickname    string
	Email       string
	Password    []byte
	Salt        []byte
	Avatar      string
	ConfirmedAt *time.Time
	DeletedAt   *time.Time
	Version     int
	UpdatedAt   time.Time
	CreatedAt   time.Time
}

type GoogleOauth2User struct {
	ID            uint
	UserID        uint
	Subject       string
	Email         *string
	EmailVerified bool
	Name          *string
	Picture       *string
	Locale        *string
	Token         []byte
	DeletedAt     *time.Time
	Version       int
	UpdatedAt     time.Time
	CreatedAt     time.Time
}

type WechatOauth2User struct {
	ID         uint
	UserID     uint
	UnionID    string
	AppID      string
	OpenID     string
	Nickname   string
	Sex        int
	City       string
	Province   string
	Country    string
	HeadImgURL *string
	Privilege  []byte
	Lang       string
	DeletedAt  *time.Time
	Version    int
	UpdatedAt  time.Time
	CreatedAt  time.Time
}

type WechatMiniProgramUser struct {
	ID        uint
	UserID    uint
	UnionID   string
	AppID     string
	OpenID    string
	Nickname  *string
	AvatarURL *string
	DeletedAt *time.Time
	Version   int
	UpdatedAt time.Time
	CreatedAt time.Time
}

type Attachment struct {
	ID          uint
	UserID      uint
	Bucket      string
	Object      string
	Title       string
	Size        uint
	ContentType string
	UploadedAt  *time.Time
	DeletedAt   *time.Time
	Version     int
	UpdatedAt   time.Time
	CreatedAt   time.Time
}

type AttachmentResource struct {
	ID           uint
	AttachmentID uint
	ResourceType string
	ResourceID   uint
	CreatedAt    time.Time
}

type LeaveWord struct {
	ID          uint
	Lang        string
	IP          uint
	Body        string
	Editor      string
	Status      string
	PublishedAt *time.Time
	DeletedAt   *time.Time
	Version     int
	UpdatedAt   time.Time
	CreatedAt   time.Time
}
