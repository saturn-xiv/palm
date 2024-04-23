package user

import pb "github.com/saturn-xiv/palm/lilac/rbac/v2"

func (p *Command) Exempt(email string, role_ string) error {
	_, user, err := user_from_email(p.db, email)
	if err != nil {
		return err
	}
	user_p := pb.User{Id: user.ID}
	user_s, err := user_p.Code()
	if err != nil {
		return err
	}
	role, err := role_from_code(role_)
	if err != nil {
		return err
	}
	role_s, err := role.Code()
	if err != nil {
		return err
	}
	if _, err = p.enforcer.DeleteRoleForUser(user_s, role_s); err != nil {
		return err
	}
	return nil
}
