package models

import (
	"time"
)

type Locale struct {
	ID        uint64
	Lang      string
	Code      string
	Message   string
	Version   uint32
	UpdatedAt time.Time
	CreatedAt time.Time
}

type Setting struct {
	ID        uint64
	UserID    *uint
	Key       string
	Value     []byte
	Salt      *[]byte
	Version   uint32
	UpdatedAt time.Time
	CreatedAt time.Time
}

type User struct {
	ID              uint64
	UID             string
	Lang            string
	Timezone        string
	SignInCount     uint32
	CurrentSignInAt *time.Time
	CurrentSignInIP *string
	LastSignInAt    *time.Time
	LastSignInIP    *string
	LockedAt        *time.Time
	DeletedAt       *time.Time
	Version         uint32
	UpdatedAt       time.Time
	CreatedAt       time.Time
}

type Log struct {
	ID           uint64
	UserID       uint64
	Plugin       string
	IP           string
	Level        string
	ResourceType string
	ResourceID   *uint64
	Message      string
	CreatedAt    time.Time
}

type Session struct {
	ID           uint64
	UserID       uint64
	UID          string
	ProviderType string
	ProviderID   uint64
	IP           string
	ExpiresAt    time.Time
	DeletedAt    *time.Time
	CreatedAt    time.Time
}

type EmailUser struct {
	ID          uint64
	UserID      uint64
	RealName    string
	Nickname    string
	Email       string
	Password    []byte
	Salt        []byte
	Avatar      string
	ConfirmedAt *time.Time
	DeletedAt   *time.Time
	Version     uint32
	UpdatedAt   time.Time
	CreatedAt   time.Time
}

type GoogleOauth2User struct {
	ID            uint64
	UserID        uint64
	Subject       string
	Email         *string
	EmailVerified bool
	Name          *string
	Picture       *string
	Locale        *string
	Token         []byte
	DeletedAt     *time.Time
	Version       uint32
	UpdatedAt     time.Time
	CreatedAt     time.Time
}

type WechatOauth2User struct {
	ID         uint64
	UserID     uint64
	UnionID    string
	AppID      string
	OpenID     string
	Nickname   string
	Sex        uint8
	City       string
	Province   string
	Country    string
	HeadImgURL *string
	Privilege  []byte
	Lang       string
	DeletedAt  *time.Time
	Version    uint32
	UpdatedAt  time.Time
	CreatedAt  time.Time
}

type WechatMiniProgramUser struct {
	ID        uint64
	UserID    uint64
	UnionID   string
	AppID     string
	OpenID    string
	Nickname  *string
	AvatarURL *string
	DeletedAt *time.Time
	Version   uint32
	UpdatedAt time.Time
	CreatedAt time.Time
}

type LeaveWord struct {
	ID          uint64
	Lang        string
	IP          string
	Body        string
	Editor      string
	Status      string
	PublishedAt *time.Time
	DeletedAt   *time.Time
	Version     uint32
	UpdatedAt   time.Time
	CreatedAt   time.Time
}
