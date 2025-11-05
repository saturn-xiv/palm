package crypto

import (
	"context"

	"github.com/tink-crypto/tink-go/v2/aead"
	"github.com/tink-crypto/tink-go/v2/tink"

	v2 "com.github/saturn_xiv/palm/daisy/crypto/v2"
)

type Aead struct {
	primitive tink.AEAD
}

func NewAead(name string) (*Aead, error) {
	handle, err := load_keyset_file(name, aead.AES256GCMKeyTemplate())
	if err != nil {
		return nil, err
	}

	primitive, err := aead.New(handle)
	if err != nil {
		return nil, err
	}
	return &Aead{primitive: primitive}, nil
}

func (p *Aead) Encrypt(plain []byte, associated []byte) ([]byte, error) {
	return p.primitive.Encrypt(plain, associated)
}

func (p *Aead) Decrypt(cipher []byte, associated []byte) ([]byte, error) {
	return p.primitive.Decrypt(cipher, associated)
}

type AeadServer struct {
	v2.UnimplementedAeadServer

	aead *Aead
}

func NewAeadServer(aead *Aead) *AeadServer {
	return &AeadServer{aead: aead}
}

func (p *AeadServer) Encrypt(ctx context.Context, req *v2.AeadEncryptRequest) (*v2.AeadEncryptResponse, error) {
	cipher, err := p.aead.Encrypt(req.Plain, req.Associated)
	if err != nil {
		return nil, err
	}

	return &v2.AeadEncryptResponse{Cipher: cipher}, nil
}

func (p *AeadServer) Decrypt(ctx context.Context, req *v2.AeadDecryptRequest) (*v2.AeadDecryptResponse, error) {
	plain, err := p.aead.Decrypt(req.Cipher, req.Associated)
	if err != nil {
		return nil, err
	}

	return &v2.AeadDecryptResponse{Plain: plain}, nil
}
