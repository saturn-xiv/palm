package com.github.saturn_xiv.palm.marigold.plugins.portal.rpc;

import java.util.List;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.grpc.server.service.GrpcService;

import com.github.saturn_xiv.palm.plugins.rbac.v1.Action;
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
import com.github.saturn_xiv.palm.marigold.plugins.portal.utils.CasbinEnforcer;
import com.google.protobuf.Empty;
import com.google.protobuf.InvalidProtocolBufferException;

import io.grpc.Status;
import io.grpc.stub.StreamObserver;
import jakarta.annotation.Resource;

@GrpcService
public class EnforcerServiceImpl extends EnforcerServiceImplBase {

    @Override
    public void addPermission(Permission request, StreamObserver<Empty> responseObserver) {
        final var subject = CasbinEnforcer.message2string(request.getSubject());
        final var action = CasbinEnforcer.message2string(request.getAction());
        final var object = CasbinEnforcer.message2string(request.getObject());
        var reply = this.enforcer.execute((x) -> {
            if (x.getLeft().addPermissionForUser(subject, object, action)) {
                x.getRight().update();
            }
            return Empty.newBuilder().build();
        });
        responseObserver.onNext(reply);
        responseObserver.onCompleted();
    }

    @Override
    public void addRoleForUser(UserRoleRequest request, StreamObserver<Empty> responseObserver) {
        final var user = CasbinEnforcer.message2string(Subject.newBuilder().setUser(request.getUser()).build());
        final var role = CasbinEnforcer.message2string(Subject.newBuilder().setRole(request.getRole()).build());
        var reply = this.enforcer.execute((x) -> {
            if (x.getLeft().addRoleForUser(user, role)) {
                x.getRight().update();
            }
            return Empty.newBuilder().build();
        });
        responseObserver.onNext(reply);
        responseObserver.onCompleted();
    }

    @Override
    public void deletePermission(Permission request, StreamObserver<Empty> responseObserver) {
        final var subject = CasbinEnforcer.message2string(request.getSubject());
        final var action = CasbinEnforcer.message2string(request.getAction());
        final var object = CasbinEnforcer.message2string(request.getObject());
        var reply = this.enforcer.execute((x) -> {
            if (x.getLeft().deletePermission(subject, object, action)) {
                x.getRight().update();
            }
            return Empty.newBuilder().build();
        });
        responseObserver.onNext(reply);
        responseObserver.onCompleted();
    }

    @Override
    public void deleteRole(Role request, StreamObserver<Empty> responseObserver) {
        final var role = CasbinEnforcer.message2string(Subject.newBuilder().setRole(request).build());
        var reply = this.enforcer.execute((x) -> {
            x.getLeft().deleteRole(role);
            x.getRight().update();
            return Empty.newBuilder().build();
        });
        responseObserver.onNext(reply);
        responseObserver.onCompleted();
    }

    @Override
    public void deleteRoleForUser(UserRoleRequest request, StreamObserver<Empty> responseObserver) {
        final var user = CasbinEnforcer.message2string(Subject.newBuilder().setUser(request.getUser()).build());
        final var role = CasbinEnforcer.message2string(Subject.newBuilder().setRole(request.getRole()).build());
        var reply = this.enforcer.execute((x) -> {
            if (x.getLeft().deleteRoleForUser(user, role)) {
                x.getRight().update();
            }
            return Empty.newBuilder().build();
        });
        responseObserver.onNext(reply);
        responseObserver.onCompleted();
    }

    @Override
    public void deleteUser(User request, StreamObserver<Empty> responseObserver) {
        final var user = CasbinEnforcer.message2string(Subject.newBuilder().setUser(request).build());
        var reply = this.enforcer.execute((x) -> {
            if (x.getLeft().deleteUser(user)) {
                x.getRight().update();
            }
            return Empty.newBuilder().build();
        });
        responseObserver.onNext(reply);
        responseObserver.onCompleted();
    }

