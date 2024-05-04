package tex_to_pdf

import (
	"github.com/saturn-xiv/palm/lily/env"
	"github.com/saturn-xiv/palm/lily/env/jasmine"
)

type Config struct {
	RabbitMq env.RabbitMq   `toml:"rabbitmq"`
	Jasmine  jasmine.Config `toml:"jasmine"`
}
