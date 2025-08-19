package casbin

import (
	"context"

	"github.com/casbin/casbin/v2"
	"google.golang.org/protobuf/types/known/emptypb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/jasmine/env/crypto"
	v2 "github.com/saturn-xiv/palm/jasmine/services/casbin/v2"
	"github.com/saturn-xiv/palm/jasmine/services/portal/models"
	"github.com/saturn-xiv/palm/jasmine/web"
)

type SessionServer struct {
	v2.UnimplementedSessionServer

	jwt      *crypto.Jwt
	db       *gorm.DB
	enforcer *casbin.Enforcer
}

func (p *SessionServer) Has(ctx context.Context, req *v2.Role) (*v2.BoolResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}

func (p *SessionServer) Can(ctx context.Context, req *v2.SessionCanRequest) (*v2.BoolResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}

func (p *SessionServer) Roles(ctx context.Context, req *emptypb.Empty) (*v2.RolesResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}

func (p *SessionServer) Permissions(ctx context.Context, req *emptypb.Empty) (*v2.PermissionsResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}

func (p *SessionServer) ImplicitRoles(ctx context.Context, req *emptypb.Empty) (*v2.RolesResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}

func (p *SessionServer) ImplicitPermissions(ctx context.Context, req *emptypb.Empty) (*v2.PermissionsResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsSignedIn() {
		return nil, web.ErrorUserIsNotSignedIn
	}
	// TODO
	return nil, nil
}

func NewSessionServer(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer) *SessionServer {
	return &SessionServer{db: db, enforcer: enforcer, jwt: jwt}
}
