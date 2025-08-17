package v2

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
func NewUserById(id uint32) *Subject {
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
func NewUserByCode(code string) *Subject {
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
func NewRoot() *Subject {
	return &Subject{
		By: &Subject_Role{
			Role: &Role{
				By: &Role_Root_{
					Root: &Role_Root{},
				},
			},
		},
	}
}
func NewAdministrator() *Subject {
	return &Subject{
		By: &Subject_Role{
			Role: &Role{
				By: &Role_Administrator_{
					Administrator: &Role_Administrator{},
				},
			},
		},
	}

}
func NewRoleByCode(code string) *Subject {
	return &Subject{
		By: &Subject_Role{
			Role: &Role{
				By: &Role_Other_{
					Other: &Role_Other{Code: code},
				},
			},
		},
	}

}

// ----------------------------------------------------------------------------
