package com.github.saturn_xiv.palm.plugins.casbin.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 * <pre>
 * ----------------------------------------------------------------------------
 * </pre>
 */
@javax.annotation.Generated(
    value = "by gRPC proto compiler (version 1.68.1)",
    comments = "Source: casbin.proto")
@io.grpc.stub.annotations.GrpcGenerated
public final class PolicyGrpc {

  private PolicyGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.casbin.v1.Policy";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.casbin.v1.UsersResponse> getGetAllUsersMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetAllUsers",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.casbin.v1.UsersResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.casbin.v1.UsersResponse> getGetAllUsersMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.casbin.v1.UsersResponse> getGetAllUsersMethod;
    if ((getGetAllUsersMethod = PolicyGrpc.getGetAllUsersMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getGetAllUsersMethod = PolicyGrpc.getGetAllUsersMethod) == null) {
          PolicyGrpc.getGetAllUsersMethod = getGetAllUsersMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.casbin.v1.UsersResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetAllUsers"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.UsersResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("GetAllUsers"))
              .build();
        }
      }
    }
    return getGetAllUsersMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.casbin.v1.ObjectsResponse> getGetAllObjectsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetAllObjects",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.casbin.v1.ObjectsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.casbin.v1.ObjectsResponse> getGetAllObjectsMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.casbin.v1.ObjectsResponse> getGetAllObjectsMethod;
    if ((getGetAllObjectsMethod = PolicyGrpc.getGetAllObjectsMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getGetAllObjectsMethod = PolicyGrpc.getGetAllObjectsMethod) == null) {
          PolicyGrpc.getGetAllObjectsMethod = getGetAllObjectsMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.casbin.v1.ObjectsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetAllObjects"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.ObjectsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("GetAllObjects"))
              .build();
        }
      }
    }
    return getGetAllObjectsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.casbin.v1.ActionsResponse> getGetAllActionsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetAllActions",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.casbin.v1.ActionsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.casbin.v1.ActionsResponse> getGetAllActionsMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.casbin.v1.ActionsResponse> getGetAllActionsMethod;
    if ((getGetAllActionsMethod = PolicyGrpc.getGetAllActionsMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getGetAllActionsMethod = PolicyGrpc.getGetAllActionsMethod) == null) {
          PolicyGrpc.getGetAllActionsMethod = getGetAllActionsMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.casbin.v1.ActionsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetAllActions"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.ActionsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("GetAllActions"))
              .build();
        }
      }
    }
    return getGetAllActionsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> getGetAllRolesMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetAllRoles",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> getGetAllRolesMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> getGetAllRolesMethod;
    if ((getGetAllRolesMethod = PolicyGrpc.getGetAllRolesMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getGetAllRolesMethod = PolicyGrpc.getGetAllRolesMethod) == null) {
          PolicyGrpc.getGetAllRolesMethod = getGetAllRolesMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetAllRoles"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("GetAllRoles"))
              .build();
        }
      }
    }
    return getGetAllRolesMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getGetAllPermissionsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetAllPermissions",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getGetAllPermissionsMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getGetAllPermissionsMethod;
    if ((getGetAllPermissionsMethod = PolicyGrpc.getGetAllPermissionsMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getGetAllPermissionsMethod = PolicyGrpc.getGetAllPermissionsMethod) == null) {
          PolicyGrpc.getGetAllPermissionsMethod = getGetAllPermissionsMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetAllPermissions"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("GetAllPermissions"))
              .build();
        }
      }
    }
    return getGetAllPermissionsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest,
      com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> getHasMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Has",
      requestType = com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest,
      com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> getHasMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest, com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> getHasMethod;
    if ((getHasMethod = PolicyGrpc.getHasMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getHasMethod = PolicyGrpc.getHasMethod) == null) {
          PolicyGrpc.getHasMethod = getHasMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest, com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Has"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("Has"))
              .build();
        }
      }
    }
    return getHasMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest,
      com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> getCanMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Can",
      requestType = com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest,
      com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> getCanMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest, com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> getCanMethod;
    if ((getCanMethod = PolicyGrpc.getCanMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getCanMethod = PolicyGrpc.getCanMethod) == null) {
          PolicyGrpc.getCanMethod = getCanMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest, com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Can"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("Can"))
              .build();
        }
      }
    }
    return getCanMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.User,
      com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> getGetRolesForUserMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetRolesForUser",
      requestType = com.github.saturn_xiv.palm.plugins.casbin.v1.User.class,
      responseType = com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.User,
      com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> getGetRolesForUserMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.User, com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> getGetRolesForUserMethod;
    if ((getGetRolesForUserMethod = PolicyGrpc.getGetRolesForUserMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getGetRolesForUserMethod = PolicyGrpc.getGetRolesForUserMethod) == null) {
          PolicyGrpc.getGetRolesForUserMethod = getGetRolesForUserMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.casbin.v1.User, com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetRolesForUser"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.User.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("GetRolesForUser"))
              .build();
        }
      }
    }
    return getGetRolesForUserMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.User,
      com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> getGetImplicitRolesForUserMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetImplicitRolesForUser",
      requestType = com.github.saturn_xiv.palm.plugins.casbin.v1.User.class,
      responseType = com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.User,
      com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> getGetImplicitRolesForUserMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.User, com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> getGetImplicitRolesForUserMethod;
    if ((getGetImplicitRolesForUserMethod = PolicyGrpc.getGetImplicitRolesForUserMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getGetImplicitRolesForUserMethod = PolicyGrpc.getGetImplicitRolesForUserMethod) == null) {
          PolicyGrpc.getGetImplicitRolesForUserMethod = getGetImplicitRolesForUserMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.casbin.v1.User, com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetImplicitRolesForUser"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.User.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("GetImplicitRolesForUser"))
              .build();
        }
      }
    }
    return getGetImplicitRolesForUserMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.Role,
      com.github.saturn_xiv.palm.plugins.casbin.v1.UsersResponse> getGetUsersForRoleMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetUsersForRole",
      requestType = com.github.saturn_xiv.palm.plugins.casbin.v1.Role.class,
      responseType = com.github.saturn_xiv.palm.plugins.casbin.v1.UsersResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.Role,
      com.github.saturn_xiv.palm.plugins.casbin.v1.UsersResponse> getGetUsersForRoleMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.Role, com.github.saturn_xiv.palm.plugins.casbin.v1.UsersResponse> getGetUsersForRoleMethod;
    if ((getGetUsersForRoleMethod = PolicyGrpc.getGetUsersForRoleMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getGetUsersForRoleMethod = PolicyGrpc.getGetUsersForRoleMethod) == null) {
          PolicyGrpc.getGetUsersForRoleMethod = getGetUsersForRoleMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.casbin.v1.Role, com.github.saturn_xiv.palm.plugins.casbin.v1.UsersResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetUsersForRole"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.Role.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.UsersResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("GetUsersForRole"))
              .build();
        }
      }
    }
    return getGetUsersForRoleMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest,
      com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> getHasRoleForUserMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "HasRoleForUser",
      requestType = com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest,
      com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> getHasRoleForUserMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest, com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> getHasRoleForUserMethod;
    if ((getHasRoleForUserMethod = PolicyGrpc.getHasRoleForUserMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getHasRoleForUserMethod = PolicyGrpc.getHasRoleForUserMethod) == null) {
          PolicyGrpc.getHasRoleForUserMethod = getHasRoleForUserMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest, com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "HasRoleForUser"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("HasRoleForUser"))
              .build();
        }
      }
    }
    return getHasRoleForUserMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest,
      com.google.protobuf.Empty> getAddRoleForUserMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "AddRoleForUser",
      requestType = com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest,
      com.google.protobuf.Empty> getAddRoleForUserMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest, com.google.protobuf.Empty> getAddRoleForUserMethod;
    if ((getAddRoleForUserMethod = PolicyGrpc.getAddRoleForUserMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getAddRoleForUserMethod = PolicyGrpc.getAddRoleForUserMethod) == null) {
          PolicyGrpc.getAddRoleForUserMethod = getAddRoleForUserMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "AddRoleForUser"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("AddRoleForUser"))
              .build();
        }
      }
    }
    return getAddRoleForUserMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest,
      com.google.protobuf.Empty> getDeleteRoleForUserMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeleteRoleForUser",
      requestType = com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest,
      com.google.protobuf.Empty> getDeleteRoleForUserMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest, com.google.protobuf.Empty> getDeleteRoleForUserMethod;
    if ((getDeleteRoleForUserMethod = PolicyGrpc.getDeleteRoleForUserMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getDeleteRoleForUserMethod = PolicyGrpc.getDeleteRoleForUserMethod) == null) {
          PolicyGrpc.getDeleteRoleForUserMethod = getDeleteRoleForUserMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeleteRoleForUser"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("DeleteRoleForUser"))
              .build();
        }
      }
    }
    return getDeleteRoleForUserMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.User,
      com.google.protobuf.Empty> getDeleteUserMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeleteUser",
      requestType = com.github.saturn_xiv.palm.plugins.casbin.v1.User.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.User,
      com.google.protobuf.Empty> getDeleteUserMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.User, com.google.protobuf.Empty> getDeleteUserMethod;
    if ((getDeleteUserMethod = PolicyGrpc.getDeleteUserMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getDeleteUserMethod = PolicyGrpc.getDeleteUserMethod) == null) {
          PolicyGrpc.getDeleteUserMethod = getDeleteUserMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.casbin.v1.User, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeleteUser"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.User.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("DeleteUser"))
              .build();
        }
      }
    }
    return getDeleteUserMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.Role,
      com.google.protobuf.Empty> getDeleteRoleMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeleteRole",
      requestType = com.github.saturn_xiv.palm.plugins.casbin.v1.Role.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.Role,
      com.google.protobuf.Empty> getDeleteRoleMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.Role, com.google.protobuf.Empty> getDeleteRoleMethod;
    if ((getDeleteRoleMethod = PolicyGrpc.getDeleteRoleMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getDeleteRoleMethod = PolicyGrpc.getDeleteRoleMethod) == null) {
          PolicyGrpc.getDeleteRoleMethod = getDeleteRoleMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.casbin.v1.Role, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeleteRole"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.Role.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("DeleteRole"))
              .build();
        }
      }
    }
    return getDeleteRoleMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.User,
      com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getGetPermissionsForUserMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetPermissionsForUser",
      requestType = com.github.saturn_xiv.palm.plugins.casbin.v1.User.class,
      responseType = com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.User,
      com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getGetPermissionsForUserMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.User, com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getGetPermissionsForUserMethod;
    if ((getGetPermissionsForUserMethod = PolicyGrpc.getGetPermissionsForUserMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getGetPermissionsForUserMethod = PolicyGrpc.getGetPermissionsForUserMethod) == null) {
          PolicyGrpc.getGetPermissionsForUserMethod = getGetPermissionsForUserMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.casbin.v1.User, com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetPermissionsForUser"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.User.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("GetPermissionsForUser"))
              .build();
        }
      }
    }
    return getGetPermissionsForUserMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.User,
      com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getGetImplicitPermissionsForUserMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetImplicitPermissionsForUser",
      requestType = com.github.saturn_xiv.palm.plugins.casbin.v1.User.class,
      responseType = com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.User,
      com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getGetImplicitPermissionsForUserMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.User, com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getGetImplicitPermissionsForUserMethod;
    if ((getGetImplicitPermissionsForUserMethod = PolicyGrpc.getGetImplicitPermissionsForUserMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getGetImplicitPermissionsForUserMethod = PolicyGrpc.getGetImplicitPermissionsForUserMethod) == null) {
          PolicyGrpc.getGetImplicitPermissionsForUserMethod = getGetImplicitPermissionsForUserMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.casbin.v1.User, com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetImplicitPermissionsForUser"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.User.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("GetImplicitPermissionsForUser"))
              .build();
        }
      }
    }
    return getGetImplicitPermissionsForUserMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest,
      com.google.protobuf.Empty> getAddPermissionForUserMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "AddPermissionForUser",
      requestType = com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest,
      com.google.protobuf.Empty> getAddPermissionForUserMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest, com.google.protobuf.Empty> getAddPermissionForUserMethod;
    if ((getAddPermissionForUserMethod = PolicyGrpc.getAddPermissionForUserMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getAddPermissionForUserMethod = PolicyGrpc.getAddPermissionForUserMethod) == null) {
          PolicyGrpc.getAddPermissionForUserMethod = getAddPermissionForUserMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "AddPermissionForUser"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("AddPermissionForUser"))
              .build();
        }
      }
    }
    return getAddPermissionForUserMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest,
      com.google.protobuf.Empty> getDeletePermissionForUserMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeletePermissionForUser",
      requestType = com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest,
      com.google.protobuf.Empty> getDeletePermissionForUserMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest, com.google.protobuf.Empty> getDeletePermissionForUserMethod;
    if ((getDeletePermissionForUserMethod = PolicyGrpc.getDeletePermissionForUserMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getDeletePermissionForUserMethod = PolicyGrpc.getDeletePermissionForUserMethod) == null) {
          PolicyGrpc.getDeletePermissionForUserMethod = getDeletePermissionForUserMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeletePermissionForUser"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("DeletePermissionForUser"))
              .build();
        }
      }
    }
    return getDeletePermissionForUserMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest,
      com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> getHasPermissionForUserMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "HasPermissionForUser",
      requestType = com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest,
      com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> getHasPermissionForUserMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest, com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> getHasPermissionForUserMethod;
    if ((getHasPermissionForUserMethod = PolicyGrpc.getHasPermissionForUserMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getHasPermissionForUserMethod = PolicyGrpc.getHasPermissionForUserMethod) == null) {
          PolicyGrpc.getHasPermissionForUserMethod = getHasPermissionForUserMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest, com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "HasPermissionForUser"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("HasPermissionForUser"))
              .build();
        }
      }
    }
    return getHasPermissionForUserMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.Role,
      com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getGetPermissionsForRoleMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetPermissionsForRole",
      requestType = com.github.saturn_xiv.palm.plugins.casbin.v1.Role.class,
      responseType = com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.Role,
      com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getGetPermissionsForRoleMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.Role, com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getGetPermissionsForRoleMethod;
    if ((getGetPermissionsForRoleMethod = PolicyGrpc.getGetPermissionsForRoleMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getGetPermissionsForRoleMethod = PolicyGrpc.getGetPermissionsForRoleMethod) == null) {
          PolicyGrpc.getGetPermissionsForRoleMethod = getGetPermissionsForRoleMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.casbin.v1.Role, com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetPermissionsForRole"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.Role.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("GetPermissionsForRole"))
              .build();
        }
      }
    }
    return getGetPermissionsForRoleMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.Role,
      com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getGetImplicitPermissionsForRoleMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetImplicitPermissionsForRole",
      requestType = com.github.saturn_xiv.palm.plugins.casbin.v1.Role.class,
      responseType = com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.Role,
      com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getGetImplicitPermissionsForRoleMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.Role, com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getGetImplicitPermissionsForRoleMethod;
    if ((getGetImplicitPermissionsForRoleMethod = PolicyGrpc.getGetImplicitPermissionsForRoleMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getGetImplicitPermissionsForRoleMethod = PolicyGrpc.getGetImplicitPermissionsForRoleMethod) == null) {
          PolicyGrpc.getGetImplicitPermissionsForRoleMethod = getGetImplicitPermissionsForRoleMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.casbin.v1.Role, com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetImplicitPermissionsForRole"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.Role.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("GetImplicitPermissionsForRole"))
              .build();
        }
      }
    }
    return getGetImplicitPermissionsForRoleMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest,
      com.google.protobuf.Empty> getAddPermissionForRoleMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "AddPermissionForRole",
      requestType = com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest,
      com.google.protobuf.Empty> getAddPermissionForRoleMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest, com.google.protobuf.Empty> getAddPermissionForRoleMethod;
    if ((getAddPermissionForRoleMethod = PolicyGrpc.getAddPermissionForRoleMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getAddPermissionForRoleMethod = PolicyGrpc.getAddPermissionForRoleMethod) == null) {
          PolicyGrpc.getAddPermissionForRoleMethod = getAddPermissionForRoleMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "AddPermissionForRole"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("AddPermissionForRole"))
              .build();
        }
      }
    }
    return getAddPermissionForRoleMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest,
      com.google.protobuf.Empty> getDeletePermissionForRoleMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeletePermissionForRole",
      requestType = com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest,
      com.google.protobuf.Empty> getDeletePermissionForRoleMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest, com.google.protobuf.Empty> getDeletePermissionForRoleMethod;
    if ((getDeletePermissionForRoleMethod = PolicyGrpc.getDeletePermissionForRoleMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getDeletePermissionForRoleMethod = PolicyGrpc.getDeletePermissionForRoleMethod) == null) {
          PolicyGrpc.getDeletePermissionForRoleMethod = getDeletePermissionForRoleMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeletePermissionForRole"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("DeletePermissionForRole"))
              .build();
        }
      }
    }
    return getDeletePermissionForRoleMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest,
      com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> getHasPermissionForRoleMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "HasPermissionForRole",
      requestType = com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest,
      com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> getHasPermissionForRoleMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest, com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> getHasPermissionForRoleMethod;
    if ((getHasPermissionForRoleMethod = PolicyGrpc.getHasPermissionForRoleMethod) == null) {
      synchronized (PolicyGrpc.class) {
        if ((getHasPermissionForRoleMethod = PolicyGrpc.getHasPermissionForRoleMethod) == null) {
          PolicyGrpc.getHasPermissionForRoleMethod = getHasPermissionForRoleMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest, com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "HasPermissionForRole"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PolicyMethodDescriptorSupplier("HasPermissionForRole"))
              .build();
        }
      }
    }
    return getHasPermissionForRoleMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static PolicyStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PolicyStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PolicyStub>() {
        @java.lang.Override
        public PolicyStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PolicyStub(channel, callOptions);
        }
      };
    return PolicyStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static PolicyBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PolicyBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PolicyBlockingStub>() {
        @java.lang.Override
        public PolicyBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PolicyBlockingStub(channel, callOptions);
        }
      };
    return PolicyBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static PolicyFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PolicyFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PolicyFutureStub>() {
        @java.lang.Override
        public PolicyFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PolicyFutureStub(channel, callOptions);
        }
      };
    return PolicyFutureStub.newStub(factory, channel);
  }

  /**
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public interface AsyncService {

    /**
     */
    default void getAllUsers(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.UsersResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetAllUsersMethod(), responseObserver);
    }

    /**
     */
    default void getAllObjects(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.ObjectsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetAllObjectsMethod(), responseObserver);
    }

    /**
     */
    default void getAllActions(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.ActionsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetAllActionsMethod(), responseObserver);
    }

    /**
     */
    default void getAllRoles(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetAllRolesMethod(), responseObserver);
    }

    /**
     */
    default void getAllPermissions(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetAllPermissionsMethod(), responseObserver);
    }

    /**
     */
    default void has(com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getHasMethod(), responseObserver);
    }

    /**
     */
    default void can(com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCanMethod(), responseObserver);
    }

    /**
     */
    default void getRolesForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.User request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetRolesForUserMethod(), responseObserver);
    }

    /**
     */
    default void getImplicitRolesForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.User request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetImplicitRolesForUserMethod(), responseObserver);
    }

    /**
     */
    default void getUsersForRole(com.github.saturn_xiv.palm.plugins.casbin.v1.Role request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.UsersResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetUsersForRoleMethod(), responseObserver);
    }

    /**
     */
    default void hasRoleForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getHasRoleForUserMethod(), responseObserver);
    }

    /**
     */
    default void addRoleForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getAddRoleForUserMethod(), responseObserver);
    }

    /**
     */
    default void deleteRoleForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeleteRoleForUserMethod(), responseObserver);
    }

    /**
     */
    default void deleteUser(com.github.saturn_xiv.palm.plugins.casbin.v1.User request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeleteUserMethod(), responseObserver);
    }

    /**
     */
    default void deleteRole(com.github.saturn_xiv.palm.plugins.casbin.v1.Role request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeleteRoleMethod(), responseObserver);
    }

    /**
     */
    default void getPermissionsForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.User request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetPermissionsForUserMethod(), responseObserver);
    }

    /**
     */
    default void getImplicitPermissionsForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.User request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetImplicitPermissionsForUserMethod(), responseObserver);
    }

    /**
     */
    default void addPermissionForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getAddPermissionForUserMethod(), responseObserver);
    }

    /**
     */
    default void deletePermissionForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeletePermissionForUserMethod(), responseObserver);
    }

    /**
     */
    default void hasPermissionForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getHasPermissionForUserMethod(), responseObserver);
    }

    /**
     */
    default void getPermissionsForRole(com.github.saturn_xiv.palm.plugins.casbin.v1.Role request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetPermissionsForRoleMethod(), responseObserver);
    }

    /**
     */
    default void getImplicitPermissionsForRole(com.github.saturn_xiv.palm.plugins.casbin.v1.Role request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetImplicitPermissionsForRoleMethod(), responseObserver);
    }

    /**
     */
    default void addPermissionForRole(com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getAddPermissionForRoleMethod(), responseObserver);
    }

    /**
     */
    default void deletePermissionForRole(com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeletePermissionForRoleMethod(), responseObserver);
    }

    /**
     */
    default void hasPermissionForRole(com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getHasPermissionForRoleMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service Policy.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static abstract class PolicyImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return PolicyGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Policy.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class PolicyStub
      extends io.grpc.stub.AbstractAsyncStub<PolicyStub> {
    private PolicyStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PolicyStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PolicyStub(channel, callOptions);
    }

    /**
     */
    public void getAllUsers(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.UsersResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetAllUsersMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getAllObjects(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.ObjectsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetAllObjectsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getAllActions(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.ActionsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetAllActionsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getAllRoles(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetAllRolesMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getAllPermissions(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetAllPermissionsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void has(com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getHasMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void can(com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCanMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getRolesForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.User request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetRolesForUserMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getImplicitRolesForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.User request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetImplicitRolesForUserMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getUsersForRole(com.github.saturn_xiv.palm.plugins.casbin.v1.Role request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.UsersResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetUsersForRoleMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void hasRoleForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getHasRoleForUserMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void addRoleForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getAddRoleForUserMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void deleteRoleForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeleteRoleForUserMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void deleteUser(com.github.saturn_xiv.palm.plugins.casbin.v1.User request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeleteUserMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void deleteRole(com.github.saturn_xiv.palm.plugins.casbin.v1.Role request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeleteRoleMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getPermissionsForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.User request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetPermissionsForUserMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getImplicitPermissionsForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.User request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetImplicitPermissionsForUserMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void addPermissionForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getAddPermissionForUserMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void deletePermissionForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeletePermissionForUserMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void hasPermissionForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getHasPermissionForUserMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getPermissionsForRole(com.github.saturn_xiv.palm.plugins.casbin.v1.Role request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetPermissionsForRoleMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getImplicitPermissionsForRole(com.github.saturn_xiv.palm.plugins.casbin.v1.Role request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetImplicitPermissionsForRoleMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void addPermissionForRole(com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getAddPermissionForRoleMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void deletePermissionForRole(com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeletePermissionForRoleMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void hasPermissionForRole(com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getHasPermissionForRoleMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Policy.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class PolicyBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<PolicyBlockingStub> {
    private PolicyBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PolicyBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PolicyBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.casbin.v1.UsersResponse getAllUsers(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetAllUsersMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.casbin.v1.ObjectsResponse getAllObjects(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetAllObjectsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.casbin.v1.ActionsResponse getAllActions(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetAllActionsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse getAllRoles(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetAllRolesMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse getAllPermissions(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetAllPermissionsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse has(com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getHasMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse can(com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCanMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse getRolesForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.User request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetRolesForUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse getImplicitRolesForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.User request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetImplicitRolesForUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.casbin.v1.UsersResponse getUsersForRole(com.github.saturn_xiv.palm.plugins.casbin.v1.Role request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetUsersForRoleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse hasRoleForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getHasRoleForUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty addRoleForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getAddRoleForUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deleteRoleForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeleteRoleForUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deleteUser(com.github.saturn_xiv.palm.plugins.casbin.v1.User request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeleteUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deleteRole(com.github.saturn_xiv.palm.plugins.casbin.v1.Role request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeleteRoleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse getPermissionsForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.User request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetPermissionsForUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse getImplicitPermissionsForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.User request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetImplicitPermissionsForUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty addPermissionForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getAddPermissionForUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deletePermissionForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeletePermissionForUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse hasPermissionForUser(com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getHasPermissionForUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse getPermissionsForRole(com.github.saturn_xiv.palm.plugins.casbin.v1.Role request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetPermissionsForRoleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse getImplicitPermissionsForRole(com.github.saturn_xiv.palm.plugins.casbin.v1.Role request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetImplicitPermissionsForRoleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty addPermissionForRole(com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getAddPermissionForRoleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deletePermissionForRole(com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeletePermissionForRoleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse hasPermissionForRole(com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getHasPermissionForRoleMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Policy.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class PolicyFutureStub
      extends io.grpc.stub.AbstractFutureStub<PolicyFutureStub> {
    private PolicyFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PolicyFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PolicyFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.casbin.v1.UsersResponse> getAllUsers(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetAllUsersMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.casbin.v1.ObjectsResponse> getAllObjects(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetAllObjectsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.casbin.v1.ActionsResponse> getAllActions(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetAllActionsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> getAllRoles(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetAllRolesMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getAllPermissions(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetAllPermissionsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> has(
        com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getHasMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> can(
        com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCanMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> getRolesForUser(
        com.github.saturn_xiv.palm.plugins.casbin.v1.User request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetRolesForUserMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> getImplicitRolesForUser(
        com.github.saturn_xiv.palm.plugins.casbin.v1.User request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetImplicitRolesForUserMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.casbin.v1.UsersResponse> getUsersForRole(
        com.github.saturn_xiv.palm.plugins.casbin.v1.Role request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetUsersForRoleMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> hasRoleForUser(
        com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getHasRoleForUserMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> addRoleForUser(
        com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getAddRoleForUserMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> deleteRoleForUser(
        com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeleteRoleForUserMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> deleteUser(
        com.github.saturn_xiv.palm.plugins.casbin.v1.User request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeleteUserMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> deleteRole(
        com.github.saturn_xiv.palm.plugins.casbin.v1.Role request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeleteRoleMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getPermissionsForUser(
        com.github.saturn_xiv.palm.plugins.casbin.v1.User request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetPermissionsForUserMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getImplicitPermissionsForUser(
        com.github.saturn_xiv.palm.plugins.casbin.v1.User request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetImplicitPermissionsForUserMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> addPermissionForUser(
        com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getAddPermissionForUserMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> deletePermissionForUser(
        com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeletePermissionForUserMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> hasPermissionForUser(
        com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getHasPermissionForUserMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getPermissionsForRole(
        com.github.saturn_xiv.palm.plugins.casbin.v1.Role request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetPermissionsForRoleMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getImplicitPermissionsForRole(
        com.github.saturn_xiv.palm.plugins.casbin.v1.Role request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetImplicitPermissionsForRoleMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> addPermissionForRole(
        com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getAddPermissionForRoleMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> deletePermissionForRole(
        com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeletePermissionForRoleMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> hasPermissionForRole(
        com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getHasPermissionForRoleMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_GET_ALL_USERS = 0;
  private static final int METHODID_GET_ALL_OBJECTS = 1;
  private static final int METHODID_GET_ALL_ACTIONS = 2;
  private static final int METHODID_GET_ALL_ROLES = 3;
  private static final int METHODID_GET_ALL_PERMISSIONS = 4;
  private static final int METHODID_HAS = 5;
  private static final int METHODID_CAN = 6;
  private static final int METHODID_GET_ROLES_FOR_USER = 7;
  private static final int METHODID_GET_IMPLICIT_ROLES_FOR_USER = 8;
  private static final int METHODID_GET_USERS_FOR_ROLE = 9;
  private static final int METHODID_HAS_ROLE_FOR_USER = 10;
  private static final int METHODID_ADD_ROLE_FOR_USER = 11;
  private static final int METHODID_DELETE_ROLE_FOR_USER = 12;
  private static final int METHODID_DELETE_USER = 13;
  private static final int METHODID_DELETE_ROLE = 14;
  private static final int METHODID_GET_PERMISSIONS_FOR_USER = 15;
  private static final int METHODID_GET_IMPLICIT_PERMISSIONS_FOR_USER = 16;
  private static final int METHODID_ADD_PERMISSION_FOR_USER = 17;
  private static final int METHODID_DELETE_PERMISSION_FOR_USER = 18;
  private static final int METHODID_HAS_PERMISSION_FOR_USER = 19;
  private static final int METHODID_GET_PERMISSIONS_FOR_ROLE = 20;
  private static final int METHODID_GET_IMPLICIT_PERMISSIONS_FOR_ROLE = 21;
  private static final int METHODID_ADD_PERMISSION_FOR_ROLE = 22;
  private static final int METHODID_DELETE_PERMISSION_FOR_ROLE = 23;
  private static final int METHODID_HAS_PERMISSION_FOR_ROLE = 24;

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
        case METHODID_GET_ALL_USERS:
          serviceImpl.getAllUsers((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.UsersResponse>) responseObserver);
          break;
        case METHODID_GET_ALL_OBJECTS:
          serviceImpl.getAllObjects((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.ObjectsResponse>) responseObserver);
          break;
        case METHODID_GET_ALL_ACTIONS:
          serviceImpl.getAllActions((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.ActionsResponse>) responseObserver);
          break;
        case METHODID_GET_ALL_ROLES:
          serviceImpl.getAllRoles((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse>) responseObserver);
          break;
        case METHODID_GET_ALL_PERMISSIONS:
          serviceImpl.getAllPermissions((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse>) responseObserver);
          break;
        case METHODID_HAS:
          serviceImpl.has((com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse>) responseObserver);
          break;
        case METHODID_CAN:
          serviceImpl.can((com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse>) responseObserver);
          break;
        case METHODID_GET_ROLES_FOR_USER:
          serviceImpl.getRolesForUser((com.github.saturn_xiv.palm.plugins.casbin.v1.User) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse>) responseObserver);
          break;
        case METHODID_GET_IMPLICIT_ROLES_FOR_USER:
          serviceImpl.getImplicitRolesForUser((com.github.saturn_xiv.palm.plugins.casbin.v1.User) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse>) responseObserver);
          break;
        case METHODID_GET_USERS_FOR_ROLE:
          serviceImpl.getUsersForRole((com.github.saturn_xiv.palm.plugins.casbin.v1.Role) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.UsersResponse>) responseObserver);
          break;
        case METHODID_HAS_ROLE_FOR_USER:
          serviceImpl.hasRoleForUser((com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse>) responseObserver);
          break;
        case METHODID_ADD_ROLE_FOR_USER:
          serviceImpl.addRoleForUser((com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_DELETE_ROLE_FOR_USER:
          serviceImpl.deleteRoleForUser((com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_DELETE_USER:
          serviceImpl.deleteUser((com.github.saturn_xiv.palm.plugins.casbin.v1.User) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_DELETE_ROLE:
          serviceImpl.deleteRole((com.github.saturn_xiv.palm.plugins.casbin.v1.Role) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_GET_PERMISSIONS_FOR_USER:
          serviceImpl.getPermissionsForUser((com.github.saturn_xiv.palm.plugins.casbin.v1.User) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse>) responseObserver);
          break;
        case METHODID_GET_IMPLICIT_PERMISSIONS_FOR_USER:
          serviceImpl.getImplicitPermissionsForUser((com.github.saturn_xiv.palm.plugins.casbin.v1.User) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse>) responseObserver);
          break;
        case METHODID_ADD_PERMISSION_FOR_USER:
          serviceImpl.addPermissionForUser((com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_DELETE_PERMISSION_FOR_USER:
          serviceImpl.deletePermissionForUser((com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_HAS_PERMISSION_FOR_USER:
          serviceImpl.hasPermissionForUser((com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse>) responseObserver);
          break;
        case METHODID_GET_PERMISSIONS_FOR_ROLE:
          serviceImpl.getPermissionsForRole((com.github.saturn_xiv.palm.plugins.casbin.v1.Role) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse>) responseObserver);
          break;
        case METHODID_GET_IMPLICIT_PERMISSIONS_FOR_ROLE:
          serviceImpl.getImplicitPermissionsForRole((com.github.saturn_xiv.palm.plugins.casbin.v1.Role) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse>) responseObserver);
          break;
        case METHODID_ADD_PERMISSION_FOR_ROLE:
          serviceImpl.addPermissionForRole((com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_DELETE_PERMISSION_FOR_ROLE:
          serviceImpl.deletePermissionForRole((com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_HAS_PERMISSION_FOR_ROLE:
          serviceImpl.hasPermissionForRole((com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse>) responseObserver);
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
          getGetAllUsersMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.casbin.v1.UsersResponse>(
                service, METHODID_GET_ALL_USERS)))
        .addMethod(
          getGetAllObjectsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.casbin.v1.ObjectsResponse>(
                service, METHODID_GET_ALL_OBJECTS)))
        .addMethod(
          getGetAllActionsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.casbin.v1.ActionsResponse>(
                service, METHODID_GET_ALL_ACTIONS)))
        .addMethod(
          getGetAllRolesMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse>(
                service, METHODID_GET_ALL_ROLES)))
        .addMethod(
          getGetAllPermissionsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse>(
                service, METHODID_GET_ALL_PERMISSIONS)))
        .addMethod(
          getHasMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest,
              com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse>(
                service, METHODID_HAS)))
        .addMethod(
          getCanMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest,
              com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse>(
                service, METHODID_CAN)))
        .addMethod(
          getGetRolesForUserMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.casbin.v1.User,
              com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse>(
                service, METHODID_GET_ROLES_FOR_USER)))
        .addMethod(
          getGetImplicitRolesForUserMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.casbin.v1.User,
              com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse>(
                service, METHODID_GET_IMPLICIT_ROLES_FOR_USER)))
        .addMethod(
          getGetUsersForRoleMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.casbin.v1.Role,
              com.github.saturn_xiv.palm.plugins.casbin.v1.UsersResponse>(
                service, METHODID_GET_USERS_FOR_ROLE)))
        .addMethod(
          getHasRoleForUserMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest,
              com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse>(
                service, METHODID_HAS_ROLE_FOR_USER)))
        .addMethod(
          getAddRoleForUserMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest,
              com.google.protobuf.Empty>(
                service, METHODID_ADD_ROLE_FOR_USER)))
        .addMethod(
          getDeleteRoleForUserMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.casbin.v1.UserRoleRequest,
              com.google.protobuf.Empty>(
                service, METHODID_DELETE_ROLE_FOR_USER)))
        .addMethod(
          getDeleteUserMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.casbin.v1.User,
              com.google.protobuf.Empty>(
                service, METHODID_DELETE_USER)))
        .addMethod(
          getDeleteRoleMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.casbin.v1.Role,
              com.google.protobuf.Empty>(
                service, METHODID_DELETE_ROLE)))
        .addMethod(
          getGetPermissionsForUserMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.casbin.v1.User,
              com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse>(
                service, METHODID_GET_PERMISSIONS_FOR_USER)))
        .addMethod(
          getGetImplicitPermissionsForUserMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.casbin.v1.User,
              com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse>(
                service, METHODID_GET_IMPLICIT_PERMISSIONS_FOR_USER)))
        .addMethod(
          getAddPermissionForUserMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest,
              com.google.protobuf.Empty>(
                service, METHODID_ADD_PERMISSION_FOR_USER)))
        .addMethod(
          getDeletePermissionForUserMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest,
              com.google.protobuf.Empty>(
                service, METHODID_DELETE_PERMISSION_FOR_USER)))
        .addMethod(
          getHasPermissionForUserMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.casbin.v1.UserPermissionRequest,
              com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse>(
                service, METHODID_HAS_PERMISSION_FOR_USER)))
        .addMethod(
          getGetPermissionsForRoleMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.casbin.v1.Role,
              com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse>(
                service, METHODID_GET_PERMISSIONS_FOR_ROLE)))
        .addMethod(
          getGetImplicitPermissionsForRoleMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.casbin.v1.Role,
              com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse>(
                service, METHODID_GET_IMPLICIT_PERMISSIONS_FOR_ROLE)))
        .addMethod(
          getAddPermissionForRoleMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest,
              com.google.protobuf.Empty>(
                service, METHODID_ADD_PERMISSION_FOR_ROLE)))
        .addMethod(
          getDeletePermissionForRoleMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest,
              com.google.protobuf.Empty>(
                service, METHODID_DELETE_PERMISSION_FOR_ROLE)))
        .addMethod(
          getHasPermissionForRoleMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.casbin.v1.RolePermissionRequest,
              com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse>(
                service, METHODID_HAS_PERMISSION_FOR_ROLE)))
        .build();
  }

  private static abstract class PolicyBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    PolicyBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.casbin.v1.Casbin.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Policy");
    }
  }

  private static final class PolicyFileDescriptorSupplier
      extends PolicyBaseDescriptorSupplier {
    PolicyFileDescriptorSupplier() {}
  }

  private static final class PolicyMethodDescriptorSupplier
      extends PolicyBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    PolicyMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (PolicyGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new PolicyFileDescriptorSupplier())
              .addMethod(getGetAllUsersMethod())
              .addMethod(getGetAllObjectsMethod())
              .addMethod(getGetAllActionsMethod())
              .addMethod(getGetAllRolesMethod())
              .addMethod(getGetAllPermissionsMethod())
              .addMethod(getHasMethod())
              .addMethod(getCanMethod())
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
              .addMethod(getAddPermissionForUserMethod())
              .addMethod(getDeletePermissionForUserMethod())
              .addMethod(getHasPermissionForUserMethod())
              .addMethod(getGetPermissionsForRoleMethod())
              .addMethod(getGetImplicitPermissionsForRoleMethod())
              .addMethod(getAddPermissionForRoleMethod())
              .addMethod(getDeletePermissionForRoleMethod())
              .addMethod(getHasPermissionForRoleMethod())
              .build();
        }
      }
    }
    return result;
  }
}
