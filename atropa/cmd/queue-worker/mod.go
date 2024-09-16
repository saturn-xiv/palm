package queue_worker

const (
	EmailSendTask = "email-send"
	SmsSendTask   = "sms-send"
	PandocTask    = "pandoc"
	TexliveTask   = "texlive"
)

type Worker interface {
	Execute(config_file string, consumer_name string, queue_name string)
}
