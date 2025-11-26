package graphql

import (
	"os"
	"time"

	graphql "github.com/graph-gophers/graphql-go"

	"github.com/saturn-xiv/palm/loquat/env"
	"github.com/saturn-xiv/palm/loquat/models"
)

func (p *Query) Refresh() *RefreshResponse {
	var res RefreshResponse
	{
		var last_run_at time.Time
		if err := models.GetB(p.db, "last.run-at", &last_run_at); err == nil {
			res.last_run_at = &last_run_at
		}
	}
	return &res
}

type RefreshResponse struct {
	last_run_at *time.Time
}

func (p *RefreshResponse) Version() string {
	return env.Version()
}
func (p *RefreshResponse) Description() string {
	return env.Description()
}

func (p *RefreshResponse) Hostname() (string, error) {
	return os.Hostname()
}
func (p *RefreshResponse) CreatedAt() graphql.Time {
	return graphql.Time{Time: time.Now()}
}

func (p *RefreshResponse) LastRunAt() *graphql.Time {
	if p.last_run_at == nil {
		return nil
	}
	return &graphql.Time{Time: *p.last_run_at}
}
