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
