package graphql

import (
	"os"
	"time"

	graphql "github.com/graph-gophers/graphql-go"

	"github.com/saturn-xiv/palm/loquat/env"
)

func (p *Query) Refresh() *RefreshResponse {
	return &RefreshResponse{}
}

type RefreshResponse struct {
}

func (p *RefreshResponse) Version() string {
	return env.Version()
}

func (p *RefreshResponse) Hostname() (string, error) {
	return os.Hostname()
}
func (p *RefreshResponse) CreatedAt() graphql.Time {
	return graphql.Time{Time: time.Now()}
}
