package s3

import (
	"bytes"
	"context"
	_ "embed"
	"fmt"
	"log/slog"
	"reflect"
	"text/template"

	"github.com/casbin/casbin/v3"
	"github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/lifecycle"
	"google.golang.org/protobuf/types/known/emptypb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/crypto"
	"github.com/saturn-xiv/palm/daisy/models"
	"github.com/saturn-xiv/palm/daisy/rbac"
	rbac_v2 "github.com/saturn-xiv/palm/daisy/rbac/v2"
	v2 "github.com/saturn-xiv/palm/daisy/s3/v2"
)

//go:embed anonymous-read.json
var anonymous_read_json string

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
		user, _, err := rbac.CurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err = rbac.IsAdministrator(p.enforcer, user); err != nil {
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
		user, _, err := rbac.CurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err = rbac.IsAdministrator(p.enforcer, user); err != nil {
			return nil, err
		}
	}

	slog.Info("create bucket", "name", req.Name)
	err := p.client.MakeBucket(ctx, req.Name, minio.MakeBucketOptions{})
	if err != nil {
		return nil, err
	}
	if req.Public {
		slog.Info("set anonymous read access", "bucket", req.Name)
		tpl, err := template.New("anonymous-read").Parse(anonymous_read_json)
		if err != nil {
			return nil, err
		}
		var buf bytes.Buffer
		if err = tpl.Execute(&buf, map[string]interface{}{"bucket": req.Name}); err != nil {
			return nil, err
		}
		policy := buf.String()
		slog.Debug("set policy", "rule", policy)
		if err = p.client.SetBucketPolicy(ctx, req.Name, policy); err != nil {
			return nil, err
		}
	}
	if req.ExpireAfterDays != nil {
		config := lifecycle.NewConfiguration()
		config.Rules = []lifecycle.Rule{
			{
				ID:     fmt.Sprintf("expire-after-%d-days", *req.ExpireAfterDays),
				Status: "Enabled",
				Expiration: lifecycle.Expiration{
					Days: lifecycle.ExpirationDays(*req.ExpireAfterDays),
				},
			},
		}

		if err = p.client.SetBucketLifecycle(ctx, req.Name, config); err != nil {
			return nil, err
		}
	}
	return &emptypb.Empty{}, nil
}

func (p *Server) BucketExists(ctx context.Context, req *v2.BucketExistsRequest) (*v2.BucketExistsResponse, error) {
	{
		user, _, err := rbac.CurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err = rbac.IsAdministrator(p.enforcer, user); err != nil {
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
	user, _, err := rbac.CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err = can_remove_object(p.enforcer, user, req.Bucket, req.Object); err != nil {
		return nil, err
	}
	if err = p.client.RemoveObject(ctx, req.Bucket, req.Object, minio.RemoveObjectOptions{ForceDelete: true}); err != nil {
		return nil, err
	}

	return &emptypb.Empty{}, nil
}
func (p *Server) PutObject(ctx context.Context, req *v2.PutObjectRequest) (*v2.PutObjectResponse, error) {
	user, _, err := rbac.CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err = can_upload_object(p.enforcer, user, req.Bucket); err != nil {
		return nil, err
	}
	url, err := p.client.PresignedPutObject(ctx, req.Bucket, req.Object, req.Ttl.AsDuration())
	if err != nil {
		return nil, err
	}

	return &v2.PutObjectResponse{
		Url: url.String(),
	}, nil
}

func can_remove_object(enforcer *casbin.Enforcer, user *models.User, bucket string, object string) error {
	s3_o := s3_object{bucket: bucket, object: object}

	if err := rbac.Can(enforcer, user, rbac_v2.ActionDelete(), &rbac_v2.Object{Type: reflect.TypeOf((*s3_object)(nil)).Elem().Name(), By: &rbac_v2.Object_Code{Code: s3_o.code()}}); err == nil {
		return nil
	}
	return rbac.Can(enforcer, user, rbac_v2.ActionManage(), &rbac_v2.Object{Type: reflect.TypeOf((*s3_bucket)(nil)).Elem().Name(), By: &rbac_v2.Object_Code{Code: bucket}})
}

func can_upload_object(enforcer *casbin.Enforcer, user *models.User, bucket string) error {
	return rbac.Can(enforcer, user, rbac_v2.ActionAppend(), &rbac_v2.Object{Type: reflect.TypeOf((*s3_bucket)(nil)).Elem().Name(), By: &rbac_v2.Object_Code{Code: bucket}})
}

type s3_bucket struct{}
type s3_object struct {
	bucket string
	object string
}

func (p *s3_object) code() string {
	return fmt.Sprintf("%s.%s", p.bucket, p.object)
}
