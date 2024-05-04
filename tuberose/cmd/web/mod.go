package web

import "github.com/saturn-xiv/palm/tuberose/env"

type Config struct {
	Database            env.Database `toml:"database"`
	TwilioCallbackToken string       `toml:"twilio-callback-token"`
}
