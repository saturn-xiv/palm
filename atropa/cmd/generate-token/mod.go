package generate_token

import (
	"errors"
	"fmt"
	"log/slog"
	"strings"
	"time"

	"github.com/BurntSushi/toml"

	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"github.com/saturn-xiv/palm/atropa/hibiscus"
)

type Config struct {
	KeysDir string `toml:"keys-dir"`
}

func Launch(config_file string, subject string, audiences []string, years int) error {
	if subject == "" {
		return errors.New("empty subject")
	}
	if len(audiences) == 0 {
		return errors.New("empty audiences")
	}
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	_, _, jwt, err := crypto.Open(config.KeysDir)
	if err != nil {
		return err
	}
	now := time.Now()
	nbf := now.Add(time.Minute * 5)
	exp := now.AddDate(years, 0, 0)
	slog.Info("generate token",
		slog.String("subject", subject),
		slog.String("audiences", strings.Join(audiences, ",")),
		slog.Time("not-before", nbf),
		slog.Time("expires-at", exp),
	)
	token, err := jwt.Sign(hibiscus.JWT_ISSUER, subject, audiences, map[string]interface{}{}, &nbf, &exp)
	if err != nil {
		return err
	}
	fmt.Println(token)
	return nil
}