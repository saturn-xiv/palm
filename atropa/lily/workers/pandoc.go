package workers

import (
	"fmt"

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

func (p *PandocWorker) Handle(id string, content_type string, body []byte) error {
	var task v2.TeXLiveTask
	if content_type == hibiscus.APPLICATION_GRPC_PROTO {
		if err := proto.Unmarshal(body, &task); err != nil {
			return err
		}
	} else {
		return fmt.Errorf("unsupported content-type(%s)", content_type)
	}
	// TODO
	return nil
}
