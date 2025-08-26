package models

import (
	"context"
	"log/slog"
	"net/http"

	"github.com/casbin/casbin/v2"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/jasmine/env/crypto"
	casbin_v2 "github.com/saturn-xiv/palm/jasmine/services/casbin/v2"
	v2 "github.com/saturn-xiv/palm/jasmine/services/portal/v2"
	"github.com/saturn-xiv/palm/jasmine/web"
)

type Session struct {
	session     *web.Session
	user        *User
	subject     string
	roles       []string
	permissions [][]string
}

func (p *Session) ID() uint32 {
	return p.user.ID
}

func (p *Session) IsSignedIn() bool {
	return p.user != nil
}

func (p *Session) IsRoot() bool {
	return p.has(casbin_v2.NewRootRole())
}

func (p *Session) IsAdministrator() bool {
	return p.has(casbin_v2.NewAdministratorRole())
}

func (p *Session) Has(role string) bool {
	return p.has(casbin_v2.NewRoleByCode(web.ToCode(role)))
}
func (p *Session) has(role_ *casbin_v2.Role) bool {
	if p.user == nil {
		return false
	}
	return casbin_v2.Has(p.roles, role_)
}

func (p *Session) Can(action *casbin_v2.Action, resource_type string, resource_id *uint32) bool {
	if resource_id == nil {
		return p.can(action, casbin_v2.NewObjectWithType(resource_type))
	}
	return p.can(action, casbin_v2.NewObjectById(resource_type, *resource_id))
}

func (p *Session) can(act_ *casbin_v2.Action, obj_ *casbin_v2.Object) bool {
	if p.user == nil {
		return false
	}
	return casbin_v2.Can(p.permissions, obj_, act_)
}

func SessionFromHttp(db *gorm.DB, enf *casbin.Enforcer, jwt *crypto.Jwt, r *http.Request) *Session {
	return new_session(db, enf, jwt, web.SessionFromHttp(r))
}

func SessionFromGrpc(db *gorm.DB, enf *casbin.Enforcer, jwt *crypto.Jwt, c context.Context) *Session {
	return new_session(db, enf, jwt, web.SessionFromGrpc(c))
}

func CurrentUser(db *gorm.DB, jwt *crypto.Jwt, ss *web.Session) (*User, error) {
	if len(ss.Token) == 0 {
		return nil, v2.ErrorUserIsNotSignedIn
	}
	// TODO
	_, uid, _, err := jwt.Verify("t", "i", "a")
	if err != nil {
		return nil, err
	}
	user, err := UserByUID(db, uid)
	if err != nil {
		return nil, err
	}
	if user.LockedAt != nil {
		return nil, v2.ErrorUserIsLocked
	}
	return user, nil
}

func new_session(db *gorm.DB, enf *casbin.Enforcer, jwt *crypto.Jwt, ss *web.Session) *Session {
	it := Session{session: ss}

	{
		user, err := CurrentUser(db, jwt, ss)
		if err != nil {
			return &it
		}
		it.user = user
	}
	{
		subject, err := web.ProtoBufMessageToString(casbin_v2.NewUserSubjectById(it.user.ID))
		if err != nil {
			slog.Error("get user subject", slog.String("reason", err.Error()))
		}
		it.subject = subject
	}
	{
		items, err := enf.GetImplicitRolesForUser(it.subject)
		if err != nil {
			slog.Error("fetch roles", slog.String("reason", err.Error()))
			return &it
		}
		it.roles = items
	}
	{
		items, err := enf.GetImplicitPermissionsForUser(it.subject)
		if err != nil {
			slog.Error("fetch permissions", slog.String("reason", err.Error()))
			return &it
		}
		it.permissions = items
	}
	return &it
}
