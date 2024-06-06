package web

import "github.com/saturn-xiv/palm/atropa/env"

type Config struct {
	TwilioCallbackToken string       `toml:"twilio-callback-token"`
	Database            env.Database `toml:"database"`
}
