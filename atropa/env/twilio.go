package env

type Twilio struct {
	AccountSid string `toml:"account-sid"`
	AuthToken  string `toml:"auth-token"`
	From       string `toml:"from"`
}
