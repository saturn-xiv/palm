package v2

import (
	"encoding/base64"
	"fmt"
	"strings"

	"google.golang.org/protobuf/proto"
)

func NewPolicyPermissionFromRule(rule []string) (*PolicyPermissionsResponse_Item, error) {
	if len(rule) != 3 {
		return nil, fmt.Errorf("unexpected permission %v", rule)
	}
	var it PolicyPermissionsResponse_Item
	{
		resource, err := NewPolicyResourceFromCode(rule[1])
		if err != nil {
			return nil, err
		}
		it.Resource = resource
	}
	{
		operation, err := NewPolicyOperationFromCode(rule[2])
		if err != nil {
			return nil, err
		}
		it.Operation = operation
	}
	return &it, nil
}
func (p *PolicyPermissionsResponse_Item) Rule() ([]string, error) {
	resource, err := p.Resource.Code()
	if err != nil {
		return nil, err
	}
	operation, err := p.Operation.Code()
	if err != nil {
		return nil, err
	}
	return []string{resource, operation}, nil
}

func NewPolicyReadOperation() *PolicyPermissionsResponse_Item_Operation {
	return &PolicyPermissionsResponse_Item_Operation{
		By: &PolicyPermissionsResponse_Item_Operation_Read_{},
	}
}
func NewPolicyWriteOperation() *PolicyPermissionsResponse_Item_Operation {
	return &PolicyPermissionsResponse_Item_Operation{
		By: &PolicyPermissionsResponse_Item_Operation_Write_{},
	}
}
func NewPolicyExecuteOperation() *PolicyPermissionsResponse_Item_Operation {
	return &PolicyPermissionsResponse_Item_Operation{
		By: &PolicyPermissionsResponse_Item_Operation_Execute_{},
	}
}
func NewPolicyDebitOperation() *PolicyPermissionsResponse_Item_Operation {
	return &PolicyPermissionsResponse_Item_Operation{
		By: &PolicyPermissionsResponse_Item_Operation_Debit_{},
	}
}
func NewPolicyCreditOperation() *PolicyPermissionsResponse_Item_Operation {
	return &PolicyPermissionsResponse_Item_Operation{
		By: &PolicyPermissionsResponse_Item_Operation_Credit_{},
	}
}
func NewPolicyInquiryOperation() *PolicyPermissionsResponse_Item_Operation {
	return &PolicyPermissionsResponse_Item_Operation{
		By: &PolicyPermissionsResponse_Item_Operation_Inquiry_{},
	}
}
func NewPolicyAppendOperation() *PolicyPermissionsResponse_Item_Operation {
	return &PolicyPermissionsResponse_Item_Operation{
		By: &PolicyPermissionsResponse_Item_Operation_Append_{},
	}
}
func NewPolicyOperationFromCode(code string) (*PolicyPermissionsResponse_Item_Operation, error) {
	buf, err := base64.URLEncoding.WithPadding(base64.NoPadding).DecodeString(code)
	if err != nil {
		return nil, err
	}
	var it PolicyPermissionsResponse_Item_Operation
	if err = proto.Unmarshal(buf, &it); err != nil {
		return nil, err
	}
	return &it, nil
}
func (p *PolicyPermissionsResponse_Item_Operation) Code() (string, error) {
	buf, err := proto.Marshal(p)
	if err != nil {
		return "", err
	}
	return base64.URLEncoding.WithPadding(base64.NoPadding).EncodeToString(buf), nil
}
func (p *PolicyPermissionsResponse_Item_Operation) Display() string {
	switch it := p.By.(type) {
	case *PolicyPermissionsResponse_Item_Operation_Read_:
		return "read"
	case *PolicyPermissionsResponse_Item_Operation_Write_:
		return "write"
	case *PolicyPermissionsResponse_Item_Operation_Append_:
		return "append"
	case *PolicyPermissionsResponse_Item_Operation_Execute_:
		return "execute"
	case *PolicyPermissionsResponse_Item_Operation_Debit_:
		return "debit"
	case *PolicyPermissionsResponse_Item_Operation_Credit_:
		return "credit"
	case *PolicyPermissionsResponse_Item_Operation_Inquiry_:
		return "inquiry"
	case *PolicyPermissionsResponse_Item_Operation_Code:
		return fmt.Sprintf("(%s)", it.Code)
	default:
		return "unknown operation"
	}
}

func (p *PolicyPermissionsResponse_Item_Operation) IsAppend() bool {
	switch p.By.(type) {
	case *PolicyPermissionsResponse_Item_Operation_Append_:
		return true
	default:
		return false
	}
}
func (p *PolicyPermissionsResponse_Item_Operation) IsRead() bool {
	switch p.By.(type) {
	case *PolicyPermissionsResponse_Item_Operation_Read_:
		return true
	default:
		return false
	}
}

