package portal

import (
	"context"
	"fmt"
	"time"

	"github.com/casbin/casbin/v3"
	"github.com/minio/minio-go/v7"
	"golang.org/x/text/language"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"google.golang.org/protobuf/types/known/emptypb"
	"google.golang.org/protobuf/types/known/timestamppb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/crypto"
	"github.com/saturn-xiv/palm/daisy/models"
	v2 "github.com/saturn-xiv/palm/daisy/portal/v2"
	"github.com/saturn-xiv/palm/daisy/rbac"
	s3_v2 "github.com/saturn-xiv/palm/daisy/s3/v2"
)

type UserServer struct {
	v2.UnimplementedUserServer

	db       *gorm.DB
	enforcer *casbin.Enforcer
	jwt      *crypto.Jwt
	hmac     *crypto.Hmac
	aead     *crypto.Aead
	s3       *minio.Client
}

func NewUserServer(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer, s3 *minio.Client, hmac *crypto.Hmac, aead *crypto.Aead) *UserServer {
	return &UserServer{db: db, s3: s3, enforcer: enforcer, jwt: jwt, hmac: hmac, aead: aead}
}

func (p *UserServer) IndexAttachment(ctx context.Context, req *v2.Page) (*v2.UserIndexAttachmentResponse, error) {
	ss, err := rbac.CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	var total int64
	if err := p.db.Model(&models.Attachment{}).Where("user_id = ?", ss.User.Id).Count(&total).Error; err != nil {
		return nil, err
	}
	pagination := v2.NewPagination(req, total)
	var items []models.Attachment
	if err := p.db.Where("user_id = ?", ss.User.Id).Order("updated_at DESC").Offset(int(pagination.Current.Offset())).Limit(int(pagination.Current.Size)).Find(&items).Error; err != nil {
		return nil, err
	}
	res := v2.UserIndexAttachmentResponse{
		Items:      []*v2.UserIndexAttachmentResponse_Item{},
		Pagination: pagination,
	}
	for _, it := range items {
		res.Items = append(res.Items, new_attachment(&it))
	}
	return &res, nil
}
func (p *UserServer) CreateAttachment(ctx context.Context, req *v2.UserCreateAttachmentRequest) (*v2.UserCreateAttachmentUploadResponse, error) {
	ss, err := rbac.CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	bucket := req.Bucket()
	object := req.Object()
	req_ := s3_v2.PutObjectRequest{Bucket: bucket, Object: object, Ttl: req.Ttl}
	url, err := req_.Execute(ctx, p.s3)
	if err != nil {
		return nil, err
	}
	{
		it := models.Attachment{
			UserID:      uint(ss.User.Id),
			Title:       req.Title,
			ContentType: req.ContentType,
			Bucket:      bucket,
			Object:      object,
			Size:        uint(req.Size),
			Public:      req.Public,
		}
		if req.ExpireAfterDays != nil {
			v := uint(*req.ExpireAfterDays)
			it.ExpireAfterDays = &v
		}
		if err = p.db.Create(&it).Error; err != nil {
			return nil, err
		}
	}
	var it models.Attachment
	if err = p.db.Where("bucket = ? AND object = ?", bucket, object).First(&it).Error; err != nil {
		return nil, err
	}
	return &v2.UserCreateAttachmentUploadResponse{
		Item: new_attachment(&it),
		Url:  url.String(),
	}, nil
}
func (p *UserServer) ShowAttachment(ctx context.Context, req *v2.UserShowAttachmentRequest) (*v2.UserShowAttachmentResponse, error) {
	ss, err := rbac.CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	var it models.Attachment
	if err = p.db.First(&it, req.Id).Error; err != nil {
		return nil, err
	}
	if it.UserID != uint(ss.User.Id) {
		return nil, status.Error(codes.PermissionDenied, "set attachment uploaded")
	}
	if !it.Available() {
		return nil, status.Error(codes.ResourceExhausted, "attachment isn't available")
	}
	if it.Public {
		req_ := s3_v2.GetObjectRequest{Bucket: it.Bucket, Object: it.Object}
		return &v2.UserShowAttachmentResponse{Url: req_.Execute(p.s3).String()}, nil
	}
	req_ := s3_v2.PresignedGetObjectRequest{Bucket: it.Bucket, Object: it.Object, Ttl: req.Ttl}
	if req.Download {
		req_.Title = &it.Title
	}
	url, err := req_.Execute(ctx, p.s3)
	if err != nil {
		return nil, err
	}
	return &v2.UserShowAttachmentResponse{Url: url.String()}, nil
}
func (p *UserServer) SetAttachmentUploaded(ctx context.Context, req *v2.IdRequest) (*emptypb.Empty, error) {
	ss, err := rbac.CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	var it models.Attachment
	if err = p.db.First(&it, req.Id).Error; err != nil {
		return nil, err
	}
	if it.UserID != uint(ss.User.Id) {
		return nil, status.Error(codes.PermissionDenied, "set attachment uploaded")
	}
	if it.UploadedAt != nil {
		return nil, status.Error(codes.ResourceExhausted, "attachment already uploaded")
	}

	if err := p.db.Model(&it).Updates(map[string]interface{}{"updated_at": time.Now()}).Error; err != nil {
		return nil, err
	}

	return &emptypb.Empty{}, nil
}
func (p *UserServer) DestroyAttachment(ctx context.Context, req *v2.IdRequest) (*emptypb.Empty, error) {
	ss, err := rbac.CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	var it models.Attachment
	if err = p.db.First(&it, req.Id).Error; err != nil {
		return nil, err
	}
	if it.UserID != uint(ss.User.Id) {
		return nil, status.Error(codes.PermissionDenied, "destroy attachment")
	}
	if err = p.db.Delete(&it).Error; err != nil {
		return nil, err
	}
	if it.Available() {
		req := s3_v2.RemoveObjectRequest{Bucket: it.Bucket, Object: it.Object}
		if err = req.Execute(ctx, p.s3); err != nil {
			return nil, err
		}
	}
	return &emptypb.Empty{}, nil
}

