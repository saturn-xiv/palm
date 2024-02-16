package v2

import (
	"encoding/base64"

	"google.golang.org/protobuf/proto"
)

func (p *User) ToSubject() (string, error) {
	buf, err := proto.Marshal(p)
	if err != nil {
		return "", err
	}
	return base64.StdEncoding.EncodeToString(buf), nil
}

func NewUserFromSubject(subject string) (*User, error) {
	buf, err := base64.StdEncoding.DecodeString(subject)
	if err != nil {
		return nil, err
	}
	var it User
	if err = proto.Unmarshal(buf, &it); err != nil {
		return nil, err
	}
	return &it, nil
}

func (p *Role) ToSubject() (string, error) {
	buf, err := proto.Marshal(p)
	if err != nil {
		return "", err
	}
	return base64.StdEncoding.EncodeToString(buf), nil
}

func NewRoleSubject(subject string) (*Role, error) {
	buf, err := base64.StdEncoding.DecodeString(subject)
	if err != nil {
		return nil, err
	}
	var it Role
	if err = proto.Unmarshal(buf, &it); err != nil {
		return nil, err
	}
	return &it, nil
}

func (p *Resource) ToObject() (string, error) {
	buf, err := proto.Marshal(p)
	if err != nil {
		return "", err
	}
	return base64.StdEncoding.EncodeToString(buf), nil
}

func NewResourceFromObject(object string) (*Resource, error) {
	buf, err := base64.StdEncoding.DecodeString(object)
	if err != nil {
		return nil, err
	}
	var it Resource
	if err = proto.Unmarshal(buf, &it); err != nil {
		return nil, err
	}
	return &it, nil
}
