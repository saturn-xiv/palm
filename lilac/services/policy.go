package services

import (
	"context"
	"fmt"
	"log/slog"

	"github.com/casbin/casbin/v2"
	"google.golang.org/protobuf/types/known/emptypb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/models"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type PolicyService struct {
	pb.UnimplementedPolicyServer

	db       *gorm.DB
	jwt      *crypto.Jwt
	enforcer *casbin.Enforcer
}

func (p *PolicyService) Users(ctx context.Context, req *emptypb.Empty) (*pb.PolicyUsersResponse, error) {
	{
		user, err := NewCurrentUser(ctx, p.db, p.jwt)
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
	var items []*pb.PolicyUsersResponse_Item
	for _, ur := range users {
		it, err := models.NewPolicyUserItem(p.db, ur.ID)
		if err != nil {
			return nil, err
		}
		items = append(items, it)
	}
	return &pb.PolicyUsersResponse{Items: items}, nil
}

func (p *PolicyService) Roles(ctx context.Context, req *emptypb.Empty) (*pb.PolicyRolesResponse, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	user_q := pb.PolicyUserRequest{Id: user.Payload.ID}
	subject, err := user_q.Code()
	if err != nil {
		return nil, err
	}
	roles, err := p.enforcer.GetImplicitRolesForUser(subject)
	if err != nil {
		return nil, err
	}
	var items []*pb.PolicyRolesResponse_Item
	for _, ro := range roles {
		it, err := pb.NewPolicyRole(ro)
		if err != nil {
			return nil, err
		}
		items = append(items, it)
	}
	return &pb.PolicyRolesResponse{Items: items}, nil
}
func (p *PolicyService) Permissions(ctx context.Context, req *emptypb.Empty) (*pb.PolicyPermissionsResponse, error) {
	user, err := NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	user_q := pb.PolicyUserRequest{Id: user.Payload.ID}
	subject, err := user_q.Code()
	if err != nil {
		return nil, err
	}
	permissions, err := p.enforcer.GetImplicitPermissionsForUser(subject)
	if err != nil {
		return nil, err
	}
	items, err := from_permissions(permissions)
	if err != nil {
		return nil, err
	}
	return &pb.PolicyPermissionsResponse{Items: items}, nil
}

func (p *PolicyService) AddRolesForUser(ctx context.Context, req *pb.PolicyRolesForUserRequest) (*emptypb.Empty, error) {
	{
		user, err := NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}

	user := pb.PolicyUserRequest{Id: req.User}
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

func (p *PolicyService) DeleteRolesForUser(ctx context.Context, req *pb.PolicyRolesForUserRequest) (*emptypb.Empty, error) {
	{
		user, err := NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}

	user := pb.PolicyUserRequest{Id: req.User}
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
func (p *PolicyService) GetRolesForUser(ctx context.Context, req *pb.PolicyUserRequest) (*pb.PolicyRolesResponse, error) {
	{
		user, err := NewCurrentUser(ctx, p.db, p.jwt)
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
	var items []*pb.PolicyRolesResponse_Item
	for _, ro := range roles {
		it, err := pb.NewPolicyRole(ro)
		if err != nil {
			return nil, err
		}
		items = append(items, it)
	}

	return &pb.PolicyRolesResponse{Items: items}, nil
}

func (p *PolicyService) AddUsersForRole(ctx context.Context, req *pb.PolicyUsersForRoleRequest) (*emptypb.Empty, error) {
	{
		user, err := NewCurrentUser(ctx, p.db, p.jwt)
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
		uq := pb.PolicyUserRequest{Id: ur}
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
func (p *PolicyService) DeleteUsersForRole(ctx context.Context, req *pb.PolicyUsersForRoleRequest) (*emptypb.Empty, error) {
	{
		user, err := NewCurrentUser(ctx, p.db, p.jwt)
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
		uq := pb.PolicyUserRequest{Id: ur}
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

func (p *PolicyService) GetUsersForRole(ctx context.Context, req *pb.PolicyRoleRequest) (*pb.PolicyUsersResponse, error) {
	{
		user, err := NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	role := pb.PolicyRolesResponse_Item{
		By: &pb.PolicyRolesResponse_Item_Member{
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
	var items []*pb.PolicyUsersResponse_Item
	for _, ur := range users {
		uq, err := pb.NewPolicyUser(ur)
		if err != nil {
			return nil, err
		}
		it, err := models.NewPolicyUserItem(p.db, uq.Id)
		if err != nil {
			return nil, err
		}
		items = append(items, it)
	}
	return &pb.PolicyUsersResponse{
		Items: items,
	}, nil
}
func (p *PolicyService) DeleteRole(ctx context.Context, req *pb.PolicyRoleRequest) (*emptypb.Empty, error) {
	{
		user, err := NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	slog.Warn("delete role", slog.String("name", req.Name))
	role := pb.PolicyRolesResponse_Item{
		By: &pb.PolicyRolesResponse_Item_Member{
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

func (p *PolicyService) AddPermissionsForUser(ctx context.Context, req *pb.PolicyPermissionsForUserRequest) (*emptypb.Empty, error) {
	{
		user, err := NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	user := pb.PolicyUserRequest{
		Id: req.User,
	}
	subject, err := user.Code()
	if err != nil {
		return nil, err
	}
	permissions, err := to_permissions(req.Permissions)
	if err != nil {
		return nil, err
	}
	if _, err = p.enforcer.AddPermissionsForUser(subject, permissions...); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil

}
func (p *PolicyService) DeletePermissionsForUser(ctx context.Context, req *pb.PolicyPermissionsForUserRequest) (*emptypb.Empty, error) {
	{
		user, err := NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	user := pb.PolicyUserRequest{
		Id: req.User,
	}
	subject, err := user.Code()
	if err != nil {
		return nil, err
	}
	permissions, err := to_permissions(req.Permissions)
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
func (p *PolicyService) SetPermissionsForUser(ctx context.Context, req *pb.PolicyPermissionsForUserRequest) (*emptypb.Empty, error) {
	{
		user, err := NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	user := pb.PolicyUserRequest{
		Id: req.User,
	}
	subject, err := user.Code()
	if err != nil {
		return nil, err
	}
	if _, err := p.enforcer.DeletePermissionsForUser(subject); err != nil {
		return nil, err
	}
	permissions, err := to_permissions(req.Permissions)
	if err != nil {
		return nil, err
	}
	if _, err = p.enforcer.AddPermissionsForUser(subject, permissions...); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *PolicyService) GetPermissionsForUser(ctx context.Context, req *pb.PolicyUserRequest) (*pb.PolicyPermissionsResponse, error) {
	{
		user, err := NewCurrentUser(ctx, p.db, p.jwt)
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
	items, err := from_permissions(permissions)
	if err != nil {
		return nil, err
	}

	return &pb.PolicyPermissionsResponse{
		Items: items,
	}, nil
}

func (p *PolicyService) AddPermissionsForRole(ctx context.Context, req *pb.PolicyPermissionsForRoleRequest) (*emptypb.Empty, error) {
	{
		user, err := NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	role := pb.PolicyRolesResponse_Item{
		By: &pb.PolicyRolesResponse_Item_Member{
			Member: req.Role,
		},
	}
	subject, err := role.Code()
	if err != nil {
		return nil, err
	}
	permissions, err := to_permissions(req.Permissions)
	if err != nil {
		return nil, err
	}
	if _, err = p.enforcer.AddPermissionsForUser(subject, permissions...); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *PolicyService) DeletePermissionsForRole(ctx context.Context, req *pb.PolicyPermissionsForRoleRequest) (*emptypb.Empty, error) {
	{
		user, err := NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	role := pb.PolicyRolesResponse_Item{
		By: &pb.PolicyRolesResponse_Item_Member{
			Member: req.Role,
		},
	}
	subject, err := role.Code()
	if err != nil {
		return nil, err
	}
	permissions, err := to_permissions(req.Permissions)
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
func (p *PolicyService) SetPermissionsForRole(ctx context.Context, req *pb.PolicyPermissionsForRoleRequest) (*emptypb.Empty, error) {
	{
		user, err := NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	role := pb.PolicyRolesResponse_Item{
		By: &pb.PolicyRolesResponse_Item_Member{
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
	permissions, err := to_permissions(req.Permissions)
	if err != nil {
		return nil, err
	}
	if _, err = p.enforcer.AddPermissionsForUser(subject, permissions...); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *PolicyService) GetPermissionsForRole(ctx context.Context, req *pb.PolicyRoleRequest) (*pb.PolicyPermissionsResponse, error) {
	{
		user, err := NewCurrentUser(ctx, p.db, p.jwt)
		if err != nil {
			return nil, err
		}
		if err := user.Payload.IsAdministrator(p.enforcer); err != nil {
			return nil, err
		}
	}
	role := pb.PolicyRolesResponse_Item{
		By: &pb.PolicyRolesResponse_Item_Member{
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
	items, err := from_permissions(permissions)
	if err != nil {
		return nil, err
	}
	return &pb.PolicyPermissionsResponse{
		Items: items,
	}, nil
}

func NewPolicyService(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer) *PolicyService {
	return &PolicyService{db: db, jwt: jwt, enforcer: enforcer}
}

func from_permissions(permissions [][]string) ([]*pb.PolicyPermissionsResponse_Item, error) {
	var items []*pb.PolicyPermissionsResponse_Item
	for _, pm := range permissions {
		if len(pm) != 3 {
			slog.Error(fmt.Sprintf("unexpected permission %v", pm))
			continue
		}
		var it pb.PolicyPermissionsResponse_Item
		it.Operation = pm[2]
		resource, err := pb.NewPolicyResource(pm[1])
		if err != nil {
			return nil, err
		}
		it.Resource = resource
		items = append(items, &it)
	}
	return items, nil
}

func to_permissions(permissions []*pb.PolicyPermissionsResponse_Item) ([][]string, error) {
	var items [][]string
	for _, pm := range permissions {
		resource, err := pm.Resource.Code()
		if err != nil {
			return nil, err
		}
		items = append(items, []string{resource, pm.Operation})
	}
	return items, nil
}
