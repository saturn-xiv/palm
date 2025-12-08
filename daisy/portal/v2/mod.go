package v2

import (
	"encoding/base64"

	"google.golang.org/protobuf/proto"
)

func (p *Session) ToString() (string, error) {
	buf, err := proto.Marshal(p)
	if err != nil {
		return "", err
	}
	return base64.URLEncoding.EncodeToString(buf), nil
}

func NewSession(s string) (*Session, error) {
	tmp, err := base64.URLEncoding.DecodeString(s)
	if err != nil {
		return nil, err
	}
	var it Session
	if err := proto.Unmarshal(tmp, &it); err != nil {
		return nil, err
	}
	return &it, nil
}
