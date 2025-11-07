package graphql

import "github.com/saturn-xiv/palm/daisy/env"

type Query struct{}

func (p *Query) Version() string { return env.Version() }
