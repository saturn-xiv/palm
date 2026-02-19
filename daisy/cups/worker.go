package cups

import (
	"context"

	"google.golang.org/protobuf/proto"

	v2 "github.com/saturn-xiv/palm/daisy/cups/v2"
)

type CupsProtobufConsumer struct {
}

func NewCupsProtobufConsumer() *CupsProtobufConsumer {
	return &CupsProtobufConsumer{}
}

func (p *CupsProtobufConsumer) Name() string {
	return "cups.protobuf"
}

func (p *CupsProtobufConsumer) Execute(ctx context.Context, id string, content_type string, body []byte) error {
	var task v2.Task
	if err := proto.Unmarshal(body, &task); err != nil {
		return err
	}
	return task.Execute()
}
