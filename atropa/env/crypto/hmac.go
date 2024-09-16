package crypto

import (
	"bytes"
	"encoding/gob"

	"github.com/tink-crypto/tink-go/v2/mac"
	"github.com/tink-crypto/tink-go/v2/tink"
)

type HMac struct {
	mac tink.MAC
}

type hmacPayloadWithSalt struct {
	Raw  []byte
	Salt []byte
}

func (p *hmacPayloadWithSalt) encode() ([]byte, error) {
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	if err := enc.Encode(p); err != nil {
		return nil, err
	}
	return buf.Bytes(), nil
}

func (p *HMac) Sign(plain []byte) ([]byte, []byte, error) {
	salt := RandomBytes(16)
	it := hmacPayloadWithSalt{
		Raw:  plain,
		Salt: salt,
	}
	buf, err := it.encode()
	if err != nil {
		return nil, nil, err
	}
	code, err := p.mac.ComputeMAC(buf)
	if err != nil {
		return nil, nil, err
	}
	return code, salt, nil
}

func (p *HMac) Verify(code []byte, plain []byte, salt []byte) error {
	it := &hmacPayloadWithSalt{
		Raw:  plain,
		Salt: salt,
	}
	buf, err := it.encode()
	if err != nil {
		return err
	}
	return p.mac.VerifyMAC(code, buf)
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
