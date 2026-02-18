package s3

import (
	"context"

	"github.com/minio/minio-go/v7"
	"google.golang.org/protobuf/types/known/emptypb"

	v2 "github.com/saturn-xiv/palm/daisy/s3/v2"
)

type Server struct {
	v2.UnimplementedS3Server

	client *minio.Client
}

func NewServer(client *minio.Client) *Server {
	return &Server{client: client}
}

func (p *Server) ListBucket(ctx context.Context, req *emptypb.Empty) (*v2.ListBucketResponse, error) {
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

	if err := req.Execute(ctx, p.client); err != nil {
		return nil, err
	}

	return &emptypb.Empty{}, nil
}

func (p *Server) BucketExists(ctx context.Context, req *v2.BucketExistsRequest) (*v2.BucketExistsResponse, error) {
	ok, err := p.client.BucketExists(ctx, req.Name)
	if err != nil {
		return nil, err
	}
	return &v2.BucketExistsResponse{
		Exists: ok,
	}, nil
}

func (p *Server) RemoveObject(ctx context.Context, req *v2.RemoveObjectRequest) (*emptypb.Empty, error) {
	if err := req.Execute(ctx, p.client); err != nil {
		return nil, err
	}

	return &emptypb.Empty{}, nil
}
func (p *Server) PutObject(ctx context.Context, req *v2.PresignedPutObjectRequest) (*v2.PresignedPutObjectResponse, error) {
	url, err := req.Execute(ctx, p.client)
	if err != nil {
		return nil, err
	}

	return &v2.PresignedPutObjectResponse{
		Url: url.String(),
	}, nil
}

func (p *Server) PresignedGetObject(ctx context.Context, req *v2.PresignedGetObjectRequest) (*v2.PresignedGetObjectResponse, error) {
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
