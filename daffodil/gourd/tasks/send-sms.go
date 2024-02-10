package tasks

type TwilioSmsConsumer struct{}

func (p *TwilioSmsConsumer) Handle(id string, content_type string, body []byte) error {
	// TODO
	return nil
}