func (p *PolicyPermissionsResponse_Item_Operation) IsWrite() bool {
	switch p.By.(type) {
	case *PolicyPermissionsResponse_Item_Operation_Write_:
		return true
	default:
		return false
	}
}
func (p *PolicyPermissionsResponse_Item_Operation) IsExecute() bool {
	switch p.By.(type) {
	case *PolicyPermissionsResponse_Item_Operation_Execute_:
		return true
	default:
		return false
	}
}
func (p *PolicyPermissionsResponse_Item_Operation) IsDebit() bool {
	switch p.By.(type) {
	case *PolicyPermissionsResponse_Item_Operation_Debit_:
		return true
	default:
		return false
	}
}
func (p *PolicyPermissionsResponse_Item_Operation) IsCredit() bool {
	switch p.By.(type) {
	case *PolicyPermissionsResponse_Item_Operation_Credit_:
		return true
	default:
		return false
	}
}
func (p *PolicyPermissionsResponse_Item_Operation) IsInquiry() bool {
	switch p.By.(type) {
	case *PolicyPermissionsResponse_Item_Operation_Inquiry_:
		return true
	default:
		return false
	}
}
func NewPolicyResourceFromCode(code string) (*PolicyPermissionsResponse_Item_Resource, error) {
	buf, err := base64.URLEncoding.WithPadding(base64.NoPadding).DecodeString(code)
	if err != nil {
		return nil, err
	}
	var it PolicyPermissionsResponse_Item_Resource
	if err = proto.Unmarshal(buf, &it); err != nil {
		return nil, err
	}
	return &it, nil
}
func (p *PolicyPermissionsResponse_Item_Resource) Code() (string, error) {
	buf, err := proto.Marshal(p)
	if err != nil {
		return "", err
	}
	return base64.URLEncoding.WithPadding(base64.NoPadding).EncodeToString(buf), nil
}
func (p *PolicyPermissionsResponse_Item_Resource_Id) Equal(o *PolicyPermissionsResponse_Item_Resource_Id) bool {
	if o != nil {
		switch pt := p.By.(type) {
		case *PolicyPermissionsResponse_Item_Resource_Id_S:
			switch ot := o.By.(type) {
			case *PolicyPermissionsResponse_Item_Resource_Id_S:
				return pt.S == ot.S
			}
		case *PolicyPermissionsResponse_Item_Resource_Id_I:
			switch ot := o.By.(type) {
			case *PolicyPermissionsResponse_Item_Resource_Id_I:
				return pt.I == ot.I
			}
		}
	}
	return false
}
func (p *PolicyPermissionsResponse_Item_Resource) Display() string {
	if p.Id == nil {
		return fmt.Sprintf("%s://*", p.Type)
	}
	switch it := p.Id.By.(type) {
	case *PolicyPermissionsResponse_Item_Resource_Id_S:
		return fmt.Sprintf("%s://%s", p.Type, it.S)
	case *PolicyPermissionsResponse_Item_Resource_Id_I:
		return fmt.Sprintf("%s://%d", p.Type, it.I)
	default:
		return "unknown resource"
	}
}

func (p *PolicyPermissionsResponse_Item_Resource) IsRoot() bool {
	return p.Id == nil
}

func NewPolicyAdministratorRole() *PolicyRolesResponse_Item {
	return &PolicyRolesResponse_Item{
		By: &PolicyRolesResponse_Item_Administrator_{},
	}
}
func NewPolicyRootRole() *PolicyRolesResponse_Item {
	return &PolicyRolesResponse_Item{
		By: &PolicyRolesResponse_Item_Root_{},
	}
}
func NewPolicyRole(name string) *PolicyRolesResponse_Item {
	return &PolicyRolesResponse_Item{
		By: &PolicyRolesResponse_Item_Code{
			Code: strings.ToLower(strings.TrimSpace(name)),
		},
	}
}
func NewPolicyRoleFromCode(code string) (*PolicyRolesResponse_Item, error) {
	buf, err := base64.URLEncoding.WithPadding(base64.NoPadding).DecodeString(code)
	if err != nil {
		return nil, err
	}
	var it PolicyRolesResponse_Item
	if err = proto.Unmarshal(buf, &it); err != nil {
		return nil, err
	}
	return &it, nil
}
func (p *PolicyRolesResponse_Item) Code() (string, error) {
	buf, err := proto.Marshal(p)
	if err != nil {
		return "", err
	}
	return base64.URLEncoding.WithPadding(base64.NoPadding).EncodeToString(buf), nil
}
func (p *PolicyRolesResponse_Item) Display() string {
	switch it := p.By.(type) {
	case *PolicyRolesResponse_Item_Administrator_:
		return "administrator"
	case *PolicyRolesResponse_Item_Root_:
		return "root"
	case *PolicyRolesResponse_Item_Code:
		return fmt.Sprintf("(%s)", it.Code)
	default:
		return "unknown role"
	}
}
func (p *PolicyRolesResponse_Item) IsAdministrator() bool {
	switch p.By.(type) {
	case *PolicyRolesResponse_Item_Administrator_:
		return true
	default:
		return false
	}
}
func (p *PolicyRolesResponse_Item) IsRoot() bool {
	switch p.By.(type) {
	case *PolicyRolesResponse_Item_Root_:
		return true
	default:
		return false
	}
}

func NewPolicyUserFromCode(code string) (*PolicyUsersResponse_Item, error) {
	buf, err := base64.URLEncoding.WithPadding(base64.NoPadding).DecodeString(code)
	if err != nil {
		return nil, err
	}
	var it PolicyUsersResponse_Item
	if err = proto.Unmarshal(buf, &it); err != nil {
		return nil, err
	}
	return &it, nil
}
func (p *PolicyUsersResponse_Item) Code() (string, error) {
	buf, err := proto.Marshal(p)
	if err != nil {
		return "", err
	}
	return base64.URLEncoding.WithPadding(base64.NoPadding).EncodeToString(buf), nil
}
func (p *PolicyUsersResponse_Item) Display() string {
	switch it := p.Id.(type) {
	case *PolicyUsersResponse_Item_I:
		return fmt.Sprintf("int(%d)", it.I)
	case *PolicyUsersResponse_Item_S:
		return fmt.Sprintf("code(%s)", it.S)
	default:
		return "unknown user"
	}
}
