package v2

import "google.golang.org/protobuf/types/known/emptypb"

// ----------------------------------------------------------------------------
func NewReadAction() *Action {
	return &Action{
		By: &Action_Read_{
			Read: &Action_Read{},
		},
	}
}
func NewWriteAction() *Action {
	return &Action{
		By: &Action_Write_{
			Write: &Action_Write{},
		},
	}
}
func NewAppendAction() *Action {
	return &Action{
		By: &Action_Append_{
			Append: &Action_Append{},
		},
	}
}
func NewCreditAction() *Action {
	return &Action{
		By: &Action_Credit_{
			Credit: &Action_Credit{},
		},
	}
}
func NewDebitAction() *Action {
	return &Action{
		By: &Action_Debit_{
			Debit: &Action_Debit{},
		},
	}
}
func NewExecuteAction() *Action {
	return &Action{
		By: &Action_Execute_{
			Execute: &Action_Execute{},
		},
	}
}
func NewInquiryAction() *Action {
	return &Action{
		By: &Action_Inquiry_{
			Inquiry: &Action_Inquiry{},
		},
	}
}
func NewOtherAction(code string) *Action {
	return &Action{
		By: &Action_Other_{
			Other: &Action_Other{
				Code: code,
			},
		},
	}
}

// ----------------------------------------------------------------------------
func NewObjectWithType(type_ string) *Object {
	return &Object{
		Type: type_,
		By: &Object_Empty{
			Empty: &emptypb.Empty{},
		},
	}
}

func NewObjectById(type_ string, id uint32) *Object {
	return &Object{
		Type: type_,
		By: &Object_Id{
			Id: id,
		},
	}
}

func NewObjectByCode(type_ string, code string) *Object {
	return &Object{
		Type: type_,
		By: &Object_Code{
			Code: code,
		},
	}
}

// ----------------------------------------------------------------------------
func NewUserSubjectById(id uint32) *Subject {
	return &Subject{
		By: &Subject_User{
			User: &User{
				By: &User_Id{
					Id: id,
				},
			},
		},
	}
}
func NewUserSubjectByCode(code string) *Subject {
	return &Subject{
		By: &Subject_User{
			User: &User{
				By: &User_Code{
					Code: code,
				},
			},
		},
	}
}

// ----------------------------------------------------------------------------
func NewRoleSubject(role *Role) *Subject {
	return &Subject{
		By: &Subject_Role{
			Role: role,
		},
	}
}
func NewRootRole() *Role {
	return &Role{
		By: &Role_Root_{
			Root: &Role_Root{},
		},
	}
}
func NewRootRoleSubject() *Subject {
	return NewRoleSubject(NewRootRole())
}
func NewAdministratorRole() *Role {
	return &Role{
		By: &Role_Administrator_{
			Administrator: &Role_Administrator{},
		}}
}
func NewAdministratorRoleSubject() *Subject {
	return NewRoleSubject(NewAdministratorRole())
}
func NewRoleByCode(code string) *Role {
	return &Role{
		By: &Role_Other_{
			Other: &Role_Other{Code: code},
		},
	}
}
func NewRoleSubjectByCode(code string) *Subject {
	return NewRoleSubject(NewRoleByCode(code))
}

// ----------------------------------------------------------------------------
