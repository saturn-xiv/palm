package models

import (
	"context"
	"log/slog"
	"net/http"
	"slices"
	"strings"

	"github.com/casbin/casbin/v2"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/jasmine/env/crypto"
	casbin_v2 "github.com/saturn-xiv/palm/jasmine/services/casbin/v2"
	"github.com/saturn-xiv/palm/jasmine/web"
)

type Session struct {
	session     *web.Session
	user        *User
	subject     string
	roles       []string
	permissions [][]string
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
	role, err := web.ProtoBufMessageToString(casbin_v2.NewRoleSubject(role_))
	if err != nil {
		slog.Error("get role subject", slog.String("reason", err.Error()))
		return false
	}
	return slices.Contains(p.roles, role)
}

func (p *Session) Can(action *casbin_v2.Action, resource_type string, resource_id *uint32) bool {
	if resource_id == nil {
		return p.can(action, casbin_v2.NewObjectWithType(resource_type))
	}
	return p.can(action, casbin_v2.NewObjectById(resource_type, *resource_id))
}

func (p *Session) can(act_ *casbin_v2.Action, obj_ *casbin_v2.Object) bool {
	act, err := web.ProtoBufMessageToString(act_)
	if err != nil {
		slog.Error("get action", slog.String("reason", err.Error()))
		return false
	}
	obj, err := web.ProtoBufMessageToString(obj_)
	if err != nil {
		slog.Error("get object", slog.String("reason", err.Error()))
		return false
	}
	for _, it := range p.permissions {
		if len(it) != 3 {
			slog.Warn("unknown permission", slog.String("rule", strings.Join(it, ",")))
			continue
		}
		if it[1] == obj && it[2] == act {
			return true
		}
	}
	return false
}

func SessionFromHttp(db *gorm.DB, enf *casbin.Enforcer, jwt *crypto.Jwt, r *http.Request) *Session {
	return new_session(db, enf, jwt, web.SessionFromHttp(r))
}

func SessionFromGrpc(db *gorm.DB, enf *casbin.Enforcer, jwt *crypto.Jwt, c context.Context) *Session {
	return new_session(db, enf, jwt, web.SessionFromGrpc(c))
}

func new_session(db *gorm.DB, enf *casbin.Enforcer, jwt *crypto.Jwt, ss *web.Session) *Session {
	it := Session{session: ss}
	if len(ss.Token) == 0 {
		return &it
	}
	{
		_, uid, _, err := jwt.Verify("t", "i", "a")
		if err != nil {
			slog.Error("parse token failed", slog.String("reason", err.Error()))
			return &it
		}
		user, err := UserByUID(db, uid)
		if err != nil {
			slog.Error("failed to find current user", slog.String("reason", err.Error()))
			return &it
		}
		if user.LockedAt != nil {
			slog.Error("user was locked")
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
