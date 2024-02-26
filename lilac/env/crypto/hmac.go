package crypto

import (
	"github.com/tink-crypto/tink-go/v2/mac"
	"github.com/tink-crypto/tink-go/v2/tink"
)

type HMac struct {
	mac tink.MAC
}

func (p *HMac) Sign(plain []byte) ([]byte, error) {
	return p.mac.ComputeMAC(plain)
}

func (p *HMac) Verify(code []byte, plain []byte) error {
	return p.mac.VerifyMAC(code, plain)
}

func NewHMac(file string, secret tink.AEAD) (*HMac, error) {
	handle, err := Restore(file, secret)
	if err != nil {
		return nil, err
	}
	mac, err := mac.New(handle)
	if err != nil {
		return nil, err
	}
	return &HMac{mac: mac}, nil
}
