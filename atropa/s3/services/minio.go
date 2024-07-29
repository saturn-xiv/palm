package services

import (
	"bytes"
	"context"
	"encoding/base32"
	"encoding/gob"
	"fmt"
	"log/slog"
	"strings"

	"github.com/saturn-xiv/palm/atropa/env/minio"
	pb "github.com/saturn-xiv/palm/atropa/s3/services/v2"
)

func NewS3Service(cluster *minio.Cluster) *S3Service {
	return &S3Service{cluster: cluster}
}

type S3Service struct {
	pb.UnimplementedS3Server

	cluster *minio.Cluster
}

func (p *S3Service) CreateBucket(ctx context.Context, req *pb.CreateBucketRequest) (*pb.CreateBucketResponse, error) {
	it := bucket{
		namespace:       p.cluster.Namespace,
		name:            req.Name,
		public:          req.Public,
		expiration_days: int(req.ExpirationDays),
	}
	name, err := it.code()
	if err != nil {
		return nil, err
	}
	for _, client := range p.cluster.Clients {
		if !client.Readonly {
			if err := client.CreateBucket(ctx, name, req.Public, int(req.ExpirationDays)); err != nil {
				return nil, err
			}
		}
	}
	return &pb.CreateBucketResponse{Name: name}, nil
}

// https://min.io/docs/minio/linux/integrations/presigned-put-upload-via-browser.html
func (p *S3Service) Upload(ctx context.Context, req *pb.UploadRequest) (*pb.UploadResponse, error) {
	client := p.cluster.Chooser.Pick()
	url, object, err := client.Upload(ctx, req.Bucket, req.Title, req.Ttl.AsDuration())
	if err != nil {
		return nil, err
	}
	return &pb.UploadResponse{Url: url.String(), Object: object}, nil
}

func (p *S3Service) PermanentUrl(ctx context.Context, req *pb.PermanentUrlRequest) (*pb.UrlResponse, error) {
	for _, client := range p.cluster.Clients {
		if _, err := client.StatObject(ctx, req.Bucket, req.Object); err == nil {
			slog.Debug("found", slog.String("node", client.String()), slog.String("bucket", req.Bucket), slog.String("object", req.Object))
			url, err := client.PermanentUrl(ctx, req.Bucket, req.Object, req.Title, req.ContentType)
			if err != nil {
				return nil, err
			}
			return &pb.UrlResponse{Url: url.String()}, nil
		}
	}
	return nil, fmt.Errorf("couldn't found %s/%s on any nodes", req.Bucket, req.Object)
}

func (p *S3Service) PresignedUrl(ctx context.Context, req *pb.PresignedUrlRequest) (*pb.UrlResponse, error) {
	for _, client := range p.cluster.Clients {
		if _, err := client.StatObject(ctx, req.Bucket, req.Object); err == nil {
			slog.Debug("found", slog.String("node", client.String()), slog.String("bucket", req.Bucket), slog.String("object", req.Object))

			url, err := client.PresignedUrl(ctx, req.Bucket, req.Object, req.Title, req.ContentType, req.Ttl.AsDuration())
			if err != nil {
				return nil, err
			}
			return &pb.UrlResponse{Url: url.String()}, nil
		}
	}
	return nil, fmt.Errorf("couldn't found %s/%s on any nodes", req.Bucket, req.Object)
}

type bucket struct {
	namespace       string
	name            string
	public          bool
	expiration_days int
}

func (p *bucket) code() (string, error) {
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	if err := enc.Encode(p); err != nil {
		return "", err
	}
	code := base32.HexEncoding.WithPadding(base32.NoPadding).EncodeToString(buf.Bytes())
	return strings.ToLower(code), nil
}
