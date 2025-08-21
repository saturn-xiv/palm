package user

import (
	"github.com/BurntSushi/toml"

	"github.com/saturn-xiv/palm/jasmine/env"
	"github.com/saturn-xiv/palm/jasmine/env/redis"
	"github.com/saturn-xiv/palm/jasmine/services/portal/models"
)

type Config struct {
	SecretsStore string        `toml:"secrets-store"`
	Redis        redis.Cluster `toml:"redis"`
	Database     env.Database  `toml:"database"`
}

func ListUser(config_file string) ([]models.EmailUser, error) {
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return nil, err
	}

	db, err := config.Database.Open()
	if err != nil {
		return nil, err
	}
	email_users, err := listEmailUser(db)
	if err != nil {
		return nil, err
	}

	return email_users, err
}
