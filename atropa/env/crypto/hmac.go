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

func decodeHmacPayloadWithSalt(bin []byte) (*hmacPayloadWithSalt, error) {
	buf := bytes.NewBuffer(bin)
	dec := gob.NewDecoder(buf)
	var it hmacPayloadWithSalt
	if err := dec.Decode(&it); err != nil {
		return nil, err
	}
	return &it, nil
}

func (p *hmacPayloadWithSalt) encode() ([]byte, error) {
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	if err := enc.Encode(p); err != nil {
		return nil, err
	}
	return buf.Bytes(), nil
}

func (p *HMac) Sign(plain []byte) ([]byte, error) {
	bin, err := p.mac.ComputeMAC(plain)
	if err != nil {
		return nil, err
	}
	it := hmacPayloadWithSalt{
		Raw:  bin,
		Salt: RandomBytes(8),
	}
	return it.encode()
}

func (p *HMac) Verify(code []byte, plain []byte) error {
	tmp, err := decodeHmacPayloadWithSalt(code)
	if err != nil {
		return err
	}
	it := &hmacPayloadWithSalt{
		Raw:  plain,
		Salt: tmp.Salt,
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
