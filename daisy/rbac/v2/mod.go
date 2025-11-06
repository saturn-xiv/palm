package v2

import (
	"bytes"
	"encoding/base64"
	"encoding/gob"
	"fmt"
	"strings"
)

func NewAction(s string) (*Action, error) {
	tmp, err := base64.URLEncoding.WithPadding(base64.NoPadding).DecodeString(s)
	if err != nil {
		return nil, err
	}
	buf := bytes.NewBuffer(tmp)
	dec := gob.NewDecoder(buf)
	var it Action
	if err = dec.Decode(&it); err != nil {
		return nil, err
	}
	return &it, nil
}

func (p *Action) ToString() (string, error) {
	var buf bytes.Buffer
	{
		enc := gob.NewEncoder(&buf)
		if err := enc.Encode(p); err != nil {
			return "", err
		}
	}
	return base64.URLEncoding.WithPadding(base64.NoPadding).EncodeToString(buf.Bytes()), nil
}

func NewObject(s string) (*Object, error) {
	tmp, err := base64.URLEncoding.WithPadding(base64.NoPadding).DecodeString(s)
	if err != nil {
		return nil, err
	}
	buf := bytes.NewBuffer(tmp)
	dec := gob.NewDecoder(buf)
	var it Object
	if err = dec.Decode(&it); err != nil {
		return nil, err
	}
	return &it, nil
}

func (p *Object) ToString() (string, error) {
	var buf bytes.Buffer
	{
		enc := gob.NewEncoder(&buf)
		if err := enc.Encode(p); err != nil {
			return "", err
		}
	}
	return base64.URLEncoding.WithPadding(base64.NoPadding).EncodeToString(buf.Bytes()), nil
}

func NewSubject(s string) (*Subject, error) {
	tmp, err := base64.URLEncoding.WithPadding(base64.NoPadding).DecodeString(s)
	if err != nil {
		return nil, err
	}
	buf := bytes.NewBuffer(tmp)
	dec := gob.NewDecoder(buf)
	var it Subject
	if err = dec.Decode(&it); err != nil {
		return nil, err
	}
	return &it, nil
}

func (p *Subject) ToString() (string, error) {
	var buf bytes.Buffer
	{
		enc := gob.NewEncoder(&buf)
		if err := enc.Encode(p); err != nil {
			return "", err
		}
	}
	return base64.URLEncoding.WithPadding(base64.NoPadding).EncodeToString(buf.Bytes()), nil
}

func (p *Subject_Role) Root() bool {
	switch p.By.(type) {
	case *Subject_Role_Root_:
		return true
	default:
		return false
	}
}

func (p *Subject_Role) Administrator() bool {
	switch p.By.(type) {
	case *Subject_Role_Administrator_:
		return true
	default:
		return false
	}
}

func (p *Subject_Role) Is(code string) bool {
	switch it := p.By.(type) {
	case *Subject_Role_Code:
		return it.Code == code
	default:
		return false
	}
}

func (p *Action) Inquiry() bool {
	switch p.By.(type) {
	case *Action_Inquiry_:
		return true
	default:
		return false
	}
}
func (p *Action) Debit() bool {
	switch p.By.(type) {
	case *Action_Debit_:
		return true
	default:
		return false
	}
}
func (p *Action) Credit() bool {
	switch p.By.(type) {
	case *Action_Credit_:
		return true
	default:
		return false
	}
}
func (p *Action) Execute() bool {
	switch p.By.(type) {
	case *Action_Execute_:
		return true
	default:
		return false
	}
}
func (p *Action) Append() bool {
	switch p.By.(type) {
	case *Action_Append_:
		return true
	default:
		return false
	}
}
func (p *Action) Write() bool {
	switch p.By.(type) {
	case *Action_Write_:
		return true
	default:
		return false
	}
}
func (p *Action) Read() bool {
	switch p.By.(type) {
	case *Action_Read_:
		return true
	default:
		return false
	}
}
func (p *Action) Is(code string) bool {
	switch it := p.By.(type) {
	case *Action_Code:
		return it.Code == code
	default:
		return false
	}
}

func NewPermission(rules []string) (*Permission, error) {
	if len(rules) != 3 {
		return nil, fmt.Errorf("unknows rules: %s", strings.Join(rules, ","))
	}
	sub, err := NewSubject(rules[0])
	if err != nil {
		return nil, err
	}
	obj, err := NewObject(rules[1])
	if err != nil {
		return nil, err
	}
	act, err := NewAction(rules[1])
	if err != nil {
		return nil, err
	}
	return &Permission{Subject: sub, Object: obj, Action: act}, nil

}

func (p *Permission) Rules() ([]string, error) {
	sub, err := p.Subject.ToString()
	if err != nil {
		return nil, err
	}
	obj, err := p.Object.ToString()
	if err != nil {
		return nil, err
	}
	act, err := p.Action.ToString()
	if err != nil {
		return nil, err
	}
	return []string{sub, obj, act}, nil
}
