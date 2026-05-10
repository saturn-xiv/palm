import logging

from google.protobuf.empty_pb2 import Empty

from dahlia.protocols import rbac_pb2_grpc, rbac_pb2, from_str, to_str, role_from_str, user_from_str

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
        subject = rbac_pb2.Subject(user=request)
        for it in self.enforcer.get_implicit_roles_for_user(to_str(subject)):
            rol = role_from_str(it)
            res.items.extend([rol])
        return res

    def GetUsersForRole(self, request, context):
        # TODO
        return rbac_pb2.UsersResponse()

    def HasRoleForUser(self, request, context):
        # TODO
        return Empty()

    def AddRoleForUser(self, request, context):
        # TODO
        return Empty()

    def DeleteRoleForUser(self, request, context):
        # TODO
        return Empty()

    def DeleteRole(self, request, context):
        # TODO
        return Empty()

    def DeleteUser(self, request, context):
        # TODO
        return Empty()

    def GetPermissionsForUser(self, request, context):
        # TODO
        return rbac_pb2.PermissionsResponse

    def GetImplicitPermissionsForUser(self, request, context):
        # TODO
        return rbac_pb2.PermissionsResponse()

    def GetPermissionsForRole(self, request, context):
        # TODO
        return rbac_pb2.PermissionsResponse()

    def DeletePermission(self, request, context):
        # TODO
        return Empty()

    def AddPermission(self, request, context):
        # TODO
        return Empty()

    def HasPermission(self, request, context):
        # TODO
        return Empty()
