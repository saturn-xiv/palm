package com.github.saturn_xiv.palm.plugins.rbac.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class EnforcerGrpc {

  private EnforcerGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.rbac.v1.Enforcer";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.rbac.v1.SubjectsResponse> getGetAllSubjectsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetAllSubjects",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.rbac.v1.SubjectsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.rbac.v1.SubjectsResponse> getGetAllSubjectsMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.rbac.v1.SubjectsResponse> getGetAllSubjectsMethod;
    if ((getGetAllSubjectsMethod = EnforcerGrpc.getGetAllSubjectsMethod) == null) {
      synchronized (EnforcerGrpc.class) {
        if ((getGetAllSubjectsMethod = EnforcerGrpc.getGetAllSubjectsMethod) == null) {
          EnforcerGrpc.getGetAllSubjectsMethod = getGetAllSubjectsMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.rbac.v1.SubjectsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetAllSubjects"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.rbac.v1.SubjectsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new EnforcerMethodDescriptorSupplier("GetAllSubjects"))
              .build();
        }
      }
    }
    return getGetAllSubjectsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.rbac.v1.ObjectsResponse> getGetAllObjectsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetAllObjects",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.rbac.v1.ObjectsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.rbac.v1.ObjectsResponse> getGetAllObjectsMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.rbac.v1.ObjectsResponse> getGetAllObjectsMethod;
    if ((getGetAllObjectsMethod = EnforcerGrpc.getGetAllObjectsMethod) == null) {
      synchronized (EnforcerGrpc.class) {
        if ((getGetAllObjectsMethod = EnforcerGrpc.getGetAllObjectsMethod) == null) {
          EnforcerGrpc.getGetAllObjectsMethod = getGetAllObjectsMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.rbac.v1.ObjectsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetAllObjects"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.rbac.v1.ObjectsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new EnforcerMethodDescriptorSupplier("GetAllObjects"))
              .build();
        }
      }
    }
    return getGetAllObjectsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.rbac.v1.ActionsResponse> getGetAllActionsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetAllActions",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.rbac.v1.ActionsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.rbac.v1.ActionsResponse> getGetAllActionsMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.rbac.v1.ActionsResponse> getGetAllActionsMethod;
    if ((getGetAllActionsMethod = EnforcerGrpc.getGetAllActionsMethod) == null) {
      synchronized (EnforcerGrpc.class) {
        if ((getGetAllActionsMethod = EnforcerGrpc.getGetAllActionsMethod) == null) {
          EnforcerGrpc.getGetAllActionsMethod = getGetAllActionsMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.rbac.v1.ActionsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetAllActions"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.rbac.v1.ActionsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new EnforcerMethodDescriptorSupplier("GetAllActions"))
              .build();
        }
      }
    }
    return getGetAllActionsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse> getGetAllRolesMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetAllRoles",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse> getGetAllRolesMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse> getGetAllRolesMethod;
    if ((getGetAllRolesMethod = EnforcerGrpc.getGetAllRolesMethod) == null) {
      synchronized (EnforcerGrpc.class) {
        if ((getGetAllRolesMethod = EnforcerGrpc.getGetAllRolesMethod) == null) {
          EnforcerGrpc.getGetAllRolesMethod = getGetAllRolesMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetAllRoles"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new EnforcerMethodDescriptorSupplier("GetAllRoles"))
              .build();
        }
      }
    }
    return getGetAllRolesMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User,
      com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse> getGetRolesForUserMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetRolesForUser",
      requestType = com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User.class,
      responseType = com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User,
      com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse> getGetRolesForUserMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User, com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse> getGetRolesForUserMethod;
    if ((getGetRolesForUserMethod = EnforcerGrpc.getGetRolesForUserMethod) == null) {
      synchronized (EnforcerGrpc.class) {
        if ((getGetRolesForUserMethod = EnforcerGrpc.getGetRolesForUserMethod) == null) {
          EnforcerGrpc.getGetRolesForUserMethod = getGetRolesForUserMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User, com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetRolesForUser"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new EnforcerMethodDescriptorSupplier("GetRolesForUser"))
              .build();
        }
      }
    }
    return getGetRolesForUserMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User,
      com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse> getGetImplicitRolesForUserMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetImplicitRolesForUser",
      requestType = com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User.class,
      responseType = com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User,
      com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse> getGetImplicitRolesForUserMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User, com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse> getGetImplicitRolesForUserMethod;
    if ((getGetImplicitRolesForUserMethod = EnforcerGrpc.getGetImplicitRolesForUserMethod) == null) {
      synchronized (EnforcerGrpc.class) {
        if ((getGetImplicitRolesForUserMethod = EnforcerGrpc.getGetImplicitRolesForUserMethod) == null) {
          EnforcerGrpc.getGetImplicitRolesForUserMethod = getGetImplicitRolesForUserMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User, com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetImplicitRolesForUser"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new EnforcerMethodDescriptorSupplier("GetImplicitRolesForUser"))
              .build();
        }
      }
    }
    return getGetImplicitRolesForUserMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role,
      com.github.saturn_xiv.palm.plugins.rbac.v1.UsersResponse> getGetUsersForRoleMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetUsersForRole",
      requestType = com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role.class,
      responseType = com.github.saturn_xiv.palm.plugins.rbac.v1.UsersResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role,
      com.github.saturn_xiv.palm.plugins.rbac.v1.UsersResponse> getGetUsersForRoleMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role, com.github.saturn_xiv.palm.plugins.rbac.v1.UsersResponse> getGetUsersForRoleMethod;
    if ((getGetUsersForRoleMethod = EnforcerGrpc.getGetUsersForRoleMethod) == null) {
      synchronized (EnforcerGrpc.class) {
        if ((getGetUsersForRoleMethod = EnforcerGrpc.getGetUsersForRoleMethod) == null) {
          EnforcerGrpc.getGetUsersForRoleMethod = getGetUsersForRoleMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role, com.github.saturn_xiv.palm.plugins.rbac.v1.UsersResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetUsersForRole"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.rbac.v1.UsersResponse.getDefaultInstance()))
              .setSchemaDescriptor(new EnforcerMethodDescriptorSupplier("GetUsersForRole"))
              .build();
        }
      }
    }
    return getGetUsersForRoleMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest,
      com.google.protobuf.Empty> getHasRoleForUserMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "HasRoleForUser",
      requestType = com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest,
      com.google.protobuf.Empty> getHasRoleForUserMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest, com.google.protobuf.Empty> getHasRoleForUserMethod;
    if ((getHasRoleForUserMethod = EnforcerGrpc.getHasRoleForUserMethod) == null) {
      synchronized (EnforcerGrpc.class) {
        if ((getHasRoleForUserMethod = EnforcerGrpc.getHasRoleForUserMethod) == null) {
          EnforcerGrpc.getHasRoleForUserMethod = getHasRoleForUserMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "HasRoleForUser"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new EnforcerMethodDescriptorSupplier("HasRoleForUser"))
              .build();
        }
      }
    }
    return getHasRoleForUserMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest,
      com.google.protobuf.Empty> getAddRoleForUserMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "AddRoleForUser",
      requestType = com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest,
      com.google.protobuf.Empty> getAddRoleForUserMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest, com.google.protobuf.Empty> getAddRoleForUserMethod;
    if ((getAddRoleForUserMethod = EnforcerGrpc.getAddRoleForUserMethod) == null) {
      synchronized (EnforcerGrpc.class) {
        if ((getAddRoleForUserMethod = EnforcerGrpc.getAddRoleForUserMethod) == null) {
          EnforcerGrpc.getAddRoleForUserMethod = getAddRoleForUserMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "AddRoleForUser"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new EnforcerMethodDescriptorSupplier("AddRoleForUser"))
              .build();
        }
      }
    }
    return getAddRoleForUserMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest,
      com.google.protobuf.Empty> getDeleteRoleForUserMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeleteRoleForUser",
      requestType = com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest,
      com.google.protobuf.Empty> getDeleteRoleForUserMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest, com.google.protobuf.Empty> getDeleteRoleForUserMethod;
    if ((getDeleteRoleForUserMethod = EnforcerGrpc.getDeleteRoleForUserMethod) == null) {
      synchronized (EnforcerGrpc.class) {
        if ((getDeleteRoleForUserMethod = EnforcerGrpc.getDeleteRoleForUserMethod) == null) {
          EnforcerGrpc.getDeleteRoleForUserMethod = getDeleteRoleForUserMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeleteRoleForUser"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new EnforcerMethodDescriptorSupplier("DeleteRoleForUser"))
              .build();
        }
      }
    }
    return getDeleteRoleForUserMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User,
      com.google.protobuf.Empty> getDeleteUserMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeleteUser",
      requestType = com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User,
      com.google.protobuf.Empty> getDeleteUserMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User, com.google.protobuf.Empty> getDeleteUserMethod;
    if ((getDeleteUserMethod = EnforcerGrpc.getDeleteUserMethod) == null) {
      synchronized (EnforcerGrpc.class) {
        if ((getDeleteUserMethod = EnforcerGrpc.getDeleteUserMethod) == null) {
          EnforcerGrpc.getDeleteUserMethod = getDeleteUserMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeleteUser"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new EnforcerMethodDescriptorSupplier("DeleteUser"))
              .build();
        }
      }
    }
    return getDeleteUserMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role,
      com.google.protobuf.Empty> getDeleteRoleMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeleteRole",
      requestType = com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role,
      com.google.protobuf.Empty> getDeleteRoleMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role, com.google.protobuf.Empty> getDeleteRoleMethod;
    if ((getDeleteRoleMethod = EnforcerGrpc.getDeleteRoleMethod) == null) {
      synchronized (EnforcerGrpc.class) {
        if ((getDeleteRoleMethod = EnforcerGrpc.getDeleteRoleMethod) == null) {
          EnforcerGrpc.getDeleteRoleMethod = getDeleteRoleMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeleteRole"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new EnforcerMethodDescriptorSupplier("DeleteRole"))
              .build();
        }
      }
    }
    return getDeleteRoleMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User,
      com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse> getGetPermissionsForUserMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetPermissionsForUser",
      requestType = com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User.class,
      responseType = com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User,
      com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse> getGetPermissionsForUserMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User, com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse> getGetPermissionsForUserMethod;
    if ((getGetPermissionsForUserMethod = EnforcerGrpc.getGetPermissionsForUserMethod) == null) {
      synchronized (EnforcerGrpc.class) {
        if ((getGetPermissionsForUserMethod = EnforcerGrpc.getGetPermissionsForUserMethod) == null) {
          EnforcerGrpc.getGetPermissionsForUserMethod = getGetPermissionsForUserMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User, com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetPermissionsForUser"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new EnforcerMethodDescriptorSupplier("GetPermissionsForUser"))
              .build();
        }
      }
    }
    return getGetPermissionsForUserMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User,
      com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse> getGetImplicitPermissionsForUserMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetImplicitPermissionsForUser",
      requestType = com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User.class,
      responseType = com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User,
      com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse> getGetImplicitPermissionsForUserMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User, com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse> getGetImplicitPermissionsForUserMethod;
    if ((getGetImplicitPermissionsForUserMethod = EnforcerGrpc.getGetImplicitPermissionsForUserMethod) == null) {
      synchronized (EnforcerGrpc.class) {
        if ((getGetImplicitPermissionsForUserMethod = EnforcerGrpc.getGetImplicitPermissionsForUserMethod) == null) {
          EnforcerGrpc.getGetImplicitPermissionsForUserMethod = getGetImplicitPermissionsForUserMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User, com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetImplicitPermissionsForUser"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new EnforcerMethodDescriptorSupplier("GetImplicitPermissionsForUser"))
              .build();
        }
      }
    }
    return getGetImplicitPermissionsForUserMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role,
      com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse> getGetPermissionsForRoleMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetPermissionsForRole",
      requestType = com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role.class,
      responseType = com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role,
      com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse> getGetPermissionsForRoleMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role, com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse> getGetPermissionsForRoleMethod;
    if ((getGetPermissionsForRoleMethod = EnforcerGrpc.getGetPermissionsForRoleMethod) == null) {
      synchronized (EnforcerGrpc.class) {
        if ((getGetPermissionsForRoleMethod = EnforcerGrpc.getGetPermissionsForRoleMethod) == null) {
          EnforcerGrpc.getGetPermissionsForRoleMethod = getGetPermissionsForRoleMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role, com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetPermissionsForRole"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new EnforcerMethodDescriptorSupplier("GetPermissionsForRole"))
              .build();
        }
      }
    }
    return getGetPermissionsForRoleMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Permission,
      com.google.protobuf.Empty> getDeletePermissionMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeletePermission",
      requestType = com.github.saturn_xiv.palm.plugins.rbac.v1.Permission.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Permission,
      com.google.protobuf.Empty> getDeletePermissionMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Permission, com.google.protobuf.Empty> getDeletePermissionMethod;
    if ((getDeletePermissionMethod = EnforcerGrpc.getDeletePermissionMethod) == null) {
      synchronized (EnforcerGrpc.class) {
        if ((getDeletePermissionMethod = EnforcerGrpc.getDeletePermissionMethod) == null) {
          EnforcerGrpc.getDeletePermissionMethod = getDeletePermissionMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.rbac.v1.Permission, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeletePermission"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.rbac.v1.Permission.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new EnforcerMethodDescriptorSupplier("DeletePermission"))
              .build();
        }
      }
    }
    return getDeletePermissionMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Permission,
      com.google.protobuf.Empty> getAddPermissionMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "AddPermission",
      requestType = com.github.saturn_xiv.palm.plugins.rbac.v1.Permission.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Permission,
      com.google.protobuf.Empty> getAddPermissionMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Permission, com.google.protobuf.Empty> getAddPermissionMethod;
    if ((getAddPermissionMethod = EnforcerGrpc.getAddPermissionMethod) == null) {
      synchronized (EnforcerGrpc.class) {
        if ((getAddPermissionMethod = EnforcerGrpc.getAddPermissionMethod) == null) {
          EnforcerGrpc.getAddPermissionMethod = getAddPermissionMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.rbac.v1.Permission, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "AddPermission"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.rbac.v1.Permission.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new EnforcerMethodDescriptorSupplier("AddPermission"))
              .build();
        }
      }
    }
    return getAddPermissionMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Permission,
      com.google.protobuf.Empty> getHasPermissionMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "HasPermission",
      requestType = com.github.saturn_xiv.palm.plugins.rbac.v1.Permission.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Permission,
      com.google.protobuf.Empty> getHasPermissionMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.rbac.v1.Permission, com.google.protobuf.Empty> getHasPermissionMethod;
    if ((getHasPermissionMethod = EnforcerGrpc.getHasPermissionMethod) == null) {
      synchronized (EnforcerGrpc.class) {
        if ((getHasPermissionMethod = EnforcerGrpc.getHasPermissionMethod) == null) {
          EnforcerGrpc.getHasPermissionMethod = getHasPermissionMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.rbac.v1.Permission, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "HasPermission"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.rbac.v1.Permission.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new EnforcerMethodDescriptorSupplier("HasPermission"))
              .build();
        }
      }
    }
    return getHasPermissionMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static EnforcerStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<EnforcerStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<EnforcerStub>() {
        @java.lang.Override
        public EnforcerStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new EnforcerStub(channel, callOptions);
        }
      };
    return EnforcerStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static EnforcerBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<EnforcerBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<EnforcerBlockingV2Stub>() {
        @java.lang.Override
        public EnforcerBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new EnforcerBlockingV2Stub(channel, callOptions);
        }
      };
    return EnforcerBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static EnforcerBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<EnforcerBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<EnforcerBlockingStub>() {
        @java.lang.Override
        public EnforcerBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new EnforcerBlockingStub(channel, callOptions);
        }
      };
    return EnforcerBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static EnforcerFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<EnforcerFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<EnforcerFutureStub>() {
        @java.lang.Override
        public EnforcerFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new EnforcerFutureStub(channel, callOptions);
        }
      };
    return EnforcerFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void getAllSubjects(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.SubjectsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetAllSubjectsMethod(), responseObserver);
    }

    /**
     */
    default void getAllObjects(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.ObjectsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetAllObjectsMethod(), responseObserver);
    }

    /**
     */
    default void getAllActions(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.ActionsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetAllActionsMethod(), responseObserver);
    }

    /**
     */
    default void getAllRoles(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetAllRolesMethod(), responseObserver);
    }

    /**
     */
    default void getRolesForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetRolesForUserMethod(), responseObserver);
    }

    /**
     */
    default void getImplicitRolesForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetImplicitRolesForUserMethod(), responseObserver);
    }

    /**
     */
    default void getUsersForRole(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.UsersResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetUsersForRoleMethod(), responseObserver);
    }

    /**
     */
    default void hasRoleForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getHasRoleForUserMethod(), responseObserver);
    }

    /**
     */
    default void addRoleForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getAddRoleForUserMethod(), responseObserver);
    }

    /**
     */
    default void deleteRoleForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeleteRoleForUserMethod(), responseObserver);
    }

    /**
     */
    default void deleteUser(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeleteUserMethod(), responseObserver);
    }

    /**
     */
    default void deleteRole(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeleteRoleMethod(), responseObserver);
    }

    /**
     */
    default void getPermissionsForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetPermissionsForUserMethod(), responseObserver);
    }

    /**
     */
    default void getImplicitPermissionsForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetImplicitPermissionsForUserMethod(), responseObserver);
    }

    /**
     */
    default void getPermissionsForRole(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetPermissionsForRoleMethod(), responseObserver);
    }

    /**
     */
    default void deletePermission(com.github.saturn_xiv.palm.plugins.rbac.v1.Permission request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeletePermissionMethod(), responseObserver);
    }

    /**
     */
    default void addPermission(com.github.saturn_xiv.palm.plugins.rbac.v1.Permission request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getAddPermissionMethod(), responseObserver);
    }

    /**
     */
    default void hasPermission(com.github.saturn_xiv.palm.plugins.rbac.v1.Permission request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getHasPermissionMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service Enforcer.
   */
  public static abstract class EnforcerImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return EnforcerGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Enforcer.
   */
  public static final class EnforcerStub
      extends io.grpc.stub.AbstractAsyncStub<EnforcerStub> {
    private EnforcerStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected EnforcerStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new EnforcerStub(channel, callOptions);
    }

    /**
     */
    public void getAllSubjects(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.SubjectsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetAllSubjectsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getAllObjects(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.ObjectsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetAllObjectsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getAllActions(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.ActionsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetAllActionsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getAllRoles(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetAllRolesMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getRolesForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetRolesForUserMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getImplicitRolesForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetImplicitRolesForUserMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getUsersForRole(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.UsersResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetUsersForRoleMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void hasRoleForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getHasRoleForUserMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void addRoleForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getAddRoleForUserMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void deleteRoleForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeleteRoleForUserMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void deleteUser(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeleteUserMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void deleteRole(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeleteRoleMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getPermissionsForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetPermissionsForUserMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getImplicitPermissionsForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetImplicitPermissionsForUserMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getPermissionsForRole(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetPermissionsForRoleMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void deletePermission(com.github.saturn_xiv.palm.plugins.rbac.v1.Permission request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeletePermissionMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void addPermission(com.github.saturn_xiv.palm.plugins.rbac.v1.Permission request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getAddPermissionMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void hasPermission(com.github.saturn_xiv.palm.plugins.rbac.v1.Permission request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getHasPermissionMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Enforcer.
   */
  public static final class EnforcerBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<EnforcerBlockingV2Stub> {
    private EnforcerBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected EnforcerBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new EnforcerBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.rbac.v1.SubjectsResponse getAllSubjects(com.google.protobuf.Empty request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getGetAllSubjectsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.rbac.v1.ObjectsResponse getAllObjects(com.google.protobuf.Empty request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getGetAllObjectsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.rbac.v1.ActionsResponse getAllActions(com.google.protobuf.Empty request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getGetAllActionsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse getAllRoles(com.google.protobuf.Empty request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getGetAllRolesMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse getRolesForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getGetRolesForUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse getImplicitRolesForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getGetImplicitRolesForUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.rbac.v1.UsersResponse getUsersForRole(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getGetUsersForRoleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty hasRoleForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getHasRoleForUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty addRoleForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getAddRoleForUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deleteRoleForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getDeleteRoleForUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deleteUser(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getDeleteUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deleteRole(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getDeleteRoleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse getPermissionsForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getGetPermissionsForUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse getImplicitPermissionsForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getGetImplicitPermissionsForUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse getPermissionsForRole(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getGetPermissionsForRoleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deletePermission(com.github.saturn_xiv.palm.plugins.rbac.v1.Permission request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getDeletePermissionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty addPermission(com.github.saturn_xiv.palm.plugins.rbac.v1.Permission request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getAddPermissionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty hasPermission(com.github.saturn_xiv.palm.plugins.rbac.v1.Permission request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getHasPermissionMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Enforcer.
   */
  public static final class EnforcerBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<EnforcerBlockingStub> {
    private EnforcerBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected EnforcerBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new EnforcerBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.rbac.v1.SubjectsResponse getAllSubjects(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetAllSubjectsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.rbac.v1.ObjectsResponse getAllObjects(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetAllObjectsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.rbac.v1.ActionsResponse getAllActions(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetAllActionsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse getAllRoles(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetAllRolesMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse getRolesForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetRolesForUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse getImplicitRolesForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetImplicitRolesForUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.rbac.v1.UsersResponse getUsersForRole(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetUsersForRoleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty hasRoleForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getHasRoleForUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty addRoleForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getAddRoleForUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deleteRoleForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeleteRoleForUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deleteUser(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeleteUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deleteRole(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeleteRoleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse getPermissionsForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetPermissionsForUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse getImplicitPermissionsForUser(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetImplicitPermissionsForUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse getPermissionsForRole(com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetPermissionsForRoleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deletePermission(com.github.saturn_xiv.palm.plugins.rbac.v1.Permission request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeletePermissionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty addPermission(com.github.saturn_xiv.palm.plugins.rbac.v1.Permission request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getAddPermissionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty hasPermission(com.github.saturn_xiv.palm.plugins.rbac.v1.Permission request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getHasPermissionMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Enforcer.
   */
  public static final class EnforcerFutureStub
      extends io.grpc.stub.AbstractFutureStub<EnforcerFutureStub> {
    private EnforcerFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected EnforcerFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new EnforcerFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.rbac.v1.SubjectsResponse> getAllSubjects(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetAllSubjectsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.rbac.v1.ObjectsResponse> getAllObjects(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetAllObjectsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.rbac.v1.ActionsResponse> getAllActions(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetAllActionsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse> getAllRoles(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetAllRolesMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse> getRolesForUser(
        com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetRolesForUserMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse> getImplicitRolesForUser(
        com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetImplicitRolesForUserMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.rbac.v1.UsersResponse> getUsersForRole(
        com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetUsersForRoleMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> hasRoleForUser(
        com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getHasRoleForUserMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> addRoleForUser(
        com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getAddRoleForUserMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> deleteRoleForUser(
        com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeleteRoleForUserMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> deleteUser(
        com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeleteUserMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> deleteRole(
        com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeleteRoleMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse> getPermissionsForUser(
        com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetPermissionsForUserMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse> getImplicitPermissionsForUser(
        com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetImplicitPermissionsForUserMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse> getPermissionsForRole(
        com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetPermissionsForRoleMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> deletePermission(
        com.github.saturn_xiv.palm.plugins.rbac.v1.Permission request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeletePermissionMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> addPermission(
        com.github.saturn_xiv.palm.plugins.rbac.v1.Permission request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getAddPermissionMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> hasPermission(
        com.github.saturn_xiv.palm.plugins.rbac.v1.Permission request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getHasPermissionMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_GET_ALL_SUBJECTS = 0;
  private static final int METHODID_GET_ALL_OBJECTS = 1;
  private static final int METHODID_GET_ALL_ACTIONS = 2;
  private static final int METHODID_GET_ALL_ROLES = 3;
  private static final int METHODID_GET_ROLES_FOR_USER = 4;
  private static final int METHODID_GET_IMPLICIT_ROLES_FOR_USER = 5;
  private static final int METHODID_GET_USERS_FOR_ROLE = 6;
  private static final int METHODID_HAS_ROLE_FOR_USER = 7;
  private static final int METHODID_ADD_ROLE_FOR_USER = 8;
  private static final int METHODID_DELETE_ROLE_FOR_USER = 9;
  private static final int METHODID_DELETE_USER = 10;
  private static final int METHODID_DELETE_ROLE = 11;
  private static final int METHODID_GET_PERMISSIONS_FOR_USER = 12;
  private static final int METHODID_GET_IMPLICIT_PERMISSIONS_FOR_USER = 13;
  private static final int METHODID_GET_PERMISSIONS_FOR_ROLE = 14;
  private static final int METHODID_DELETE_PERMISSION = 15;
  private static final int METHODID_ADD_PERMISSION = 16;
  private static final int METHODID_HAS_PERMISSION = 17;

  private static final class MethodHandlers<Req, Resp> implements
      io.grpc.stub.ServerCalls.UnaryMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.ServerStreamingMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.ClientStreamingMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.BidiStreamingMethod<Req, Resp> {
    private final AsyncService serviceImpl;
    private final int methodId;

    MethodHandlers(AsyncService serviceImpl, int methodId) {
      this.serviceImpl = serviceImpl;
      this.methodId = methodId;
    }

    @java.lang.Override
    @java.lang.SuppressWarnings("unchecked")
    public void invoke(Req request, io.grpc.stub.StreamObserver<Resp> responseObserver) {
      switch (methodId) {
        case METHODID_GET_ALL_SUBJECTS:
          serviceImpl.getAllSubjects((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.SubjectsResponse>) responseObserver);
          break;
        case METHODID_GET_ALL_OBJECTS:
          serviceImpl.getAllObjects((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.ObjectsResponse>) responseObserver);
          break;
        case METHODID_GET_ALL_ACTIONS:
          serviceImpl.getAllActions((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.ActionsResponse>) responseObserver);
          break;
        case METHODID_GET_ALL_ROLES:
          serviceImpl.getAllRoles((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse>) responseObserver);
          break;
        case METHODID_GET_ROLES_FOR_USER:
          serviceImpl.getRolesForUser((com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse>) responseObserver);
          break;
        case METHODID_GET_IMPLICIT_ROLES_FOR_USER:
          serviceImpl.getImplicitRolesForUser((com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse>) responseObserver);
          break;
        case METHODID_GET_USERS_FOR_ROLE:
          serviceImpl.getUsersForRole((com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.UsersResponse>) responseObserver);
          break;
        case METHODID_HAS_ROLE_FOR_USER:
          serviceImpl.hasRoleForUser((com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_ADD_ROLE_FOR_USER:
          serviceImpl.addRoleForUser((com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_DELETE_ROLE_FOR_USER:
          serviceImpl.deleteRoleForUser((com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_DELETE_USER:
          serviceImpl.deleteUser((com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_DELETE_ROLE:
          serviceImpl.deleteRole((com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_GET_PERMISSIONS_FOR_USER:
          serviceImpl.getPermissionsForUser((com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse>) responseObserver);
          break;
        case METHODID_GET_IMPLICIT_PERMISSIONS_FOR_USER:
          serviceImpl.getImplicitPermissionsForUser((com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse>) responseObserver);
          break;
        case METHODID_GET_PERMISSIONS_FOR_ROLE:
          serviceImpl.getPermissionsForRole((com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse>) responseObserver);
          break;
        case METHODID_DELETE_PERMISSION:
          serviceImpl.deletePermission((com.github.saturn_xiv.palm.plugins.rbac.v1.Permission) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_ADD_PERMISSION:
          serviceImpl.addPermission((com.github.saturn_xiv.palm.plugins.rbac.v1.Permission) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_HAS_PERMISSION:
          serviceImpl.hasPermission((com.github.saturn_xiv.palm.plugins.rbac.v1.Permission) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        default:
          throw new AssertionError();
      }
    }

    @java.lang.Override
    @java.lang.SuppressWarnings("unchecked")
    public io.grpc.stub.StreamObserver<Req> invoke(
        io.grpc.stub.StreamObserver<Resp> responseObserver) {
      switch (methodId) {
        default:
          throw new AssertionError();
      }
    }
  }

  public static final io.grpc.ServerServiceDefinition bindService(AsyncService service) {
    return io.grpc.ServerServiceDefinition.builder(getServiceDescriptor())
        .addMethod(
          getGetAllSubjectsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.rbac.v1.SubjectsResponse>(
                service, METHODID_GET_ALL_SUBJECTS)))
        .addMethod(
          getGetAllObjectsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.rbac.v1.ObjectsResponse>(
                service, METHODID_GET_ALL_OBJECTS)))
        .addMethod(
          getGetAllActionsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.rbac.v1.ActionsResponse>(
                service, METHODID_GET_ALL_ACTIONS)))
        .addMethod(
          getGetAllRolesMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse>(
                service, METHODID_GET_ALL_ROLES)))
        .addMethod(
          getGetRolesForUserMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User,
              com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse>(
                service, METHODID_GET_ROLES_FOR_USER)))
        .addMethod(
          getGetImplicitRolesForUserMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User,
              com.github.saturn_xiv.palm.plugins.rbac.v1.RolesResponse>(
                service, METHODID_GET_IMPLICIT_ROLES_FOR_USER)))
        .addMethod(
          getGetUsersForRoleMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role,
              com.github.saturn_xiv.palm.plugins.rbac.v1.UsersResponse>(
                service, METHODID_GET_USERS_FOR_ROLE)))
        .addMethod(
          getHasRoleForUserMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest,
              com.google.protobuf.Empty>(
                service, METHODID_HAS_ROLE_FOR_USER)))
        .addMethod(
          getAddRoleForUserMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest,
              com.google.protobuf.Empty>(
                service, METHODID_ADD_ROLE_FOR_USER)))
        .addMethod(
          getDeleteRoleForUserMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.rbac.v1.UserRoleRequest,
              com.google.protobuf.Empty>(
                service, METHODID_DELETE_ROLE_FOR_USER)))
        .addMethod(
          getDeleteUserMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User,
              com.google.protobuf.Empty>(
                service, METHODID_DELETE_USER)))
        .addMethod(
          getDeleteRoleMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role,
              com.google.protobuf.Empty>(
                service, METHODID_DELETE_ROLE)))
        .addMethod(
          getGetPermissionsForUserMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User,
              com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse>(
                service, METHODID_GET_PERMISSIONS_FOR_USER)))
        .addMethod(
          getGetImplicitPermissionsForUserMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User,
              com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse>(
                service, METHODID_GET_IMPLICIT_PERMISSIONS_FOR_USER)))
        .addMethod(
          getGetPermissionsForRoleMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role,
              com.github.saturn_xiv.palm.plugins.rbac.v1.PermissionsResponse>(
                service, METHODID_GET_PERMISSIONS_FOR_ROLE)))
        .addMethod(
          getDeletePermissionMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.rbac.v1.Permission,
              com.google.protobuf.Empty>(
                service, METHODID_DELETE_PERMISSION)))
        .addMethod(
          getAddPermissionMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.rbac.v1.Permission,
              com.google.protobuf.Empty>(
                service, METHODID_ADD_PERMISSION)))
        .addMethod(
          getHasPermissionMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.rbac.v1.Permission,
              com.google.protobuf.Empty>(
                service, METHODID_HAS_PERMISSION)))
        .build();
  }

  private static abstract class EnforcerBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    EnforcerBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.rbac.v1.RbacProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Enforcer");
    }
  }

  private static final class EnforcerFileDescriptorSupplier
      extends EnforcerBaseDescriptorSupplier {
    EnforcerFileDescriptorSupplier() {}
  }

  private static final class EnforcerMethodDescriptorSupplier
      extends EnforcerBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    EnforcerMethodDescriptorSupplier(java.lang.String methodName) {
      this.methodName = methodName;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.MethodDescriptor getMethodDescriptor() {
      return getServiceDescriptor().findMethodByName(methodName);
    }
  }

  private static volatile io.grpc.ServiceDescriptor serviceDescriptor;

  public static io.grpc.ServiceDescriptor getServiceDescriptor() {
    io.grpc.ServiceDescriptor result = serviceDescriptor;
    if (result == null) {
      synchronized (EnforcerGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new EnforcerFileDescriptorSupplier())
              .addMethod(getGetAllSubjectsMethod())
              .addMethod(getGetAllObjectsMethod())
              .addMethod(getGetAllActionsMethod())
              .addMethod(getGetAllRolesMethod())
              .addMethod(getGetRolesForUserMethod())
              .addMethod(getGetImplicitRolesForUserMethod())
              .addMethod(getGetUsersForRoleMethod())
              .addMethod(getHasRoleForUserMethod())
              .addMethod(getAddRoleForUserMethod())
              .addMethod(getDeleteRoleForUserMethod())
              .addMethod(getDeleteUserMethod())
              .addMethod(getDeleteRoleMethod())
              .addMethod(getGetPermissionsForUserMethod())
              .addMethod(getGetImplicitPermissionsForUserMethod())
              .addMethod(getGetPermissionsForRoleMethod())
              .addMethod(getDeletePermissionMethod())
              .addMethod(getAddPermissionMethod())
              .addMethod(getHasPermissionMethod())
              .build();
        }
      }
    }
    return result;
  }
}