    @Override
    public void getAllActions(Empty request, StreamObserver<ActionsResponse> responseObserver) {
        var actions = this.enforcer.execute((x) -> {
            return x.getLeft().getAllActions();
        });

        var builder = ActionsResponse.newBuilder();
        try {
            for (var x : actions) {
                builder.addItems(CasbinEnforcer.string2message(x, Action.parser()));
            }
        } catch (InvalidProtocolBufferException e) {
            responseObserver.onError(Status.INVALID_ARGUMENT.withCause(e).asRuntimeException());
            return;
        }
        responseObserver.onNext(builder.build());
        responseObserver.onCompleted();
    }

    @Override
    public void getAllObjects(Empty request, StreamObserver<ObjectsResponse> responseObserver) {
        var objects = this.enforcer.execute((x) -> {
            return x.getLeft().getAllObjects();
        });

        var builder = ObjectsResponse.newBuilder();
        try {
            for (var x : objects) {
                builder.addItems(
                        CasbinEnforcer.string2message(x, com.github.saturn_xiv.palm.plugins.rbac.v1.Object.parser()));
            }
        } catch (InvalidProtocolBufferException e) {
            responseObserver.onError(Status.INVALID_ARGUMENT.withCause(e).asRuntimeException());
            return;
        }
        responseObserver.onNext(builder.build());
        responseObserver.onCompleted();
    }

    @Override
    public void getAllRoles(Empty request, StreamObserver<RolesResponse> responseObserver) {
        var roles = this.enforcer.execute((x) -> {
            return x.getLeft().getAllRoles();
        });

        var builder = RolesResponse.newBuilder();
        try {
            for (var x : roles) {
                var it = CasbinEnforcer.string2message(x, Subject.parser());
                if (!it.hasRole()) {
                    logger.error("invalid role subject {}", x);
                    continue;
                }
                builder.addItems(it.getRole());
            }
        } catch (InvalidProtocolBufferException e) {
            responseObserver.onError(Status.INVALID_ARGUMENT.withCause(e).asRuntimeException());
            return;
        }
        responseObserver.onNext(builder.build());
        responseObserver.onCompleted();
    }

    @Override
    public void getAllSubjects(Empty request, StreamObserver<SubjectsResponse> responseObserver) {
        var subjects = this.enforcer.execute((x) -> {
            return x.getLeft().getAllSubjects();
        });

        var builder = SubjectsResponse.newBuilder();
        try {
            for (var x : subjects) {
                builder.addItems(CasbinEnforcer.string2message(x, Subject.parser()));
            }
        } catch (InvalidProtocolBufferException e) {
            responseObserver.onError(Status.INVALID_ARGUMENT.withCause(e).asRuntimeException());
            return;
        }
        responseObserver.onNext(builder.build());
        responseObserver.onCompleted();
    }

    @Override
    public void getImplicitPermissions(Subject request, StreamObserver<PermissionsResponse> responseObserver) {
        final var user = CasbinEnforcer.message2string(request);
        var permissions = this.enforcer.execute((x) -> {
            return x.getLeft().getImplicitPermissionsForUser(user);
        });
        this.permissions(permissions, responseObserver);
    }

    @Override
    public void getImplicitRolesForUser(User request, StreamObserver<RolesResponse> responseObserver) {
        final var user = CasbinEnforcer.message2string(request);
        var roles = this.enforcer.execute((x) -> {
            return x.getLeft().getImplicitRolesForUser(user);
        });
        this.roles(roles, responseObserver);
    }

    @Override
    public void getImplicitUsersForRole(Role request, StreamObserver<UsersResponse> responseObserver) {
        final var role = CasbinEnforcer.message2string(request);
        var users = this.enforcer.execute((x) -> {
            return x.getLeft().getImplicitUsersForRole(role);
        });
        this.users(users, responseObserver);
    }

    @Override
    public void getPermissions(Subject request, StreamObserver<PermissionsResponse> responseObserver) {
        final var user = CasbinEnforcer.message2string(request);
        var permissions = this.enforcer.execute((x) -> {
            return x.getLeft().getPermissionsForUser(user);
        });
        this.permissions(permissions, responseObserver);
    }

    @Override
    public void getRolesForUser(User request, StreamObserver<RolesResponse> responseObserver) {
        final var user = CasbinEnforcer.message2string(request);
        var roles = this.enforcer.execute((x) -> {
            return x.getLeft().getRolesForUser(user);
        });
        this.roles(roles, responseObserver);
    }

