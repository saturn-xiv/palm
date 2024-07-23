package generate_token

import (
	"errors"
	"fmt"
	"log/slog"
	"strings"
	"time"

	"github.com/saturn-xiv/palm/atropa/env"
	"github.com/saturn-xiv/palm/atropa/env/crypto"
)

func Launch(keys_dir string, subject string, audiences []string, years int) error {
	if subject == "" {
		return errors.New("empty subject")
	}
	if len(audiences) == 0 {
		return errors.New("empty audiences")
	}
	_, _, jwt, err := crypto.Open(keys_dir)
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
	token, err := jwt.Sign(env.JWT_ISSUER, subject, audiences, map[string]interface{}{}, &nbf, &exp)
	if err != nil {
		return err
	}
	fmt.Println(token)
	return nil
}
