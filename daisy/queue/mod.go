package queue

import "context"

type Consumer interface {
	Name() string
	Execute(ctx context.Context, id string, content_type string, body []byte) error
}