    @Override
    public void getUsersForRole(Role request, StreamObserver<UsersResponse> responseObserver) {
        final var role = CasbinEnforcer.message2string(request);
        var users = this.enforcer.execute((x) -> {
            return x.getLeft().getUsersForRole(role);
        });
        this.users(users, responseObserver);
    }

    @Override
    public void hasPermission(Permission request, StreamObserver<Empty> responseObserver) {
        final var user = CasbinEnforcer.message2string(request.getSubject());
        final var object = CasbinEnforcer.message2string(request.getObject());
        final var action = CasbinEnforcer.message2string(request.getAction());
        var permissions = this.enforcer.execute((x) -> {
            return x.getLeft().getImplicitPermissionsForUser(user);
        });
        for (var x : permissions) {
            if (x.size() == 3 && object.equals(x.get(1)) && action.equals(x.get(2))) {
                responseObserver.onNext(Empty.newBuilder().build());
                responseObserver.onCompleted();
                return;
            }
        }

        responseObserver.onError(Status.PERMISSION_DENIED.asRuntimeException());
    }

    @Override
    public void hasRoleForUser(UserRoleRequest request, StreamObserver<Empty> responseObserver) {
        final var user = CasbinEnforcer.message2string(request.getUser());
        final var role = CasbinEnforcer.message2string(request.getRole());
        var roles = this.enforcer.execute((x) -> {
            return x.getLeft().getImplicitRolesForUser(user);
        });
        if (roles.contains(role)) {
            responseObserver.onNext(Empty.newBuilder().build());
            responseObserver.onCompleted();
            return;
        }
        responseObserver.onError(Status.PERMISSION_DENIED.asRuntimeException());
    }

    private void roles(List<String> roles, StreamObserver<RolesResponse> responseObserver) {
        var builder = RolesResponse.newBuilder();
        try {
            for (var x : roles) {
                var it = CasbinEnforcer.string2message(x, Subject.parser());
                if (!it.hasRole()) {
                    logger.error("invalid role {}", x);
                }
                builder.addItems(it.getRole());
            }
        } catch (InvalidProtocolBufferException e) {
            responseObserver.onError(Status.INVALID_ARGUMENT.withCause(e).asRuntimeException());
            return;
        }
        responseObserver.onNext(builder.build());
        responseObserver.onCompleted();
    }

    void users(List<String> users, StreamObserver<UsersResponse> responseObserver) {
        var builder = UsersResponse.newBuilder();
        try {
            for (var x : users) {
                var it = CasbinEnforcer.string2message(x, Subject.parser());
                if (!it.hasUser()) {
                    logger.error("invalid user {}", x);
                }
                builder.addItems(it.getUser());
            }
        } catch (InvalidProtocolBufferException e) {
            responseObserver.onError(Status.INVALID_ARGUMENT.withCause(e).asRuntimeException());
            return;
        }
        responseObserver.onNext(builder.build());
        responseObserver.onCompleted();
    }

    private void permissions(List<List<String>> permissions, StreamObserver<PermissionsResponse> responseObserver) {
        var builder = PermissionsResponse.newBuilder();
        try {
            for (var x : permissions) {
                if (x.size() != 3) {
                    logger.error("invalid permission rule ({})", String.join(",", x));
                }
                builder.addItems(Permission.newBuilder()
                        .setSubject(CasbinEnforcer.string2message(x.get(0), Subject.parser()))
                        .setObject(CasbinEnforcer.string2message(x.get(1),
                                com.github.saturn_xiv.palm.plugins.rbac.v1.Object.parser()))
                        .setAction(CasbinEnforcer.string2message(x.get(0), Action.parser()))
                        .build());
            }
        } catch (InvalidProtocolBufferException e) {
            responseObserver.onError(Status.INVALID_ARGUMENT.withCause(e).asRuntimeException());
            return;
        }
        responseObserver.onNext(builder.build());
        responseObserver.onCompleted();
    }

    @Resource
    CasbinEnforcer enforcer;

    private final static Logger logger = LoggerFactory.getLogger(EnforcerServiceImpl.class);

}
