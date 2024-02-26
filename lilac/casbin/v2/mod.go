package v2

import (
	"encoding/base64"
	"fmt"
	"strings"

	"google.golang.org/protobuf/proto"
)

func (p *User) ToSubject() (string, error) {
	buf, err := proto.Marshal(p)
	if err != nil {
		return "", err
	}
	return base64.StdEncoding.EncodeToString(buf), nil
}

func NewUser(subject string) (*User, error) {
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

func NewRole(subject string) (*Role, error) {
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

func NewResource(object string) (*Resource, error) {
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

func NewPermission(args ...string) (*Permission, error) {
	if len(args) != 3 {
		return nil, fmt.Errorf("unknown permission(%s)", strings.Join(args, ","))
	}
	resource, err := NewResource(args[1])
	if err != nil {
		return nil, err
	}
	return &Permission{
		Resource: resource,
		Action:   args[2],
	}, nil
}