func (p *UserServer) SetAttachmentTitle(ctx context.Context, req *v2.UserSetAttachmentTitleRequest) (*emptypb.Empty, error) {
	ss, err := rbac.CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	var it models.Attachment
	if err = p.db.First(&it, req.Id).Error; err != nil {
		return nil, err
	}
	if it.UserID != uint(ss.User.Id) {
		return nil, status.Error(codes.PermissionDenied, "set attachment title")
	}
	if err = p.db.Model(&it).Updates(map[string]interface{}{"title": req.Title}).Error; err != nil {
		return nil, err
	}
	if it.Available() {
		req := s3_v2.RemoveObjectRequest{Bucket: it.Bucket, Object: it.Object}
		if err = req.Execute(ctx, p.s3); err != nil {
			return nil, err
		}
	}
	return &emptypb.Empty{}, nil
}

func (p *UserServer) Index(ctx context.Context, req *v2.Page) (*v2.UserIndexResponse, error) {
	{
		ss, err := rbac.CurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err = ss.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	var total int64
	if err := p.db.Model(&models.User{}).Count(&total).Error; err != nil {
		return nil, err
	}
	pagination := v2.NewPagination(req, total)
	var items []models.User
	if err := p.db.Order("updated_at DESC").Offset(int(pagination.Current.Offset())).Limit(int(pagination.Current.Size)).Find(&items).Error; err != nil {
		return nil, err
	}
	res := v2.UserIndexResponse{
		Items:      []*v2.UserIndexResponse_Item{},
		Pagination: pagination,
	}
	for _, it := range items {
		res.Items = append(res.Items, rbac.NewUser(&it))
	}
	return &res, nil
}
func (p *UserServer) Destroy(ctx context.Context, req *v2.IdRequest) (*emptypb.Empty, error) {
	ss, err := rbac.CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err = ss.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}

	var it models.User
	if err := p.db.First(&it, req.Id).Error; err != nil {
		return nil, err
	}
	if err := ss.IsRoot(p.enforcer); err == nil {
		return nil, status.Error(codes.PermissionDenied, "it is a root user")
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if err := tx.Delete(&it).Error; err != nil {
			return err
		}
		return models.CreateLog(tx, it.ID, v2.Plugin(), ss.ClientIp, v2.UserIndexLogResponse_Item_WARNING, fmt.Sprintf("deleted by %s", ss.Name))
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *UserServer) Lock(ctx context.Context, req *v2.IdRequest) (*emptypb.Empty, error) {
	ss, err := rbac.CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err = ss.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}

	var it models.User
	if err := p.db.First(&it, req.Id).Error; err != nil {
		return nil, err
	}
	if err := ss.IsRoot(p.enforcer); err == nil {
		return nil, status.Error(codes.PermissionDenied, "it is a root user")
	}
	if it.LockedAt != nil {
		return nil, status.Error(codes.InvalidArgument, "user is already locked")
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if err := tx.Model(&it).Updates(map[string]interface{}{"locked_at": nil}).Error; err != nil {
			return err
		}
		return models.CreateLog(tx, it.ID, v2.Plugin(), ss.ClientIp, v2.UserIndexLogResponse_Item_WARNING, fmt.Sprintf("lock by %s", ss.Name))
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *UserServer) Unlock(ctx context.Context, req *v2.IdRequest) (*emptypb.Empty, error) {
	ss, err := rbac.CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err = ss.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}

	var it models.User
	if err := p.db.First(&it, req.Id).Error; err != nil {
		return nil, err
	}
	if err := ss.IsRoot(p.enforcer); err == nil {
		return nil, status.Error(codes.PermissionDenied, "it is a root user")
	}
	if it.LockedAt == nil {
		return nil, status.Error(codes.InvalidArgument, "user isn't locked")
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if err := tx.Model(&it).Updates(map[string]interface{}{"locked_at": nil}).Error; err != nil {
			return err
		}
		return models.CreateLog(tx, it.ID, v2.Plugin(), ss.ClientIp, v2.UserIndexLogResponse_Item_WARNING, fmt.Sprintf("unlock by %s", ss.Name))
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *UserServer) IndexLog(ctx context.Context, req *v2.Page) (*v2.UserIndexLogResponse, error) {
	ss, err := rbac.CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	var total int64
	if err := p.db.Model(&models.Log{}).Where("user_id = ?", ss.User.Id).Count(&total).Error; err != nil {
		return nil, err
	}
	pagination := v2.NewPagination(req, total)
	var items []models.Log
	if err := p.db.Where("user_id = ?", ss.User.Id).Order("created_at DESC").Offset(int(pagination.Current.Offset())).Limit(int(pagination.Current.Size)).Find(&items).Error; err != nil {
		return nil, err
	}
	res := v2.UserIndexLogResponse{
		Items:      []*v2.UserIndexLogResponse_Item{},
		Pagination: pagination,
	}
	for _, it := range items {
		res.Items = append(res.Items, new_log(&it))
	}
	return &res, nil
}
func (p *UserServer) SetLang(ctx context.Context, req *v2.UserSetLangRequest) (*emptypb.Empty, error) {
	tag, err := language.Parse(req.Lang)
	if err != nil {
		return nil, err
	}
	ss, err := rbac.CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if err := tx.Model(&models.User{}).Where("id = ?", ss.User.Id).Updates(map[string]interface{}{"lang": tag.String()}).Error; err != nil {
			return err
		}
		return models.CreateLog(tx, uint(ss.User.Id), v2.Plugin(), ss.ClientIp, v2.UserIndexLogResponse_Item_INFO, "update lang")
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *UserServer) SetTimezone(ctx context.Context, req *v2.UserSetTimezoneRequest) (*emptypb.Empty, error) {
	tz, err := time.LoadLocation(req.Timezone)
	if err != nil {
		return nil, err
	}
	ss, err := rbac.CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		if err := tx.Model(&models.User{}).Where("id = ?", ss.User.Id).Updates(map[string]interface{}{"timezone": tz.String()}).Error; err != nil {
			return err
		}
		return models.CreateLog(tx, uint(ss.User.Id), v2.Plugin(), ss.ClientIp, v2.UserIndexLogResponse_Item_INFO, "update timezone")
	}); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func new_log(it *models.Log) *v2.UserIndexLogResponse_Item {
	return &v2.UserIndexLogResponse_Item{
		Id:        int64(it.ID),
		Plugin:    it.Plugin,
		Ip:        it.Ip,
		Message:   it.Message,
		Level:     v2.UserIndexLogResponse_Item_Level(it.Level),
		CreatedAt: timestamppb.New(it.CreatedAt),
	}
}
func new_attachment(it *models.Attachment) *v2.UserIndexAttachmentResponse_Item {
	v := v2.UserIndexAttachmentResponse_Item{
		Id:          int64(it.ID),
		Title:       it.Title,
		Bucket:      it.Bucket,
		Object:      it.Object,
		ContentType: it.ContentType,
		Size:        uint64(it.Size),
		Public:      it.Public,
		UpdatedAt:   timestamppb.New(it.UpdatedAt),
	}
	if it.UploadedAt != nil {
		v.UploadedAt = timestamppb.New(*it.UploadedAt)
	}
	if it.ExpireAfterDays != nil {
		d := uint32(*it.ExpireAfterDays)
		v.ExpireAfterDays = &d
	}
	return &v
}
