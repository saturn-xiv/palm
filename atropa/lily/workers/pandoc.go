package workers

import (
	"context"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"

	"google.golang.org/protobuf/proto"

	"github.com/saturn-xiv/palm/atropa/env/minio"
	"github.com/saturn-xiv/palm/atropa/hibiscus"
	v2 "github.com/saturn-xiv/palm/atropa/lily/services/v2"
)

type PandocWorker struct {
	client *minio.Client
}

func NewPandocWorker(client *minio.Client) *PandocWorker {
	return &PandocWorker{client: client}
}

func (p *PandocWorker) Handle(ctx context.Context, id string, content_type string, body []byte) error {
	var task v2.PandocTask
	if content_type == hibiscus.APPLICATION_GRPC_PROTO {
		if err := proto.Unmarshal(body, &task); err != nil {
			return err
		}
	} else {
		return fmt.Errorf("unsupported content-type(%s)", content_type)
	}

	if task.Input.Format == v2.PandocTask_Markdown && (task.Output.Format == v2.PandocTask_Docx) {
		return p.execute(ctx, task.Input.Payload, "markdown", "docx", task.Output.Bucket, task.Output.Object, hibiscus.APPLICATION_DOCX)
	}
	if task.Input.Format == v2.PandocTask_Markdown && task.Output.Format == v2.PandocTask_Pdf {
		return p.execute(ctx, task.Input.Payload, "markdown", "pdf", task.Output.Bucket, task.Output.Object, hibiscus.APPLICATION_PDF)
	}
	if task.Input.Format == v2.PandocTask_Markdown && task.Output.Format == v2.PandocTask_Plain {
		return p.execute(ctx, task.Input.Payload, "markdown", "plain", task.Output.Bucket, task.Output.Object, hibiscus.TEXT_PLAIN_UTF8)
	}
	return nil
}

func (p *PandocWorker) execute(ctx context.Context, payload []byte, input_format string, output_format string, bucket string, object string, content_type string) error {
	root, err := os.MkdirTemp("", "pandoc-consumer")
	if err != nil {
		return err
	}
	defer os.RemoveAll(root)

	in := filepath.Join(root, "i")

	{
		slog.Debug("create", slog.String("name", in))
		if err := os.WriteFile(in, payload, 0600); err != nil {
			return err
		}
	}

	out := filepath.Join(root, "o")
	if err := hibiscus.Shell(root, "pandoc", "-f", input_format, "-t", output_format, "-o", out, in); err != nil {
		return err
	}

	return p.client.UploadFile(ctx, out, content_type, bucket, object)
}
