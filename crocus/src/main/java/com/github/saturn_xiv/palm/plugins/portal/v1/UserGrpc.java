package com.github.saturn_xiv.palm.plugins.portal.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@javax.annotation.Generated(
    value = "by gRPC proto compiler (version 1.68.1)",
    comments = "Source: portal.proto")
@io.grpc.stub.annotations.GrpcGenerated
public final class UserGrpc {

  private UserGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.portal.v1.User";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page,
      com.github.saturn_xiv.palm.plugins.portal.v1.UserLogsResponse> getLogsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Logs",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.Page.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.UserLogsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page,
      com.github.saturn_xiv.palm.plugins.portal.v1.UserLogsResponse> getLogsMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page, com.github.saturn_xiv.palm.plugins.portal.v1.UserLogsResponse> getLogsMethod;
    if ((getLogsMethod = UserGrpc.getLogsMethod) == null) {
      synchronized (UserGrpc.class) {
        if ((getLogsMethod = UserGrpc.getLogsMethod) == null) {
          UserGrpc.getLogsMethod = getLogsMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.Page, com.github.saturn_xiv.palm.plugins.portal.v1.UserLogsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Logs"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.Page.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.UserLogsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new UserMethodDescriptorSupplier("Logs"))
              .build();
        }
      }
    }
    return getLogsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page,
      com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse> getIndexMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Index",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.Page.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page,
      com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse> getIndexMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page, com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse> getIndexMethod;
    if ((getIndexMethod = UserGrpc.getIndexMethod) == null) {
      synchronized (UserGrpc.class) {
        if ((getIndexMethod = UserGrpc.getIndexMethod) == null) {
          UserGrpc.getIndexMethod = getIndexMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.Page, com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Index"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.Page.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse.getDefaultInstance()))
              .setSchemaDescriptor(new UserMethodDescriptorSupplier("Index"))
              .build();
        }
      }
    }
    return getIndexMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.UserSetLocationRequest,
      com.google.protobuf.Empty> getSetLocationMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SetLocation",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.UserSetLocationRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.UserSetLocationRequest,
      com.google.protobuf.Empty> getSetLocationMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.UserSetLocationRequest, com.google.protobuf.Empty> getSetLocationMethod;
    if ((getSetLocationMethod = UserGrpc.getSetLocationMethod) == null) {
      synchronized (UserGrpc.class) {
        if ((getSetLocationMethod = UserGrpc.getSetLocationMethod) == null) {
          UserGrpc.getSetLocationMethod = getSetLocationMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.UserSetLocationRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SetLocation"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.UserSetLocationRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new UserMethodDescriptorSupplier("SetLocation"))
              .build();
        }
      }
    }
    return getSetLocationMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.UserSetVRequest,
      com.google.protobuf.Empty> getSetVMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SetV",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.UserSetVRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.UserSetVRequest,
      com.google.protobuf.Empty> getSetVMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.UserSetVRequest, com.google.protobuf.Empty> getSetVMethod;
    if ((getSetVMethod = UserGrpc.getSetVMethod) == null) {
      synchronized (UserGrpc.class) {
        if ((getSetVMethod = UserGrpc.getSetVMethod) == null) {
          UserGrpc.getSetVMethod = getSetVMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.UserSetVRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SetV"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.UserSetVRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new UserMethodDescriptorSupplier("SetV"))
              .build();
        }
      }
    }
    return getSetVMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.UserGetVRequest,
      com.github.saturn_xiv.palm.plugins.portal.v1.UserGetVResponse> getGetVMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetV",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.UserGetVRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.UserGetVResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.UserGetVRequest,
      com.github.saturn_xiv.palm.plugins.portal.v1.UserGetVResponse> getGetVMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.UserGetVRequest, com.github.saturn_xiv.palm.plugins.portal.v1.UserGetVResponse> getGetVMethod;
    if ((getGetVMethod = UserGrpc.getGetVMethod) == null) {
      synchronized (UserGrpc.class) {
        if ((getGetVMethod = UserGrpc.getGetVMethod) == null) {
          UserGrpc.getGetVMethod = getGetVMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.UserGetVRequest, com.github.saturn_xiv.palm.plugins.portal.v1.UserGetVResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetV"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.UserGetVRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.UserGetVResponse.getDefaultInstance()))
              .setSchemaDescriptor(new UserMethodDescriptorSupplier("GetV"))
              .build();
        }
      }
    }
    return getGetVMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.UserUploadRequest,
      com.github.saturn_xiv.palm.plugins.portal.v1.UserUploadResponse> getUploadMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Upload",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.UserUploadRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.UserUploadResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.UserUploadRequest,
      com.github.saturn_xiv.palm.plugins.portal.v1.UserUploadResponse> getUploadMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.UserUploadRequest, com.github.saturn_xiv.palm.plugins.portal.v1.UserUploadResponse> getUploadMethod;
    if ((getUploadMethod = UserGrpc.getUploadMethod) == null) {
      synchronized (UserGrpc.class) {
        if ((getUploadMethod = UserGrpc.getUploadMethod) == null) {
          UserGrpc.getUploadMethod = getUploadMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.UserUploadRequest, com.github.saturn_xiv.palm.plugins.portal.v1.UserUploadResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Upload"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.UserUploadRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.UserUploadResponse.getDefaultInstance()))
              .setSchemaDescriptor(new UserMethodDescriptorSupplier("Upload"))
              .build();
        }
      }
    }
    return getUploadMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.google.protobuf.Empty> getSignOutMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SignOut",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.google.protobuf.Empty> getSignOutMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.google.protobuf.Empty> getSignOutMethod;
    if ((getSignOutMethod = UserGrpc.getSignOutMethod) == null) {
      synchronized (UserGrpc.class) {
        if ((getSignOutMethod = UserGrpc.getSignOutMethod) == null) {
          UserGrpc.getSignOutMethod = getSignOutMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SignOut"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new UserMethodDescriptorSupplier("SignOut"))
              .build();
        }
      }
    }
    return getSignOutMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest,
      com.google.protobuf.Empty> getLockMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Lock",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest,
      com.google.protobuf.Empty> getLockMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest, com.google.protobuf.Empty> getLockMethod;
    if ((getLockMethod = UserGrpc.getLockMethod) == null) {
      synchronized (UserGrpc.class) {
        if ((getLockMethod = UserGrpc.getLockMethod) == null) {
          UserGrpc.getLockMethod = getLockMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Lock"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new UserMethodDescriptorSupplier("Lock"))
              .build();
        }
      }
    }
    return getLockMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest,
      com.google.protobuf.Empty> getUnlockMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Unlock",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest,
      com.google.protobuf.Empty> getUnlockMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest, com.google.protobuf.Empty> getUnlockMethod;
    if ((getUnlockMethod = UserGrpc.getUnlockMethod) == null) {
      synchronized (UserGrpc.class) {
        if ((getUnlockMethod = UserGrpc.getUnlockMethod) == null) {
          UserGrpc.getUnlockMethod = getUnlockMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Unlock"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new UserMethodDescriptorSupplier("Unlock"))
              .build();
        }
      }
    }
    return getUnlockMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest,
      com.google.protobuf.Empty> getDisableMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Disable",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest,
      com.google.protobuf.Empty> getDisableMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest, com.google.protobuf.Empty> getDisableMethod;
    if ((getDisableMethod = UserGrpc.getDisableMethod) == null) {
      synchronized (UserGrpc.class) {
        if ((getDisableMethod = UserGrpc.getDisableMethod) == null) {
          UserGrpc.getDisableMethod = getDisableMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Disable"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new UserMethodDescriptorSupplier("Disable"))
              .build();
        }
      }
    }
    return getDisableMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest,
      com.google.protobuf.Empty> getEnableMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Enable",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest,
      com.google.protobuf.Empty> getEnableMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest, com.google.protobuf.Empty> getEnableMethod;
    if ((getEnableMethod = UserGrpc.getEnableMethod) == null) {
      synchronized (UserGrpc.class) {
        if ((getEnableMethod = UserGrpc.getEnableMethod) == null) {
          UserGrpc.getEnableMethod = getEnableMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Enable"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new UserMethodDescriptorSupplier("Enable"))
              .build();
        }
      }
    }
    return getEnableMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static UserStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<UserStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<UserStub>() {
        @java.lang.Override
        public UserStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new UserStub(channel, callOptions);
        }
      };
    return UserStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static UserBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<UserBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<UserBlockingStub>() {
        @java.lang.Override
        public UserBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new UserBlockingStub(channel, callOptions);
        }
      };
    return UserBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static UserFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<UserFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<UserFutureStub>() {
        @java.lang.Override
        public UserFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new UserFutureStub(channel, callOptions);
        }
      };
    return UserFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void logs(com.github.saturn_xiv.palm.plugins.portal.v1.Page request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserLogsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getLogsMethod(), responseObserver);
    }

    /**
     */
    default void index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getIndexMethod(), responseObserver);
    }

    /**
     */
    default void setLocation(com.github.saturn_xiv.palm.plugins.portal.v1.UserSetLocationRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSetLocationMethod(), responseObserver);
    }

    /**
     */
    default void setV(com.github.saturn_xiv.palm.plugins.portal.v1.UserSetVRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSetVMethod(), responseObserver);
    }

    /**
     */
    default void getV(com.github.saturn_xiv.palm.plugins.portal.v1.UserGetVRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserGetVResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetVMethod(), responseObserver);
    }

    /**
     */
    default void upload(com.github.saturn_xiv.palm.plugins.portal.v1.UserUploadRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserUploadResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getUploadMethod(), responseObserver);
    }

    /**
     */
    default void signOut(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSignOutMethod(), responseObserver);
    }

    /**
     */
    default void lock(com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getLockMethod(), responseObserver);
    }

    /**
     */
    default void unlock(com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getUnlockMethod(), responseObserver);
    }

    /**
     */
    default void disable(com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDisableMethod(), responseObserver);
    }

    /**
     */
    default void enable(com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getEnableMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service User.
   */
  public static abstract class UserImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return UserGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service User.
   */
  public static final class UserStub
      extends io.grpc.stub.AbstractAsyncStub<UserStub> {
    private UserStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected UserStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new UserStub(channel, callOptions);
    }

    /**
     */
    public void logs(com.github.saturn_xiv.palm.plugins.portal.v1.Page request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserLogsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getLogsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void setLocation(com.github.saturn_xiv.palm.plugins.portal.v1.UserSetLocationRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSetLocationMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void setV(com.github.saturn_xiv.palm.plugins.portal.v1.UserSetVRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSetVMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getV(com.github.saturn_xiv.palm.plugins.portal.v1.UserGetVRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserGetVResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetVMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void upload(com.github.saturn_xiv.palm.plugins.portal.v1.UserUploadRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserUploadResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getUploadMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void signOut(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSignOutMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void lock(com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getLockMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void unlock(com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getUnlockMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void disable(com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDisableMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void enable(com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getEnableMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service User.
   */
  public static final class UserBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<UserBlockingStub> {
    private UserBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected UserBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new UserBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.UserLogsResponse logs(com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getLogsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getIndexMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setLocation(com.github.saturn_xiv.palm.plugins.portal.v1.UserSetLocationRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetLocationMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setV(com.github.saturn_xiv.palm.plugins.portal.v1.UserSetVRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetVMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.UserGetVResponse getV(com.github.saturn_xiv.palm.plugins.portal.v1.UserGetVRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetVMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.UserUploadResponse upload(com.github.saturn_xiv.palm.plugins.portal.v1.UserUploadRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getUploadMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty signOut(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSignOutMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty lock(com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getLockMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty unlock(com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getUnlockMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty disable(com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDisableMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty enable(com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getEnableMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service User.
   */
  public static final class UserFutureStub
      extends io.grpc.stub.AbstractFutureStub<UserFutureStub> {
    private UserFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected UserFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new UserFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.UserLogsResponse> logs(
        com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getLogsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse> index(
        com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> setLocation(
        com.github.saturn_xiv.palm.plugins.portal.v1.UserSetLocationRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSetLocationMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> setV(
        com.github.saturn_xiv.palm.plugins.portal.v1.UserSetVRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSetVMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.UserGetVResponse> getV(
        com.github.saturn_xiv.palm.plugins.portal.v1.UserGetVRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetVMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.UserUploadResponse> upload(
        com.github.saturn_xiv.palm.plugins.portal.v1.UserUploadRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getUploadMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> signOut(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSignOutMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> lock(
        com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getLockMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> unlock(
        com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getUnlockMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> disable(
        com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDisableMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> enable(
        com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getEnableMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_LOGS = 0;
  private static final int METHODID_INDEX = 1;
  private static final int METHODID_SET_LOCATION = 2;
  private static final int METHODID_SET_V = 3;
  private static final int METHODID_GET_V = 4;
  private static final int METHODID_UPLOAD = 5;
  private static final int METHODID_SIGN_OUT = 6;
  private static final int METHODID_LOCK = 7;
  private static final int METHODID_UNLOCK = 8;
  private static final int METHODID_DISABLE = 9;
  private static final int METHODID_ENABLE = 10;

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
        case METHODID_LOGS:
          serviceImpl.logs((com.github.saturn_xiv.palm.plugins.portal.v1.Page) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserLogsResponse>) responseObserver);
          break;
        case METHODID_INDEX:
          serviceImpl.index((com.github.saturn_xiv.palm.plugins.portal.v1.Page) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse>) responseObserver);
          break;
        case METHODID_SET_LOCATION:
          serviceImpl.setLocation((com.github.saturn_xiv.palm.plugins.portal.v1.UserSetLocationRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_SET_V:
          serviceImpl.setV((com.github.saturn_xiv.palm.plugins.portal.v1.UserSetVRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_GET_V:
          serviceImpl.getV((com.github.saturn_xiv.palm.plugins.portal.v1.UserGetVRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserGetVResponse>) responseObserver);
          break;
        case METHODID_UPLOAD:
          serviceImpl.upload((com.github.saturn_xiv.palm.plugins.portal.v1.UserUploadRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserUploadResponse>) responseObserver);
          break;
        case METHODID_SIGN_OUT:
          serviceImpl.signOut((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_LOCK:
          serviceImpl.lock((com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_UNLOCK:
          serviceImpl.unlock((com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_DISABLE:
          serviceImpl.disable((com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_ENABLE:
          serviceImpl.enable((com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest) request,
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
          getLogsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.Page,
              com.github.saturn_xiv.palm.plugins.portal.v1.UserLogsResponse>(
                service, METHODID_LOGS)))
        .addMethod(
          getIndexMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.Page,
              com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse>(
                service, METHODID_INDEX)))
        .addMethod(
          getSetLocationMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.UserSetLocationRequest,
              com.google.protobuf.Empty>(
                service, METHODID_SET_LOCATION)))
        .addMethod(
          getSetVMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.UserSetVRequest,
              com.google.protobuf.Empty>(
                service, METHODID_SET_V)))
        .addMethod(
          getGetVMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.UserGetVRequest,
              com.github.saturn_xiv.palm.plugins.portal.v1.UserGetVResponse>(
                service, METHODID_GET_V)))
        .addMethod(
          getUploadMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.UserUploadRequest,
              com.github.saturn_xiv.palm.plugins.portal.v1.UserUploadResponse>(
                service, METHODID_UPLOAD)))
        .addMethod(
          getSignOutMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.google.protobuf.Empty>(
                service, METHODID_SIGN_OUT)))
        .addMethod(
          getLockMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest,
              com.google.protobuf.Empty>(
                service, METHODID_LOCK)))
        .addMethod(
          getUnlockMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest,
              com.google.protobuf.Empty>(
                service, METHODID_UNLOCK)))
        .addMethod(
          getDisableMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest,
              com.google.protobuf.Empty>(
                service, METHODID_DISABLE)))
        .addMethod(
          getEnableMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest,
              com.google.protobuf.Empty>(
                service, METHODID_ENABLE)))
        .build();
  }

  private static abstract class UserBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    UserBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.portal.v1.Portal.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("User");
    }
  }

  private static final class UserFileDescriptorSupplier
      extends UserBaseDescriptorSupplier {
    UserFileDescriptorSupplier() {}
  }

  private static final class UserMethodDescriptorSupplier
      extends UserBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    UserMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (UserGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new UserFileDescriptorSupplier())
              .addMethod(getLogsMethod())
              .addMethod(getIndexMethod())
              .addMethod(getSetLocationMethod())
              .addMethod(getSetVMethod())
              .addMethod(getGetVMethod())
              .addMethod(getUploadMethod())
              .addMethod(getSignOutMethod())
              .addMethod(getLockMethod())
              .addMethod(getUnlockMethod())
              .addMethod(getDisableMethod())
              .addMethod(getEnableMethod())
              .build();
        }
      }
    }
    return result;
  }
}
