package graphql

import (
	"context"
	"time"

	"github.com/graph-gophers/graphql-go"
	"google.golang.org/protobuf/types/known/durationpb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/models"
	s3_v2 "github.com/saturn-xiv/palm/daisy/s3/v2"
)

func (p *Mutation) SetAttachmentUploaded(ctx context.Context, args struct {
	Id graphql.ID
}) (*Ok, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	id, err := FromId(args.Id)
	if err != nil {
		return nil, err
	}
	var item models.Attachment
	if err := p.db.First(&item, id).Error; err != nil {
		return nil, err
	}
	if err := item.CanManage(p.enforcer, ss); err != nil {
		return nil, err
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if err = tx.Model(&models.Attachment{}).Where("id = ?", id).Updates(map[string]interface{}{"updated_at": time.Now()}).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}
func (p *Mutation) SetAttachmentTitle(ctx context.Context, args struct {
	Id    graphql.ID
	Title string
}) (*Ok, error) {
	if err := gl_validate.Struct(&setAttachmentTitleForm{Title: args.Title}); err != nil {
		return nil, err
	}
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	id, err := FromId(args.Id)
	if err != nil {
		return nil, err
	}
	{
		var item models.Attachment
		if err := p.db.First(&item, id).Error; err != nil {
			return nil, err
		}
		if err := item.CanManage(p.enforcer, ss); err != nil {
			return nil, err
		}
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if err = tx.Model(&models.Attachment{}).Where("id = ?", id).Updates(map[string]interface{}{"title": args.Title}).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}

type setAttachmentTitleForm struct {
	Title string `validate:"required,min=2,max=63"`
}

type createAttachmentTitleForm struct {
	Title           string `validate:"required,min=2,max=63"`
	ContentType     string `validate:"required,min=2,max=127"`
	Ttl             int32  `validate:"required,min=300,max=7200"`
	ExpireAfterDays *int32 `validate:"min=1"`
	Size            int32  `validate:"required,min=1"`
}

func (p *Mutation) CreateAttachment(ctx context.Context, args struct {
	Title           string
	ContentType     string
	Size            uint
	Public          bool
	ExpireAfterDays *int32
	Ttl             int32
}) (string, error) {

	if err := gl_validate.Struct(&createAttachmentTitleForm{
		Title:           args.Title,
		ContentType:     args.ContentType,
		Ttl:             args.Ttl,
		ExpireAfterDays: args.ExpireAfterDays,
		Size:            int32(args.Size),
	}); err != nil {
		return "", err
	}
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return "", err
	}

	var expire_after_days *uint32
	if args.ExpireAfterDays != nil {
		v := uint32(*args.ExpireAfterDays)
		expire_after_days = &v
	}

	bucket := s3_v2.BucketName("uploads", args.Public, expire_after_days)
	object := s3_v2.ObjectName(args.Title)
	{
		ok, err := p.minio.BucketExists(ctx, bucket)
		if err != nil {
			return "", err
		}
		if !ok {
			req := &s3_v2.MakeBucketRequest{
				Public:          args.Public,
				ExpireAfterDays: expire_after_days,
				Name:            bucket,
			}
			if err := req.Execute(ctx, p.minio); err != nil {
				return "", err
			}
		}
	}

	upload_req := s3_v2.PresignedPutObjectRequest{Bucket: bucket, Object: object, Ttl: durationpb.New(time.Second * time.Duration(args.Ttl))}
	url, err := upload_req.Execute(ctx, p.minio)
	if err != nil {
		return "", err
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		it := models.Attachment{
			UserID:      uint(ss.User.Id),
			Title:       args.Title,
			ContentType: args.ContentType,
			Bucket:      bucket,
			Object:      object,
			Size:        uint(args.Size),
			Public:      args.Public,
		}
		if args.ExpireAfterDays != nil {
			v := uint(*args.ExpireAfterDays)
			it.ExpireAfterDays = &v
		}
		if err = tx.Create(&it).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return "", err
	}
	return url.String(), nil
}

func (p *Mutation) DestroyAttachment(ctx context.Context, args struct {
	Id graphql.ID
}) (*Ok, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	id, err := FromId(args.Id)
	if err != nil {
		return nil, err
	}
	var item models.Attachment
	if err := p.db.First(&item, id).Error; err != nil {
		return nil, err
	}
	if err := item.CanManage(p.enforcer, ss); err != nil {
		return nil, err
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if err := tx.Where("attachment_id = ?", id).Delete(&models.AttachmentResource{}).Error; err != nil {
			return err
		}
		if err := tx.Where("id = ?", id).Delete(&models.Attachment{}).Error; err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}

func (p *Query) ShowAttachment(ctx context.Context, args struct {
	Id graphql.ID
}) (*Attachment, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	id, err := FromId(args.Id)
	if err != nil {
		return nil, err
	}
	var item models.Attachment
	if err := p.db.First(&item, id).Error; err != nil {
		return nil, err
	}
	if err := item.CanManage(p.enforcer, ss); err != nil {
		return nil, err
	}
	return &Attachment{item: &item, db: p.db}, nil
}

func (p *Query) IndexAttachment(ctx context.Context, args struct {
	Page Page
}) (*IndexAttachmentResponse, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}

	if err = ss.User.IsAdministrator(p.enforcer); err == nil {
		var total int64
		if err := p.db.Model(&models.Attachment{}).Count(&total).Error; err != nil {
			return nil, err
		}
		pagination := NewPagination(&args.Page, uint(total))
		var items []models.Attachment
		if err := p.db.Offset(int(pagination.current.Offset())).Limit(int(pagination.current.Size)).Order("uploaded_at DESC").Find(&items).Error; err != nil {
			return nil, err
		}
		return &IndexAttachmentResponse{items: items, pagination: pagination, db: p.db}, nil
	}

	var total int64
	if err := p.db.Model(&models.Attachment{}).Where("user_id = ?", ss.User.Id).Count(&total).Error; err != nil {
		return nil, err
	}
	pagination := NewPagination(&args.Page, uint(total))
	var items []models.Attachment
	if err := p.db.Where("user_id = ?", ss.User.Id).Offset(int(pagination.current.Offset())).Limit(int(pagination.current.Size)).Order("uploaded_at DESC").Find(&items).Error; err != nil {
		return nil, err
	}
	return &IndexAttachmentResponse{items: items, pagination: pagination, db: p.db}, nil
}

type Attachment struct {
	item *models.Attachment
	db   *gorm.DB
}

func (p *Attachment) Id() graphql.ID {
	return ToId(p.item.ID)
}
func (p *Attachment) Size() int32 {
	return int32(p.item.Size)
}
func (p *Attachment) Title() string {
	return p.item.Title
}
func (p *Attachment) Bucket() string {
	return p.item.Bucket
}
func (p *Attachment) Object() string {
	return p.item.Object
}
func (p *Attachment) ContentType() string {
	return p.item.ContentType
}
func (p *Attachment) Public() bool {
	return p.item.Public
}
func (p *Attachment) UploadedAt() *graphql.Time {
	if p.item.UploadedAt == nil {
		return nil
	}
	return &graphql.Time{Time: *p.item.UploadedAt}
}
func (p *Attachment) UpdatedAt() graphql.Time {
	return graphql.Time{Time: p.item.UpdatedAt}
}
func (p *Attachment) Resources() ([]*Resource, error) {
	var items []models.AttachmentResource
	if err := p.db.Where("attachment_id = ?", p.item.ID).Order("created_at DESC").Find(&items).Error; err != nil {
		return nil, err
	}
	var res []*Resource
	for _, it := range items {
		res = append(res, &Resource{type_: it.ResourceType, id: it.ResourceId})
	}
	return res, nil
}

type IndexAttachmentResponse struct {
	items      []models.Attachment
	pagination *Pagination
	db         *gorm.DB
}

func (p *IndexAttachmentResponse) Pagination() *Pagination {
	return p.pagination
}
func (p *IndexAttachmentResponse) Items() []*Attachment {
	var items []*Attachment
	for _, it := range p.items {
		items = append(items, &Attachment{item: &it, db: p.db})
	}
	return items
}
