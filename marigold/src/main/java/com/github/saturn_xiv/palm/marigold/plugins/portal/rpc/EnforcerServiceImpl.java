package com.github.saturn_xiv.palm.marigold.plugins.portal.rpc;

import org.springframework.grpc.server.service.GrpcService;

import com.github.saturn_xiv.palm.plugins.rbac.v1.ActionsResponse;
import com.github.saturn_xiv.palm.plugins.rbac.v1.EnforcerServiceGrpc.EnforcerServiceImplBase;
import com.github.saturn_xiv.palm.plugins.rbac.v1.ObjectsResponse;
import com.github.saturn_xiv.palm.plugins.rbac.v1.Permission;
import com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse;
import com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse;
import com.github.saturn_xiv.palm.plugins.rbac.v1.Subject;
import com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role;
import com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User;
import com.github.saturn_xiv.palm.plugins.rbac.v1.SubjectsResponse;
import com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest;
import com.github.saturn_xiv.palm.plugins.rbac.v1.UsersResponse;
import com.google.protobuf.Empty;

import io.grpc.stub.StreamObserver;

@GrpcService
public class EnforcerServiceImpl extends EnforcerServiceImplBase {

    @Override
    public void addPermission(Permission request, StreamObserver<Empty> responseObserver) {
        // TODO Auto-generated method stub
        super.addPermission(request, responseObserver);
    }

    @Override
    public void addRoleForUser(UserRoleRequest request, StreamObserver<Empty> responseObserver) {
        // TODO Auto-generated method stub
        super.addRoleForUser(request, responseObserver);
    }

    @Override
    public void deletePermission(Permission request, StreamObserver<Empty> responseObserver) {
        // TODO Auto-generated method stub
        super.deletePermission(request, responseObserver);
    }

    @Override
    public void deleteRole(Role request, StreamObserver<Empty> responseObserver) {
        // TODO Auto-generated method stub
        super.deleteRole(request, responseObserver);
    }

    @Override
    public void deleteRoleForUser(UserRoleRequest request, StreamObserver<Empty> responseObserver) {
        // TODO Auto-generated method stub
        super.deleteRoleForUser(request, responseObserver);
    }

    @Override
    public void deleteUser(User request, StreamObserver<Empty> responseObserver) {
        // TODO Auto-generated method stub
        super.deleteUser(request, responseObserver);
    }

    @Override
    public void getAllActions(Empty request, StreamObserver<ActionsResponse> responseObserver) {
        // TODO Auto-generated method stub
        super.getAllActions(request, responseObserver);
    }

    @Override
    public void getAllObjects(Empty request, StreamObserver<ObjectsResponse> responseObserver) {
        // TODO Auto-generated method stub
        super.getAllObjects(request, responseObserver);
    }

    @Override
    public void getAllRoles(Empty request, StreamObserver<RolesResponse> responseObserver) {
        // TODO Auto-generated method stub
        super.getAllRoles(request, responseObserver);
    }

    @Override
    public void getAllSubjects(Empty request, StreamObserver<SubjectsResponse> responseObserver) {
        // TODO Auto-generated method stub
        super.getAllSubjects(request, responseObserver);
    }

    @Override
    public void getImplicitPermissions(Subject request, StreamObserver<PermissionsResponse> responseObserver) {
        // TODO Auto-generated method stub
        super.getImplicitPermissions(request, responseObserver);
    }

    @Override
    public void getImplicitRolesForUser(User request, StreamObserver<RolesResponse> responseObserver) {
        // TODO Auto-generated method stub
        super.getImplicitRolesForUser(request, responseObserver);
    }

    @Override
    public void getImplicitUsersForRole(Role request, StreamObserver<UsersResponse> responseObserver) {
        // TODO Auto-generated method stub
        super.getImplicitUsersForRole(request, responseObserver);
    }

    @Override
    public void getPermissions(Subject request, StreamObserver<PermissionsResponse> responseObserver) {
        // TODO Auto-generated method stub
        super.getPermissions(request, responseObserver);
    }

    @Override
    public void getRolesForUser(User request, StreamObserver<RolesResponse> responseObserver) {
        // TODO Auto-generated method stub
        super.getRolesForUser(request, responseObserver);
    }

    @Override
    public void getUsersForRole(Role request, StreamObserver<UsersResponse> responseObserver) {
        // TODO Auto-generated method stub
        super.getUsersForRole(request, responseObserver);
    }

    @Override
    public void hasPermission(Permission request, StreamObserver<Empty> responseObserver) {
        // TODO Auto-generated method stub
        super.hasPermission(request, responseObserver);
    }

    @Override
    public void hasRoleForUser(UserRoleRequest request, StreamObserver<Empty> responseObserver) {
        // TODO Auto-generated method stub
        super.hasRoleForUser(request, responseObserver);
    }

}
