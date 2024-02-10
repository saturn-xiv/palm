package env

import (
	twilio "github.com/twilio/twilio-go"
)

type Twilio struct {
	AccountSid string `toml:"account-sid"`
	AuthToken  string `toml:"auth-token"`
	From       string `toml:"from"`
}

func (p *Twilio) Open() *twilio.RestClient {
	return twilio.NewRestClientWithParams(twilio.ClientParams{
		Username: p.AccountSid,
		Password: p.AuthToken,
	})
}
