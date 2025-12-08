package tex

import (
	"context"
	"fmt"
	"log/slog"

	"github.com/minio/minio-go/v7"
	"google.golang.org/protobuf/proto"

	v2 "github.com/saturn-xiv/palm/daisy/tex/v2"
)

type TexProtobufConsumer struct {
	client *minio.Client
}

func NewTexProtobufConsumer(client *minio.Client) *TexProtobufConsumer {
	return &TexProtobufConsumer{client: client}
}

func (p *TexProtobufConsumer) Name() string {
	return "tex.protobuf"
}
func (p *TexProtobufConsumer) Execute(ctx context.Context, id string, content_type string, body []byte) error {
	var task v2.Task
	if err := proto.Unmarshal(body, &task); err != nil {
		return err
	}
	if err := task.Execute(func(file string, bucket string, object string) error {
		ok, err := p.client.BucketExists(ctx, bucket)
		if err != nil {
			return err
		}
		if !ok {
			return fmt.Errorf("bucket %s isn't exists", bucket)
		}
		res, err := p.client.FPutObject(ctx, bucket, object, file, minio.PutObjectOptions{ContentType: "application/pdf"})
		if err != nil {
			return err
		}
		slog.Info("uploaded", "location", res.Location, "size", res.Size)
		return nil
	}); err != nil {
		return err
	}
	return nil
}
