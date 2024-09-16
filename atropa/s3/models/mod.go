package models

import "time"

type Attachment struct {
	ID          uint64
	UserID      uint64
	Bucket      string
	Object      string
	Title       string
	Size        uint64
	ContentType string
	UploadedAt  *time.Time
	DeletedAt   *time.Time
	Version     uint32
	UpdatedAt   time.Time
	CreatedAt   time.Time
}

type AttachmentResource struct {
	ID           uint64
	AttachmentID uint64
	ResourceType string
	ResourceID   *uint64
	CreatedAt    time.Time
}
