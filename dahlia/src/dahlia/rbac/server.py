import logging

from grpc import StatusCode

from dahlia.protocols import rbac_pb2_grpc, rbac_pb2, from_str, to_str, role_from_str, user_from_str, permission_from_line

logger = logging.getLogger(__name__)


class Server(rbac_pb2_grpc.EnforcerServicer):
    def __init__(self, enforcer):
        self.enforcer = enforcer

    def GetAllSubjects(self, request, context):
        res = rbac_pb2.SubjectsResponse()
        for it in self.enforcer.get_all_subjects():
            sub = res.items.add()
            from_str(it, sub)
        return res

    def GetAllObjects(self, request, context):
        res = rbac_pb2.ObjectsResponse()
        for it in self.enforcer.get_all_objects():
            obj = res.items.add()
            from_str(it, obj)
        return res

    def GetAllActions(self, request, context):
        res = rbac_pb2.ActionsResponse()
        for it in self.enforcer.get_all_actions():
            act = res.items.add()
            from_str(it, act)
        return res

    def GetAllRoles(self, request, context):
        res = rbac_pb2.RolesResponse()
        for it in self.enforcer.get_all_roles():
            rol = role_from_str(it, rol)
            res.items.extend([rol])
        return res

    def GetRolesForUser(self, request, context):
        res = rbac_pb2.RolesResponse()
        subject = rbac_pb2.Subject(user=request)
        for it in self.enforcer.get_roles_for_user(to_str(subject)):
            rol = role_from_str(it)
            res.items.extend([rol])
        return res

    def GetImplicitRolesForUser(self, request, context):
        res = rbac_pb2.RolesResponse()
        subject = to_str(rbac_pb2.Subject(user=request))
        for it in self.enforcer.get_implicit_roles_for_user(subject):
            rol = role_from_str(it)
            res.items.extend([rol])
        return res

    def GetUsersForRole(self, request, context):
        res = rbac_pb2.UsersResponse()
        subject = to_str(rbac_pb2.Subject(role=request))
        for it in self.enforcer.get_users_for_role(subject):
            usr = user_from_str(it)
            res.items.extend([usr])
        return res

    def GetImplicitUsersForRole(self, request, context):
        res = rbac_pb2.UsersResponse()
        subject = to_str(rbac_pb2.Subject(role=request))
        for it in self.enforcer.get_implicit_users_for_role(subject):
            usr = user_from_str(it)
            res.items.extend([usr])
        return res

    def HasRoleForUser(self, request, context):
        usr = to_str(rbac_pb2.Subject(user=request.user))
        rol = to_str(rbac_pb2.Subject(role=request.role))
        if rol in self.enforcer.get_implicit_roles_for_user(to_str(usr)):
            return rbac_pb2.Empty()
        context.abort(StatusCode.NOT_FOUND, "didn't have role")

    def AddRoleForUser(self, request, context):
        usr = to_str(rbac_pb2.Subject(user=request.user))
        rol = to_str(rbac_pb2.Subject(role=request.role))
        self.enforcer.add_role_for_user(usr, rol)
        return rbac_pb2.Empty()

    def DeleteRoleForUser(self, request, context):
        usr = to_str(rbac_pb2.Subject(user=request.user))
        rol = to_str(rbac_pb2.Subject(role=request.role))
        self.enforcer.delete_role_for_user(usr, rol)
        return rbac_pb2.Empty()

    def DeleteRole(self, request, context):
        rol = to_str(rbac_pb2.Subject(role=request.role))
        self.enforcer.delete_role(rol)
        return rbac_pb2.Empty()

    def DeleteUser(self, request, context):
        usr = to_str(rbac_pb2.Subject(user=request.user))
        self.enforcer.delete_user(usr)
        return rbac_pb2.Empty()

    def GetPermissions(self, request, context):
        res = rbac_pb2.PermissionsResponse()
        for it in self.enforcer.get_permissions_for_user(to_str(request)):
            pem = permission_from_line(it)
            res.items.extend([pem])
        return res

    def GetImplicitPermissions(self, request, context):
        res = rbac_pb2.PermissionsResponse()
        for it in self.enforcer.get_implicit_permissions_for_user(to_str(request)):
            pem = permission_from_line(it)
            res.items.extend([pem])
        return res

    def DeletePermission(self, request, context):
        self.enforcer.delete_permissions_for_user(
            to_str(request.subject), to_str(request.object), to_str(request.action))
        return rbac_pb2.Empty()

    def AddPermission(self, request, context):
        self.enforcer.add_permission_for_user(
            to_str(request.subject), to_str(request.object), to_str(request.action))
        return rbac_pb2.Empty()

    def HasPermission(self, request, context):
        sub = to_str(request.subject)
        obj = to_str(request.object)
        act = to_str(request.action)
        for rule in self.enforcer.get_implicit_permissions_for_user(sub):
            if len(rule) == 3 and obj == rule[1] and act == rule[2]:
                return rbac_pb2.Empty()
        context.abort(StatusCode.NOT_FOUND, "didn't have permission")
