package s3

import (
	"context"

	"github.com/casbin/casbin/v2"
	"github.com/minio/minio-go/v7"
	"google.golang.org/protobuf/types/known/emptypb"
	"google.golang.org/protobuf/types/known/timestamppb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/jasmine/env/crypto"
	"github.com/saturn-xiv/palm/jasmine/services/portal/models"
	s3_v2 "github.com/saturn-xiv/palm/jasmine/services/s3/v2"
	"github.com/saturn-xiv/palm/jasmine/web"
)

type S3Server struct {
	s3_v2.UnimplementedS3Server

	client   *minio.Client
	jwt      *crypto.Jwt
	db       *gorm.DB
	enforcer *casbin.Enforcer
}

func (p *S3Server) ListBuckets(ctx context.Context, req *emptypb.Empty) (*s3_v2.ListBucketsResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	buckets, err := p.client.ListBuckets(ctx)
	if err != nil {
		return nil, err
	}
	var reply s3_v2.ListBucketsResponse
	for _, it := range buckets {
		reply.Items = append(reply.Items, &s3_v2.ListBucketsResponse_Item{
			Name:         it.Name,
			Region:       it.BucketRegion,
			CreationDate: timestamppb.New(it.CreationDate),
		})
	}
	return &reply, nil
}
func (p *S3Server) BucketExists(ctx context.Context, req *s3_v2.BucketRequest) (*s3_v2.BucketExistsResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	ok, err := p.client.BucketExists(ctx, req.Bucket)
	if err != nil {
		return nil, err
	}
	var reply s3_v2.BucketExistsResponse
	reply.Exists = ok
	return &reply, nil
}
func (p *S3Server) GetBucketEncryption(ctx context.Context, req *s3_v2.BucketRequest) (*s3_v2.GetBucketEncryptionResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}
func (p *S3Server) GetBucketPolicy(ctx context.Context, req *s3_v2.BucketRequest) (*s3_v2.GetBucketPolicyResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}
func (p *S3Server) GetBucketTags(ctx context.Context, req *s3_v2.BucketRequest) (*s3_v2.GetBucketTagsResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}
func (p *S3Server) GetBucketLifecycle(ctx context.Context, req *s3_v2.BucketRequest) (*s3_v2.GetBucketLifecycleResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}
func (p *S3Server) MakeBucket(ctx context.Context, req *s3_v2.MakeBucketRequest) (*emptypb.Empty, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}
func (p *S3Server) RemoveBucket(ctx context.Context, req *s3_v2.BucketRequest) (*emptypb.Empty, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}
func (p *S3Server) SetBucketTags(ctx context.Context, req *s3_v2.SetBucketTagsRequest) (*emptypb.Empty, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}
func (p *S3Server) SetBucketPolicy(ctx context.Context, req *s3_v2.SetBucketPolicyRequest) (*emptypb.Empty, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}
func (p *S3Server) SetBucketLifecycle(ctx context.Context, req *s3_v2.SetBucketLifecycleRequest) (*emptypb.Empty, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}
func (p *S3Server) SetBucketEncryption(ctx context.Context, req *s3_v2.SetBucketEncryptionRequest) (*emptypb.Empty, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}
func (p *S3Server) DeleteBucketEncryption(ctx context.Context, req *s3_v2.BucketRequest) (*emptypb.Empty, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}
func (p *S3Server) DeleteBucketPolicy(ctx context.Context, req *s3_v2.BucketRequest) (*emptypb.Empty, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}
func (p *S3Server) DeleteBucketTags(ctx context.Context, req *s3_v2.BucketRequest) (*emptypb.Empty, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}
func (p *S3Server) DeleteBucketLifecycle(ctx context.Context, req *s3_v2.BucketRequest) (*emptypb.Empty, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}

func (p *S3Server) GetPresignedPostFormData(ctx context.Context, req *s3_v2.GetPresignedPostFormDataRequest) (*s3_v2.GetPresignedPostFormDataResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}
func (p *S3Server) GetPresignedObjectUrl(ctx context.Context, req *s3_v2.GetPresignedObjectUrlRequest) (*s3_v2.GetPresignedObjectUrlResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}
func (p *S3Server) ListObjects(ctx context.Context, req *s3_v2.BucketRequest) (*s3_v2.ListObjectsResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}
func (p *S3Server) GetObjectTags(ctx context.Context, req *s3_v2.ObjectRequest) (*s3_v2.GetObjectTagsResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}
func (p *S3Server) GetObjectRetention(ctx context.Context, req *s3_v2.ObjectRequest) (*s3_v2.GetObjectRetentionResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}
func (p *S3Server) StatObject(ctx context.Context, req *s3_v2.ObjectRequest) (*s3_v2.StatObjectResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}
func (p *S3Server) SetObjectTags(ctx context.Context, req *s3_v2.SetObjectTagsRequest) (*emptypb.Empty, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}
func (p *S3Server) DeleteObjects(ctx context.Context, req *s3_v2.DeleteObjectsRequest) (*emptypb.Empty, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}

func NewS3Server(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer, client *minio.Client) *S3Server {
	return &S3Server{db: db, enforcer: enforcer, jwt: jwt, client: client}
}
