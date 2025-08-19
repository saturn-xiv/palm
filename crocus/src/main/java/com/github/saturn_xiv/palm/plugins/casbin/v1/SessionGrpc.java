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
public final class SessionGrpc {

  private SessionGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.casbin.v1.Session";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.Role,
      com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> getHasMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Has",
      requestType = com.github.saturn_xiv.palm.plugins.casbin.v1.Role.class,
      responseType = com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.Role,
      com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> getHasMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.Role, com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> getHasMethod;
    if ((getHasMethod = SessionGrpc.getHasMethod) == null) {
      synchronized (SessionGrpc.class) {
        if ((getHasMethod = SessionGrpc.getHasMethod) == null) {
          SessionGrpc.getHasMethod = getHasMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.casbin.v1.Role, com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Has"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.Role.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SessionMethodDescriptorSupplier("Has"))
              .build();
        }
      }
    }
    return getHasMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.SessionCanRequest,
      com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> getCanMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Can",
      requestType = com.github.saturn_xiv.palm.plugins.casbin.v1.SessionCanRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.SessionCanRequest,
      com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> getCanMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.casbin.v1.SessionCanRequest, com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> getCanMethod;
    if ((getCanMethod = SessionGrpc.getCanMethod) == null) {
      synchronized (SessionGrpc.class) {
        if ((getCanMethod = SessionGrpc.getCanMethod) == null) {
          SessionGrpc.getCanMethod = getCanMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.casbin.v1.SessionCanRequest, com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Can"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.SessionCanRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SessionMethodDescriptorSupplier("Can"))
              .build();
        }
      }
    }
    return getCanMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> getRolesMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Roles",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> getRolesMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> getRolesMethod;
    if ((getRolesMethod = SessionGrpc.getRolesMethod) == null) {
      synchronized (SessionGrpc.class) {
        if ((getRolesMethod = SessionGrpc.getRolesMethod) == null) {
          SessionGrpc.getRolesMethod = getRolesMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Roles"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SessionMethodDescriptorSupplier("Roles"))
              .build();
        }
      }
    }
    return getRolesMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getPermissionsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Permissions",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getPermissionsMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getPermissionsMethod;
    if ((getPermissionsMethod = SessionGrpc.getPermissionsMethod) == null) {
      synchronized (SessionGrpc.class) {
        if ((getPermissionsMethod = SessionGrpc.getPermissionsMethod) == null) {
          SessionGrpc.getPermissionsMethod = getPermissionsMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Permissions"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SessionMethodDescriptorSupplier("Permissions"))
              .build();
        }
      }
    }
    return getPermissionsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> getImplicitRolesMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ImplicitRoles",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> getImplicitRolesMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> getImplicitRolesMethod;
    if ((getImplicitRolesMethod = SessionGrpc.getImplicitRolesMethod) == null) {
      synchronized (SessionGrpc.class) {
        if ((getImplicitRolesMethod = SessionGrpc.getImplicitRolesMethod) == null) {
          SessionGrpc.getImplicitRolesMethod = getImplicitRolesMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ImplicitRoles"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SessionMethodDescriptorSupplier("ImplicitRoles"))
              .build();
        }
      }
    }
    return getImplicitRolesMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getImplicitPermissionsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ImplicitPermissions",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getImplicitPermissionsMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> getImplicitPermissionsMethod;
    if ((getImplicitPermissionsMethod = SessionGrpc.getImplicitPermissionsMethod) == null) {
      synchronized (SessionGrpc.class) {
        if ((getImplicitPermissionsMethod = SessionGrpc.getImplicitPermissionsMethod) == null) {
          SessionGrpc.getImplicitPermissionsMethod = getImplicitPermissionsMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ImplicitPermissions"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SessionMethodDescriptorSupplier("ImplicitPermissions"))
              .build();
        }
      }
    }
    return getImplicitPermissionsMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static SessionStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SessionStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SessionStub>() {
        @java.lang.Override
        public SessionStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SessionStub(channel, callOptions);
        }
      };
    return SessionStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static SessionBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SessionBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SessionBlockingStub>() {
        @java.lang.Override
        public SessionBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SessionBlockingStub(channel, callOptions);
        }
      };
    return SessionBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static SessionFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SessionFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SessionFutureStub>() {
        @java.lang.Override
        public SessionFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SessionFutureStub(channel, callOptions);
        }
      };
    return SessionFutureStub.newStub(factory, channel);
  }

  /**
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public interface AsyncService {

    /**
     */
    default void has(com.github.saturn_xiv.palm.plugins.casbin.v1.Role request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getHasMethod(), responseObserver);
    }

    /**
     */
    default void can(com.github.saturn_xiv.palm.plugins.casbin.v1.SessionCanRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCanMethod(), responseObserver);
    }

    /**
     */
    default void roles(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRolesMethod(), responseObserver);
    }

    /**
     */
    default void permissions(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getPermissionsMethod(), responseObserver);
    }

    /**
     */
    default void implicitRoles(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getImplicitRolesMethod(), responseObserver);
    }

    /**
     */
    default void implicitPermissions(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getImplicitPermissionsMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service Session.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static abstract class SessionImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return SessionGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Session.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class SessionStub
      extends io.grpc.stub.AbstractAsyncStub<SessionStub> {
    private SessionStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SessionStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SessionStub(channel, callOptions);
    }

    /**
     */
    public void has(com.github.saturn_xiv.palm.plugins.casbin.v1.Role request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getHasMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void can(com.github.saturn_xiv.palm.plugins.casbin.v1.SessionCanRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCanMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void roles(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRolesMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void permissions(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getPermissionsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void implicitRoles(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getImplicitRolesMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void implicitPermissions(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getImplicitPermissionsMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Session.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class SessionBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<SessionBlockingStub> {
    private SessionBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SessionBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SessionBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse has(com.github.saturn_xiv.palm.plugins.casbin.v1.Role request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getHasMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse can(com.github.saturn_xiv.palm.plugins.casbin.v1.SessionCanRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCanMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse roles(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRolesMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse permissions(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getPermissionsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse implicitRoles(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getImplicitRolesMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse implicitPermissions(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getImplicitPermissionsMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Session.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class SessionFutureStub
      extends io.grpc.stub.AbstractFutureStub<SessionFutureStub> {
    private SessionFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SessionFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SessionFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> has(
        com.github.saturn_xiv.palm.plugins.casbin.v1.Role request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getHasMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse> can(
        com.github.saturn_xiv.palm.plugins.casbin.v1.SessionCanRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCanMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> roles(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRolesMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> permissions(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getPermissionsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse> implicitRoles(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getImplicitRolesMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse> implicitPermissions(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getImplicitPermissionsMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_HAS = 0;
  private static final int METHODID_CAN = 1;
  private static final int METHODID_ROLES = 2;
  private static final int METHODID_PERMISSIONS = 3;
  private static final int METHODID_IMPLICIT_ROLES = 4;
  private static final int METHODID_IMPLICIT_PERMISSIONS = 5;

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
        case METHODID_HAS:
          serviceImpl.has((com.github.saturn_xiv.palm.plugins.casbin.v1.Role) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse>) responseObserver);
          break;
        case METHODID_CAN:
          serviceImpl.can((com.github.saturn_xiv.palm.plugins.casbin.v1.SessionCanRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse>) responseObserver);
          break;
        case METHODID_ROLES:
          serviceImpl.roles((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse>) responseObserver);
          break;
        case METHODID_PERMISSIONS:
          serviceImpl.permissions((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse>) responseObserver);
          break;
        case METHODID_IMPLICIT_ROLES:
          serviceImpl.implicitRoles((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse>) responseObserver);
          break;
        case METHODID_IMPLICIT_PERMISSIONS:
          serviceImpl.implicitPermissions((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse>) responseObserver);
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
          getHasMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.casbin.v1.Role,
              com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse>(
                service, METHODID_HAS)))
        .addMethod(
          getCanMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.casbin.v1.SessionCanRequest,
              com.github.saturn_xiv.palm.plugins.casbin.v1.BoolResponse>(
                service, METHODID_CAN)))
        .addMethod(
          getRolesMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse>(
                service, METHODID_ROLES)))
        .addMethod(
          getPermissionsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse>(
                service, METHODID_PERMISSIONS)))
        .addMethod(
          getImplicitRolesMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.casbin.v1.RolesResponse>(
                service, METHODID_IMPLICIT_ROLES)))
        .addMethod(
          getImplicitPermissionsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.casbin.v1.PermissionsResponse>(
                service, METHODID_IMPLICIT_PERMISSIONS)))
        .build();
  }

  private static abstract class SessionBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    SessionBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.casbin.v1.Casbin.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Session");
    }
  }

  private static final class SessionFileDescriptorSupplier
      extends SessionBaseDescriptorSupplier {
    SessionFileDescriptorSupplier() {}
  }

  private static final class SessionMethodDescriptorSupplier
      extends SessionBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    SessionMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (SessionGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new SessionFileDescriptorSupplier())
              .addMethod(getHasMethod())
              .addMethod(getCanMethod())
              .addMethod(getRolesMethod())
              .addMethod(getPermissionsMethod())
              .addMethod(getImplicitRolesMethod())
              .addMethod(getImplicitPermissionsMethod())
              .build();
        }
      }
    }
    return result;
  }
}
