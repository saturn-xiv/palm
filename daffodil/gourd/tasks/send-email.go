package tasks

type SmtpConsumer struct{}

func (p *SmtpConsumer) Handle(id string, content_type string, body []byte) error {
	// TODO
	return nil
}
