package queue

type Consumer interface {
	Name() string
	Execute(id string, content_type string, body []byte) error
}
