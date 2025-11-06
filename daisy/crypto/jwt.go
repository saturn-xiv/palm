package crypto

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/tink-crypto/tink-go/v2/jwt"

	v2 "github.com/saturn_xiv/palm/daisy/crypto/v2"
)

type Jwt struct {
	primitive jwt.MAC
}

func (p *Jwt) Sign(issuer string, subject string, audiences []string, ttl time.Duration) (string, error) {
	jid := uuid.New().String()
	now := time.Now()
	exp := now.Add(ttl)
	nbf := now.Add(time.Second * -1)

	raw, err := jwt.NewRawJWT(&jwt.RawJWTOptions{
		JWTID:     &jid,
		Issuer:    &issuer,
		Subject:   &subject,
		Audiences: audiences,
		NotBefore: &nbf,
		ExpiresAt: &exp,
		IssuedAt:  &now,
	})
	if err != nil {
		return "", nil
	}
	return p.primitive.ComputeMACAndEncode(raw)
}

func (p *Jwt) Verify(token string, issuer string, audience string) (string, string, error) {
	validator, err := jwt.NewValidator(&jwt.ValidatorOpts{
		ExpectedAudience:       &audience,
		ExpectedIssuer:         &issuer,
		ExpectIssuedInThePast:  true,
		AllowMissingExpiration: false,
	})
	if err != nil {
		return "", "", err
	}
	raw, err := p.primitive.VerifyMACAndDecode(token, validator)
	if err != nil {
		return "", "", err
	}

	sub, err := raw.Subject()
	if err != nil {
		return "", "", err
	}
	jid, err := raw.JWTID()
	if err != nil {
		return "", "", err
	}
	return jid, sub, nil
}

func NewJwt(name string) (*Jwt, error) {
	handle, err := load_keyset_file(name, jwt.HS512Template())
	if err != nil {
		return nil, err
	}

	primitive, err := jwt.NewMAC(handle)
	if err != nil {
		return nil, err
	}
	return &Jwt{primitive: primitive}, nil
}

type JwtServer struct {
	v2.UnimplementedJwtServer

	jwt *Jwt
}

func NewJwtServer(jwt *Jwt) *JwtServer {
	return &JwtServer{jwt: jwt}
}

func (p *JwtServer) Sign(ctx context.Context, req *v2.JwtSignRequest) (*v2.JwtSignResponse, error) {
	token, err := p.jwt.Sign(req.Issuer, req.Subject, req.Audiences, req.Ttl.AsDuration())
	if err != nil {
		return nil, err
	}
	return &v2.JwtSignResponse{Token: token}, nil
}

func (p *JwtServer) Verify(ctx context.Context, req *v2.JwtVerifyRequest) (*v2.JwtVerifyResponse, error) {
	jid, sub, err := p.jwt.Verify(req.Token, req.Issuer, req.Audience)
	if err != nil {
		return nil, err
	}
	return &v2.JwtVerifyResponse{Subject: sub, Id: jid}, nil
}
