import logging

from google.protobuf.empty_pb2 import Empty

from dahlia.protocols import rbac_pb2_grpc, rbac_pb2

logger = logging.getLogger(__name__)


class Server(rbac_pb2_grpc.EnforcerServicer):
    def __init__(self, enforcer):
        self.enforcer = enforcer

    def GetAllSubjects(self, request, context):
        res = rbac_pb2.SubjectsResponse()
        for s in self.enforcer.get_all_subjects():
            it = res.items.add()
            # from_str(s, it)
        return res

    def GetAllObjects(self, request, context):
        res = rbac_pb2.ObjectsResponse()
        for s in self.enforcer.get_all_objects():
            it = res.items.add()
            # from_str(s, it)
        return res

    def GetAllActions(self, request, context):
        res = rbac_pb2.ActionsResponse()
        for s in self.enforcer.get_all_actions():
            it = res.items.add()
            # from_str(s, it)
        return res

    def GetAllRoles(self, request, context):
        res = rbac_pb2.RolesResponse()
        for s in self.enforcer.get_all_roles():
            it = res.items.add()
            # from_str(s, it)
        return res

    def GetRolesForUser(self, request, context):
        # TODO
        return rbac_pb2.RolesResponse()

    def GetImplicitRolesForUser(self, request, context):
        # TODO
        return rbac_pb2.RolesResponse()

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
