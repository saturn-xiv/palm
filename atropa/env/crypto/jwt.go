package crypto

import (
	"time"

	"github.com/google/uuid"
	"github.com/tink-crypto/tink-go/v2/jwt"
	"github.com/tink-crypto/tink-go/v2/tink"
)

type Jwt struct {
	mac jwt.MAC
}

func (p *Jwt) Verify(token string, issuer string, audience string) (string, string, map[string]string, error) {
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

	claims := make(map[string]string)
	for _, k := range verified_jwt.CustomClaimNames() {
		v, err := verified_jwt.StringClaim(k)
		if err != nil {
			return "", subject, nil, nil
		}
		claims[k] = v
	}

	return jwt_id, subject, claims, nil
}

func (p *Jwt) Sign(issuer string, subject string, audience string, claims map[string]string, ttl time.Duration) (string, error) {
	custom_claims := make(map[string]interface{})
	for k, v := range claims {
		custom_claims[k] = v
	}

	kid := uuid.NewString()
	now := time.Now()
	exp := now.Add(ttl)

	raw, err := jwt.NewRawJWT(&jwt.RawJWTOptions{
		JWTID:        &kid,
		Audience:     &audience,
		Subject:      &subject,
		CustomClaims: custom_claims,
		ExpiresAt:    &exp,
		NotBefore:    &now,
		Issuer:       &issuer,
		IssuedAt:     &now,
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
