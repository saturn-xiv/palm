package s3

import (
	"context"
	"fmt"
	"reflect"

	"github.com/casbin/casbin/v3"
	"github.com/minio/minio-go/v7"
	"google.golang.org/protobuf/types/known/emptypb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/crypto"
	portal_v2 "github.com/saturn-xiv/palm/daisy/portal/v2"
	"github.com/saturn-xiv/palm/daisy/rbac"
	rbac_v2 "github.com/saturn-xiv/palm/daisy/rbac/v2"
	v2 "github.com/saturn-xiv/palm/daisy/s3/v2"
)

type Server struct {
	v2.UnimplementedS3Server

	client   *minio.Client
	enforcer *casbin.Enforcer
	db       *gorm.DB
	jwt      *crypto.Jwt
}

func NewServer(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer, client *minio.Client) *Server {
	return &Server{db: db, jwt: jwt, enforcer: enforcer, client: client}
}

func (p *Server) ListBucket(ctx context.Context, req *emptypb.Empty) (*v2.ListBucketResponse, error) {
	{
		ss, err := rbac.CurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err = ss.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	buckets, err := p.client.ListBuckets(ctx)
	if err != nil {
		return nil, err
	}
	var res v2.ListBucketResponse
	for _, it := range buckets {
		res.Items = append(res.Items, &v2.ListBucketResponse_Item{Name: it.Name})
	}
	return &res, nil
}

func (p *Server) MakeBucket(ctx context.Context, req *v2.MakeBucketRequest) (*emptypb.Empty, error) {
	{
		ss, err := rbac.CurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err = ss.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}

	if err := req.Execute(ctx, p.client); err != nil {
		return nil, err
	}

	return &emptypb.Empty{}, nil
}

func (p *Server) BucketExists(ctx context.Context, req *v2.BucketExistsRequest) (*v2.BucketExistsResponse, error) {
	{
		ss, err := rbac.CurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err = ss.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	ok, err := p.client.BucketExists(ctx, req.Name)
	if err != nil {
		return nil, err
	}
	return &v2.BucketExistsResponse{
		Exists: ok,
	}, nil
}

func (p *Server) RemoveObject(ctx context.Context, req *v2.RemoveObjectRequest) (*emptypb.Empty, error) {
	ss, err := rbac.CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err = can_remove_object(p.enforcer, ss, req.Bucket, req.Object); err != nil {
		return nil, err
	}
	if err = req.Execute(ctx, p.client); err != nil {
		return nil, err
	}

	return &emptypb.Empty{}, nil
}
func (p *Server) PutObject(ctx context.Context, req *v2.PutObjectRequest) (*v2.PutObjectResponse, error) {
	ss, err := rbac.CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err = can_upload_object(p.enforcer, ss, req.Bucket); err != nil {
		return nil, err
	}
	url, err := req.Execute(ctx, p.client)
	if err != nil {
		return nil, err
	}

	return &v2.PutObjectResponse{
		Url: url.String(),
	}, nil
}

func (p *Server) PresignedGetObject(ctx context.Context, req *v2.PresignedGetObjectRequest) (*v2.PresignedGetObjectResponse, error) {
	ss, err := rbac.CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err = can_show_object(p.enforcer, ss, req.Bucket, req.Object); err != nil {
		return nil, err
	}
	url, err := req.Execute(ctx, p.client)
	if err != nil {
		return nil, err
	}
	return &v2.PresignedGetObjectResponse{Url: url.String()}, nil
}

func (p *Server) GetObject(ctx context.Context, req *v2.GetObjectRequest) (*v2.GetObjectResponse, error) {
	url := req.Execute(p.client)
	return &v2.GetObjectResponse{Url: url.String()}, nil
}

func can_remove_object(enforcer *casbin.Enforcer, ss *portal_v2.Session, bucket string, object string) error {
	s3_o := s3_object{bucket: bucket, object: object}

	if err := ss.Can(enforcer, rbac_v2.ActionDelete(), &rbac_v2.Object{Type: reflect.TypeOf((*s3_object)(nil)).Elem().Name(), By: &rbac_v2.Object_Code{Code: s3_o.code()}}); err == nil {
		return nil
	}
	return ss.Can(enforcer, rbac_v2.ActionManage(), &rbac_v2.Object{Type: reflect.TypeOf((*s3_bucket)(nil)).Elem().Name(), By: &rbac_v2.Object_Code{Code: bucket}})
}

func can_show_object(enforcer *casbin.Enforcer, ss *portal_v2.Session, bucket string, object string) error {
	s3_o := s3_object{bucket: bucket, object: object}

	if err := ss.Can(enforcer, rbac_v2.ActionInquiry(), &rbac_v2.Object{Type: reflect.TypeOf((*s3_object)(nil)).Elem().Name(), By: &rbac_v2.Object_Code{Code: s3_o.code()}}); err == nil {
		return nil
	}
	return ss.Can(enforcer, rbac_v2.ActionManage(), &rbac_v2.Object{Type: reflect.TypeOf((*s3_bucket)(nil)).Elem().Name(), By: &rbac_v2.Object_Code{Code: bucket}})
}

func can_upload_object(enforcer *casbin.Enforcer, ss *portal_v2.Session, bucket string) error {
	return ss.Can(enforcer, rbac_v2.ActionAppend(), &rbac_v2.Object{Type: reflect.TypeOf((*s3_bucket)(nil)).Elem().Name(), By: &rbac_v2.Object_Code{Code: bucket}})
}

type s3_bucket struct{}
type s3_object struct {
	bucket string
	object string
}

func (p *s3_object) code() string {
	return fmt.Sprintf("%s.%s", p.bucket, p.object)
}
