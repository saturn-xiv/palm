package v2

import (
	"bytes"
	"encoding/base64"
	"encoding/gob"
)

func (p *PolicyUserRequest) Code() (string, error) {
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	if err := enc.Encode(p); err != nil {
		return "", err
	}
	return base64.StdEncoding.EncodeToString(buf.Bytes()), nil
}

func (p *PolicyRolesResponse_Item) Code() (string, error) {
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	if err := enc.Encode(p); err != nil {
		return "", err
	}
	return base64.StdEncoding.EncodeToString(buf.Bytes()), nil
}

func NewPolicyRole(code string) (*PolicyRolesResponse_Item, error) {
	bin, err := base64.StdEncoding.DecodeString(code)
	if err != nil {
		return nil, err
	}
	buf := bytes.NewBuffer(bin)
	dec := gob.NewDecoder(buf)
	var it PolicyRolesResponse_Item
	if err = dec.Decode(&it); err != nil {
		return nil, err
	}
	return &it, nil
}

func NewPolicyUser(code string) (*PolicyUserRequest, error) {
	bin, err := base64.StdEncoding.DecodeString(code)
	if err != nil {
		return nil, err
	}
	buf := bytes.NewBuffer(bin)
	dec := gob.NewDecoder(buf)
	var it PolicyUserRequest
	if err = dec.Decode(&it); err != nil {
		return nil, err
	}
	return &it, nil
}
