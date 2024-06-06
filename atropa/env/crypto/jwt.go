package crypto

import (
	"log/slog"
	"time"

	"github.com/google/uuid"
	"github.com/tink-crypto/tink-go/v2/jwt"
	"github.com/tink-crypto/tink-go/v2/tink"
)

type Jwt struct {
	mac jwt.MAC
}

func (p *Jwt) Verify(token string, issuer string, audience string) (string, string, map[string]interface{}, error) {
	validator, err := jwt.NewValidator(&jwt.ValidatorOpts{
		ExpectedAudience: &audience,
		ExpectedIssuer:   &issuer,
	})
	if err != nil {
		return "", "", nil, err
	}
	verified_jwt, err := p.mac.VerifyMACAndDecode(token, validator)
	if err != nil {
		return "", "", nil, err
	}

	jwt_id, err := verified_jwt.JWTID()
	if err != nil {
		return "", "", nil, err
	}
	subject, err := verified_jwt.Subject()
	if err != nil {
		return "", "", nil, err
	}

	claims := make(map[string]interface{})

	for _, k := range verified_jwt.CustomClaimNames() {
		{
			v, err := verified_jwt.StringClaim(k)
			if err == nil {
				claims[k] = v
				continue
			}
		}
		{
			v, err := verified_jwt.NumberClaim(k)
			if err == nil {
				claims[k] = v
				continue
			}
		}
		slog.Error("unknown custom claim type", slog.String("name", k))
	}

	return jwt_id, subject, claims, nil
}

func (p *Jwt) Sign(issuer string, subject string, audiences []string, claims map[string]interface{}, not_before *time.Time, expires_at *time.Time) (string, error) {
	id := uuid.NewString()
	now := time.Now()

	raw, err := jwt.NewRawJWT(&jwt.RawJWTOptions{
		JWTID:        &id,
		Subject:      &subject,
		CustomClaims: claims,
		ExpiresAt:    expires_at,
		NotBefore:    not_before,
		Issuer:       &issuer,
		IssuedAt:     &now,
		Audiences:    audiences,
	})
	if err != nil {
		return "", err
	}

	return p.mac.ComputeMACAndEncode(raw)
}

func NewJwt(file string, secret tink.AEAD) (*Jwt, error) {
	handle, err := Restore(file, secret)
	if err != nil {
		return nil, err
	}
	mac, err := jwt.NewMAC(handle)
	if err != nil {
		return nil, err
	}
	return &Jwt{
		mac: mac,
	}, nil
}
