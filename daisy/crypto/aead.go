package crypto

import (
	"github.com/tink-crypto/tink-go/v2/aead"
	"github.com/tink-crypto/tink-go/v2/tink"
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
