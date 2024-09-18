import logging

from google.protobuf import empty_pb2 as google_empty_pb2

from palm.rbac.v1 import rbac_pb2, rbac_pb2_grpc

logger = logging.getLogger(__name__)


class Service(rbac_pb2_grpc.PolicyServicer):
    def Has(self, request, context):
        # TODO
        return google_empty_pb2.Empty()

    def Can(self, request, context):
        # TODO
        return google_empty_pb2.Empty()

    def DeleteUser(self, request, context):
        # TODO
        return google_empty_pb2.Empty()

    def DeleteRole(self, request, context):
        # TODO
        return google_empty_pb2.Empty()

    def GetRolesForUser(self, request, context):
        # TODO
        return rbac_pb2.PolicyRolesResponse()

    def GetImplicitRolesForUser(self, request, context):
        # TODO
        return rbac_pb2.PolicyRolesResponse()

    def GetUsersForRole(self, request, context):
        # TODO
        return rbac_pb2.PolicyUsersResponse()

    def GetImplicitUsersForRole(self, request, context):
        # TODO
        return rbac_pb2.PolicyUsersResponse()

    def AddRolesForUser(self, request, context):
        # TODO
        return google_empty_pb2.Empty()

    def DeleteRolesForUser(self, request, context):
        # TODO
        return google_empty_pb2.Empty()

    def GetPermissionsForUser(self, request, context):
        # TODO
        return rbac_pb2.PolicyPermissionsResponse()

    def GetImplicitPermissionsForUser(self, request, context):
        # TODO
        return rbac_pb2.PolicyPermissionsResponse()

    def AddPermissionsForUser(self, request, context):
        # TODO
        return google_empty_pb2.Empty()

    def DeletePermissionsForUser(self, request, context):
        # TODO
        return google_empty_pb2.Empty()

    def GetPermissionsForRole(self, request, context):
        # TODO
        return rbac_pb2.PolicyPermissionsResponse()

    def GetImplicitPermissionsForRole(self, request, context):
        # TODO
        return rbac_pb2.PolicyPermissionsResponse()

    def AddPermissionsForRole(self, request, context):
        # TODO
        return google_empty_pb2.Empty()

    def DeletePermissionsForRole(self, request, context):
        # TODO
        return google_empty_pb2.Empty()
