package portal

import (
	"bytes"
	"context"
	_ "embed"
	"encoding/base32"
	"encoding/gob"
	"fmt"
	"log/slog"
	"net/url"
	"path/filepath"
	"strings"
	"text/template"
	"time"

	"github.com/casbin/casbin/v2"
	"github.com/google/uuid"
	"github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/lifecycle"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/jasmine/env/crypto"
	"github.com/saturn-xiv/palm/jasmine/services/portal/models"
	v2 "github.com/saturn-xiv/palm/jasmine/services/portal/v2"
	"github.com/saturn-xiv/palm/jasmine/web"
)

//go:embed public-policy.json
var gl_minio_public_policy string

var gl_minio_response_content_disposition = "response-content-disposition"
var gl_minio_response_content_type = "response-content-type"

type AttachmentServer struct {
	v2.UnimplementedAttachmentServer

	minio     *minio.Client
	jwt       *crypto.Jwt
	db        *gorm.DB
	enforcer  *casbin.Enforcer
	namespace string
}

func (p *AttachmentServer) Index(ctx context.Context, req *v2.Page) (*v2.AttachmentIndexResponse, error) {
	// TOTO
	return &v2.AttachmentIndexResponse{}, nil
}
func (p *AttachmentServer) Upload(ctx context.Context, req *v2.AttachmentUploadRequest) (*v2.AttachmentUploadResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, v2.ErrorUserIsNotSignedIn
	}
	bucket := s3_bucket{
		Namespace:      p.namespace,
		Public:         req.Bucket.Public,
		ExpirationDays: req.Bucket.ExpirationDays,
	}
	bucket_name, err := bucket.toString()
	if err != nil {
		return nil, err
	}
	{
		exists, err := p.minio.BucketExists(ctx, bucket_name)
		if err != nil {
			return nil, err
		}
		if !exists {
			slog.Warn("create bucket", slog.String("name", bucket_name))
			if err := p.minio.MakeBucket(ctx, bucket_name, minio.MakeBucketOptions{}); err != nil {
				return nil, err
			}
			// https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_version.html
			// mc set my-minio https://base-url access-key secret-key
			// mc ls my-minio
			// mc mb my-minio/downloads
			// mc anonymous set download my-minio/downloads
			// mc anonymous get-json my-minio/downloads > /tmp/policy.json
			// mc rb my-minio/downloads
			if bucket.Public {
				slog.Warn("set bucket public policy", slog.String("name", bucket_name))
				tpl, err := template.New("").Parse(gl_minio_public_policy)
				if err != nil {
					return nil, err
				}
				var buf bytes.Buffer
				if err = tpl.Execute(&buf, web.H{"name": bucket_name}); err != nil {
					return nil, err
				}
				if err = p.minio.SetBucketPolicy(ctx, bucket_name, buf.String()); err != nil {
					return nil, err
				}
			}
			if bucket.ExpirationDays != nil && *bucket.ExpirationDays > 0 {
				slog.Warn("set bucket expiration", slog.String("name", bucket_name), slog.Int("days", int(*bucket.ExpirationDays)))
				config := lifecycle.NewConfiguration()
				config.Rules = append(config.Rules, lifecycle.Rule{
					ID:     fmt.Sprintf("expires-in-%d-days", *bucket.ExpirationDays),
					Status: "Enabled",
					Expiration: lifecycle.Expiration{
						Days: lifecycle.ExpirationDays(*bucket.ExpirationDays),
					},
				})
				if err = p.minio.SetBucketLifecycle(ctx, bucket_name, config); err != nil {
					return nil, err
				}
			}
		}
	}

	object := uuid.New().String()
	ext := filepath.Ext(req.FileName)
	if ext != "" {
		object = fmt.Sprintf("%s.%s", object, ext)
	}
	url, err := p.minio.PresignedPutObject(ctx, bucket_name, object, req.Expires.AsDuration())
	if err != nil {
		return nil, err
	}
	return &v2.AttachmentUploadResponse{Bucket: bucket_name, Object: object, Url: url.String()}, nil
}

func (p *AttachmentServer) Show(ctx context.Context, req *v2.AttachmentShowRequest) (*v2.AttachmentShowResponse, error) {

	var res v2.AttachmentShowResponse
	{
		bucket, err := new_s3_bucket(req.Bucket)
		if err != nil {
			return nil, err
		}
		if bucket.Namespace != p.namespace {
			return nil, v2.ErrorNotFound
		}
		if bucket.Public {
			res.Url = fmt.Sprintf("%s/%s/%s", p.minio.EndpointURL().String(), req.Bucket, req.Object)
			return &res, nil
		}
	}
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, v2.ErrorUserIsNotSignedIn
	}
	// TODO CHECK permission
	params := make(url.Values)
	params.Set(gl_minio_response_content_type, req.ContentType)
	if req.Inline {
		params.Set(gl_minio_response_content_disposition, web.ContentDispositionInline)
	} else {
		params.Set(gl_minio_response_content_disposition, web.ContentDispositionAttachment(req.FileName))
	}
	expires := time.Duration(24*7) * time.Hour
	if req.Expires != nil {
		expires = req.Expires.AsDuration()
	}

	url, err := p.minio.PresignedGetObject(context.Background(), req.Bucket, req.Object, expires, params)
	if err != nil {
		return nil, err
	}
	res.Url = url.String()
	return &res, nil
}

func NewAttachmentServer(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer, minio *minio.Client, namespace string) *AttachmentServer {
	return &AttachmentServer{db: db, enforcer: enforcer, jwt: jwt, minio: minio, namespace: namespace}
}

// https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html#general-purpose-bucket-names
type s3_bucket struct {
	Namespace      string
	Public         bool
	ExpirationDays *uint32
}

func (p *s3_bucket) toString() (string, error) {
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	if err := enc.Encode(p); err != nil {
		return "", err
	}
	return strings.ToLower(base32.StdEncoding.WithPadding(base32.NoPadding).EncodeToString(buf.Bytes())), nil
}

func new_s3_bucket(s string) (*s3_bucket, error) {
	tmp, err := base32.StdEncoding.WithPadding(base32.NoPadding).DecodeString(strings.ToUpper(s))
	if err != nil {
		return nil, err
	}

	buf := bytes.NewBuffer(tmp)
	dec := gob.NewDecoder(buf)
	var it s3_bucket
	if err = dec.Decode(&it); err != nil {
		return nil, err
	}
	return &it, nil
}
