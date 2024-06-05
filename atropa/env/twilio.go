package env

import (
	"github.com/twilio/twilio-go"

	"github.com/saturn-xiv/palm/atropa/workers"
)

type Twilio struct {
	AccountSid string `toml:"account-sid"`
	AuthToken  string `toml:"auth-token"`
	From       string `toml:"from"`
}

func (p *Twilio) Open() *workers.SendSmsWorker {
	client := twilio.NewRestClientWithParams(twilio.ClientParams{
		Username: p.AccountSid,
		Password: p.AuthToken,
	})
	return workers.NewSendSmsWorker(client, p.From)
}
