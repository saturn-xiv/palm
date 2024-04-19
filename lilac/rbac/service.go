package rbac

import (
	"context"
	"log/slog"

	"github.com/casbin/casbin/v2"
	"google.golang.org/protobuf/types/known/emptypb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/auth"
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/models"
	pb "github.com/saturn-xiv/palm/lilac/rbac/v2"
)

type Service struct {
	pb.UnimplementedPolicyServer

	db       *gorm.DB
	jwt      *crypto.Jwt
	enforcer *casbin.Enforcer
}

func (p *Service) Users(ctx context.Context, req *emptypb.Empty) (*pb.PolicyUsersResponse, error) {
	{
		user, err := auth.NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	var users []models.User
	if rst := p.db.Select("id").Find(&users); rst.Error != nil {
		return nil, rst.Error
	}
	var items []*pb.UserDetail
	for _, ur := range users {
		it, err := models.NewUserDetail(p.db, ur.ID)
		if err != nil {
			return nil, err
		}
		items = append(items, it)
	}
	return &pb.PolicyUsersResponse{Items: items}, nil
}

func (p *Service) Roles(ctx context.Context, req *emptypb.Empty) (*pb.PolicyRolesResponse, error) {
	user, err := auth.NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	user_q := pb.User{Id: user.Payload.ID}
	subject, err := user_q.Code()
	if err != nil {
		return nil, err
	}
	roles, err := p.enforcer.GetImplicitRolesForUser(subject)
	if err != nil {
		return nil, err
	}
	var items []*pb.Role
	for _, ro := range roles {
		it, err := pb.RoleFromCode(ro)
		if err != nil {
			return nil, err
		}
		items = append(items, it)
	}
	return &pb.PolicyRolesResponse{Items: items}, nil
}
func (p *Service) Permissions(ctx context.Context, req *emptypb.Empty) (*pb.PolicyPermissionsResponse, error) {
	user, err := auth.NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	user_q := pb.User{Id: user.Payload.ID}
	subject, err := user_q.Code()
	if err != nil {
		return nil, err
	}
	permissions, err := p.enforcer.GetImplicitPermissionsForUser(subject)
	if err != nil {
		return nil, err
	}
	items, err := pb.FromPermissions(permissions)
	if err != nil {
		return nil, err
	}
	return &pb.PolicyPermissionsResponse{Items: items}, nil
}

func (p *Service) AddRolesForUser(ctx context.Context, req *pb.PolicyRolesForUserRequest) (*emptypb.Empty, error) {
	{
		user, err := auth.NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}

	user := pb.User{Id: req.User}
	user_s, err := user.Code()
	if err != nil {
		return nil, err
	}
	var roles []string
	for _, ro := range req.Roles {
		rs, err := ro.Code()
		if err != nil {
			return nil, err
		}
		roles = append(roles, rs)
	}
	if _, err = p.enforcer.AddRolesForUser(user_s, roles); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *Service) DeleteRolesForUser(ctx context.Context, req *pb.PolicyRolesForUserRequest) (*emptypb.Empty, error) {
	{
		user, err := auth.NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}

	user := pb.User{Id: req.User}
	user_s, err := user.Code()
	if err != nil {
		return nil, err
	}
	for _, ro := range req.Roles {
		rs, err := ro.Code()
		if err != nil {
			return nil, err
		}
		if _, err = p.enforcer.DeleteRoleForUser(user_s, rs); err != nil {
			return nil, err
		}
	}

	return &emptypb.Empty{}, nil
}
func (p *Service) GetRolesForUser(ctx context.Context, req *pb.User) (*pb.PolicyRolesResponse, error) {
	{
		user, err := auth.NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	user, err := req.Code()
	if err != nil {
		return nil, err
	}

	roles, err := p.enforcer.GetRolesForUser(user)
	if err != nil {
		return nil, err
	}
	var items []*pb.Role
	for _, ro := range roles {
		it, err := pb.RoleFromCode(ro)
		if err != nil {
			return nil, err
		}
		items = append(items, it)
	}

	return &pb.PolicyRolesResponse{Items: items}, nil
}

func (p *Service) AddUsersForRole(ctx context.Context, req *pb.PolicyUsersForRoleRequest) (*emptypb.Empty, error) {
	{
		user, err := auth.NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}

	role, err := req.Role.Code()
	if err != nil {
		return nil, err
	}
	for _, ur := range req.Users {
		uq := pb.User{Id: ur}
		us, err := uq.Code()
		if err != nil {
			return nil, err
		}
		if _, err := p.enforcer.AddRoleForUser(us, role); err != nil {
			return nil, err
		}
	}
	return &emptypb.Empty{}, nil
}
func (p *Service) DeleteUsersForRole(ctx context.Context, req *pb.PolicyUsersForRoleRequest) (*emptypb.Empty, error) {
	{
		user, err := auth.NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}

	role, err := req.Role.Code()
	if err != nil {
		return nil, err
	}
	for _, ur := range req.Users {
		uq := pb.User{Id: ur}
		us, err := uq.Code()
		if err != nil {
			return nil, err
		}
		if _, err := p.enforcer.DeleteRoleForUser(us, role); err != nil {
			return nil, err
		}
	}
	return &emptypb.Empty{}, nil
}

func (p *Service) GetUsersForRole(ctx context.Context, req *pb.PolicyRoleRequest) (*pb.PolicyUsersResponse, error) {
	{
		user, err := auth.NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	role := pb.Role{
		By: &pb.Role_Member{
			Member: req.Name,
		},
	}
	role_s, err := role.Code()
	if err != nil {
		return nil, err
	}
	users, err := p.enforcer.GetUsersForRole(role_s)
	if err != nil {
		return nil, err
	}
	var items []*pb.UserDetail
	for _, ur := range users {
		uq, err := pb.UserFromCode(ur)
		if err != nil {
			return nil, err
		}
		it, err := models.NewUserDetail(p.db, uq.Id)
		if err != nil {
			return nil, err
		}
		items = append(items, it)
	}
	return &pb.PolicyUsersResponse{
		Items: items,
	}, nil
}
func (p *Service) DeleteRole(ctx context.Context, req *pb.PolicyRoleRequest) (*emptypb.Empty, error) {
	{
		user, err := auth.NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	slog.Warn("delete role", slog.String("name", req.Name))
	role := pb.Role{
		By: &pb.Role_Member{
			Member: req.Name,
		},
	}
	role_s, err := role.Code()
	if err != nil {
		return nil, err
	}
	if _, err = p.enforcer.DeleteRole(role_s); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *Service) AddPermissionsForUser(ctx context.Context, req *pb.PolicyPermissionsForUserRequest) (*emptypb.Empty, error) {
	{
		user, err := auth.NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	user := pb.User{
		Id: req.User,
	}
	subject, err := user.Code()
	if err != nil {
		return nil, err
	}
	permissions, err := pb.ToPermissions(req.Permissions)
	if err != nil {
		return nil, err
	}
	if _, err = p.enforcer.AddPermissionsForUser(subject, permissions...); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil

}
func (p *Service) DeletePermissionsForUser(ctx context.Context, req *pb.PolicyPermissionsForUserRequest) (*emptypb.Empty, error) {
	{
		user, err := auth.NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	user := pb.User{
		Id: req.User,
	}
	subject, err := user.Code()
	if err != nil {
		return nil, err
	}
	permissions, err := pb.ToPermissions(req.Permissions)
	if err != nil {
		return nil, err
	}
	for _, it := range permissions {
		if _, err = p.enforcer.DeletePermissionForUser(subject, it...); err != nil {
			return nil, err
		}
	}
	return &emptypb.Empty{}, nil
}
func (p *Service) SetPermissionsForUser(ctx context.Context, req *pb.PolicyPermissionsForUserRequest) (*emptypb.Empty, error) {
	{
		user, err := auth.NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	user := pb.User{
		Id: req.User,
	}
	subject, err := user.Code()
	if err != nil {
		return nil, err
	}
	if _, err := p.enforcer.DeletePermissionsForUser(subject); err != nil {
		return nil, err
	}
	permissions, err := pb.ToPermissions(req.Permissions)
	if err != nil {
		return nil, err
	}
	if _, err = p.enforcer.AddPermissionsForUser(subject, permissions...); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *Service) GetPermissionsForUser(ctx context.Context, req *pb.User) (*pb.PolicyPermissionsResponse, error) {
	{
		user, err := auth.NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}

	subject, err := req.Code()
	if err != nil {
		return nil, err
	}
	permissions, err := p.enforcer.GetPermissionsForUser(subject)
	if err != nil {
		return nil, err
	}
	items, err := pb.FromPermissions(permissions)
	if err != nil {
		return nil, err
	}

	return &pb.PolicyPermissionsResponse{
		Items: items,
	}, nil
}

func (p *Service) AddPermissionsForRole(ctx context.Context, req *pb.PolicyPermissionsForRoleRequest) (*emptypb.Empty, error) {
	{
		user, err := auth.NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	role := pb.Role{
		By: &pb.Role_Member{
			Member: req.Role,
		},
	}
	subject, err := role.Code()
	if err != nil {
		return nil, err
	}
	permissions, err := pb.ToPermissions(req.Permissions)
	if err != nil {
		return nil, err
	}
	if _, err = p.enforcer.AddPermissionsForUser(subject, permissions...); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *Service) DeletePermissionsForRole(ctx context.Context, req *pb.PolicyPermissionsForRoleRequest) (*emptypb.Empty, error) {
	{
		user, err := auth.NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	role := pb.Role{
		By: &pb.Role_Member{
			Member: req.Role,
		},
	}
	subject, err := role.Code()
	if err != nil {
		return nil, err
	}
	permissions, err := pb.ToPermissions(req.Permissions)
	if err != nil {
		return nil, err
	}
	for _, it := range permissions {
		if _, err = p.enforcer.DeletePermissionForUser(subject, it...); err != nil {
			return nil, err
		}
	}
	return &emptypb.Empty{}, nil
}
func (p *Service) SetPermissionsForRole(ctx context.Context, req *pb.PolicyPermissionsForRoleRequest) (*emptypb.Empty, error) {
	{
		user, err := auth.NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	role := pb.Role{
		By: &pb.Role_Member{
			Member: req.Role,
		},
	}
	subject, err := role.Code()
	if err != nil {
		return nil, err
	}
	if _, err := p.enforcer.DeletePermissionsForUser(subject); err != nil {
		return nil, err
	}
	permissions, err := pb.ToPermissions(req.Permissions)
	if err != nil {
		return nil, err
	}
	if _, err = p.enforcer.AddPermissionsForUser(subject, permissions...); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *Service) GetPermissionsForRole(ctx context.Context, req *pb.PolicyRoleRequest) (*pb.PolicyPermissionsResponse, error) {
	{
		user, err := auth.NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	role := pb.Role{
		By: &pb.Role_Member{
			Member: req.Name,
		},
	}
	subject, err := role.Code()
	if err != nil {
		return nil, err
	}
	permissions, err := p.enforcer.GetPermissionsForUser(subject)
	if err != nil {
		return nil, err
	}
	items, err := pb.FromPermissions(permissions)
	if err != nil {
		return nil, err
	}
	return &pb.PolicyPermissionsResponse{
		Items: items,
	}, nil
}

func NewService(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer) *Service {
	return &Service{db: db, jwt: jwt, enforcer: enforcer}
}
