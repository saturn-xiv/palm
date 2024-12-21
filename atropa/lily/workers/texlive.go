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

type TeXLiveWorker struct {
	client *minio.Client
}

func NewTeXLiveWorker(client *minio.Client) *TeXLiveWorker {
	return &TeXLiveWorker{client: client}
}

func (p *TeXLiveWorker) Handle(ctx context.Context, id string, content_type string, body []byte) error {
	var task v2.TeXLiveTask
	if content_type == hibiscus.APPLICATION_GRPC_PROTO {
		if err := proto.Unmarshal(body, &task); err != nil {
			return err
		}
	} else {
		return fmt.Errorf("unsupported content-type(%s)", content_type)
	}

	root, err := os.MkdirTemp("", "pandoc-consumer")
	if err != nil {
		return err
	}
	defer os.RemoveAll(root)

	entry := filepath.Join(root, "main.tex")
	{
		slog.Debug("create", slog.String("name", entry))
		if err := os.WriteFile(entry, task.Entry, 0600); err != nil {
			return err
		}
	}

	for k, v := range task.Attachments {
		f := filepath.Join(root, k)
		slog.Debug("create", slog.String("name", f))
		if err := os.WriteFile(f, v, 0600); err != nil {
			return err
		}
	}

	for range 2 {
		if err := hibiscus.Shell(root, "xelatex", "-halt-on-error", entry); err != nil {
			return err
		}
	}

	return p.client.UploadFile(ctx, filepath.Join(root, "main.pdf"), hibiscus.APPLICATION_PDF, task.Output.Bucket, task.Output.Object)
}
