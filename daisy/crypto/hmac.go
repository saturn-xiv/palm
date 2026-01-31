package crypto

import (
	"github.com/tink-crypto/tink-go/v2/mac"
	"github.com/tink-crypto/tink-go/v2/tink"
)

type Hmac struct {
	primitive tink.MAC
}

func NewHmac(name string) (*Hmac, error) {
	handle, err := load_keyset_file(name, mac.HMACSHA512Tag256KeyTemplate())
	if err != nil {
		return nil, err
	}

	primitive, err := mac.New(handle)
	if err != nil {
		return nil, err
	}
	return &Hmac{primitive: primitive}, nil
}

func (p *Hmac) Compute(data []byte) ([]byte, error) {
	return p.primitive.ComputeMAC(data)
}
func (p *Hmac) Verify(mac []byte, data []byte) error {
	return p.primitive.VerifyMAC(mac, data)
}
