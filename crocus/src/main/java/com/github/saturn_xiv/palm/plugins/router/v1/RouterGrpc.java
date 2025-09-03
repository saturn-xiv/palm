package com.github.saturn_xiv.palm.plugins.router.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@javax.annotation.Generated(
    value = "by gRPC proto compiler (version 1.71.0)",
    comments = "Source: router.proto")
@io.grpc.stub.annotations.GrpcGenerated
public final class RouterGrpc {

  private RouterGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.router.v1.Router";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse.Item,
      com.google.protobuf.Empty> getSetEthernetMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SetEthernet",
      requestType = com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse.Item.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse.Item,
      com.google.protobuf.Empty> getSetEthernetMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse.Item, com.google.protobuf.Empty> getSetEthernetMethod;
    if ((getSetEthernetMethod = RouterGrpc.getSetEthernetMethod) == null) {
      synchronized (RouterGrpc.class) {
        if ((getSetEthernetMethod = RouterGrpc.getSetEthernetMethod) == null) {
          RouterGrpc.getSetEthernetMethod = getSetEthernetMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse.Item, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SetEthernet"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse.Item.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new RouterMethodDescriptorSupplier("SetEthernet"))
              .build();
        }
      }
    }
    return getSetEthernetMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse> getIndexEthernetMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "IndexEthernet",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse> getIndexEthernetMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse> getIndexEthernetMethod;
    if ((getIndexEthernetMethod = RouterGrpc.getIndexEthernetMethod) == null) {
      synchronized (RouterGrpc.class) {
        if ((getIndexEthernetMethod = RouterGrpc.getIndexEthernetMethod) == null) {
          RouterGrpc.getIndexEthernetMethod = getIndexEthernetMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "IndexEthernet"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse.getDefaultInstance()))
              .setSchemaDescriptor(new RouterMethodDescriptorSupplier("IndexEthernet"))
              .build();
        }
      }
    }
    return getIndexEthernetMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.google.protobuf.Empty> getRebootMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Reboot",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.google.protobuf.Empty> getRebootMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.google.protobuf.Empty> getRebootMethod;
    if ((getRebootMethod = RouterGrpc.getRebootMethod) == null) {
      synchronized (RouterGrpc.class) {
        if ((getRebootMethod = RouterGrpc.getRebootMethod) == null) {
          RouterGrpc.getRebootMethod = getRebootMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Reboot"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new RouterMethodDescriptorSupplier("Reboot"))
              .build();
        }
      }
    }
    return getRebootMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.google.protobuf.Empty> getApplyMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Apply",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.google.protobuf.Empty> getApplyMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.google.protobuf.Empty> getApplyMethod;
    if ((getApplyMethod = RouterGrpc.getApplyMethod) == null) {
      synchronized (RouterGrpc.class) {
        if ((getApplyMethod = RouterGrpc.getApplyMethod) == null) {
          RouterGrpc.getApplyMethod = getApplyMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Apply"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new RouterMethodDescriptorSupplier("Apply"))
              .build();
        }
      }
    }
    return getApplyMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.google.protobuf.Empty> getFactoryResetMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "FactoryReset",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.google.protobuf.Empty> getFactoryResetMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.google.protobuf.Empty> getFactoryResetMethod;
    if ((getFactoryResetMethod = RouterGrpc.getFactoryResetMethod) == null) {
      synchronized (RouterGrpc.class) {
        if ((getFactoryResetMethod = RouterGrpc.getFactoryResetMethod) == null) {
          RouterGrpc.getFactoryResetMethod = getFactoryResetMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "FactoryReset"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new RouterMethodDescriptorSupplier("FactoryReset"))
              .build();
        }
      }
    }
    return getFactoryResetMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static RouterStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RouterStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RouterStub>() {
        @java.lang.Override
        public RouterStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RouterStub(channel, callOptions);
        }
      };
    return RouterStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static RouterBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RouterBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RouterBlockingV2Stub>() {
        @java.lang.Override
        public RouterBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RouterBlockingV2Stub(channel, callOptions);
        }
      };
    return RouterBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static RouterBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RouterBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RouterBlockingStub>() {
        @java.lang.Override
        public RouterBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RouterBlockingStub(channel, callOptions);
        }
      };
    return RouterBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static RouterFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RouterFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RouterFutureStub>() {
        @java.lang.Override
        public RouterFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RouterFutureStub(channel, callOptions);
        }
      };
    return RouterFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void setEthernet(com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse.Item request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSetEthernetMethod(), responseObserver);
    }

    /**
     */
    default void indexEthernet(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getIndexEthernetMethod(), responseObserver);
    }

    /**
     */
    default void reboot(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRebootMethod(), responseObserver);
    }

    /**
     */
    default void apply(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getApplyMethod(), responseObserver);
    }

    /**
     */
    default void factoryReset(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getFactoryResetMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service Router.
   */
  public static abstract class RouterImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return RouterGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Router.
   */
  public static final class RouterStub
      extends io.grpc.stub.AbstractAsyncStub<RouterStub> {
    private RouterStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RouterStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RouterStub(channel, callOptions);
    }

    /**
     */
    public void setEthernet(com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse.Item request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSetEthernetMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void indexEthernet(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getIndexEthernetMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void reboot(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRebootMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void apply(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getApplyMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void factoryReset(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getFactoryResetMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Router.
   */
  public static final class RouterBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<RouterBlockingV2Stub> {
    private RouterBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RouterBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RouterBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.google.protobuf.Empty setEthernet(com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse.Item request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetEthernetMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse indexEthernet(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getIndexEthernetMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty reboot(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRebootMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty apply(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getApplyMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty factoryReset(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getFactoryResetMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Router.
   */
  public static final class RouterBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<RouterBlockingStub> {
    private RouterBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RouterBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RouterBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.google.protobuf.Empty setEthernet(com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse.Item request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetEthernetMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse indexEthernet(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getIndexEthernetMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty reboot(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRebootMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty apply(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getApplyMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty factoryReset(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getFactoryResetMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Router.
   */
  public static final class RouterFutureStub
      extends io.grpc.stub.AbstractFutureStub<RouterFutureStub> {
    private RouterFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RouterFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RouterFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> setEthernet(
        com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse.Item request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSetEthernetMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse> indexEthernet(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getIndexEthernetMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> reboot(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRebootMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> apply(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getApplyMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> factoryReset(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getFactoryResetMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_SET_ETHERNET = 0;
  private static final int METHODID_INDEX_ETHERNET = 1;
  private static final int METHODID_REBOOT = 2;
  private static final int METHODID_APPLY = 3;
  private static final int METHODID_FACTORY_RESET = 4;

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
        case METHODID_SET_ETHERNET:
          serviceImpl.setEthernet((com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse.Item) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_INDEX_ETHERNET:
          serviceImpl.indexEthernet((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse>) responseObserver);
          break;
        case METHODID_REBOOT:
          serviceImpl.reboot((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_APPLY:
          serviceImpl.apply((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_FACTORY_RESET:
          serviceImpl.factoryReset((com.google.protobuf.Empty) request,
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
          getSetEthernetMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse.Item,
              com.google.protobuf.Empty>(
                service, METHODID_SET_ETHERNET)))
        .addMethod(
          getIndexEthernetMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.router.v1.RouterIndexEthernetResponse>(
                service, METHODID_INDEX_ETHERNET)))
        .addMethod(
          getRebootMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.google.protobuf.Empty>(
                service, METHODID_REBOOT)))
        .addMethod(
          getApplyMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.google.protobuf.Empty>(
                service, METHODID_APPLY)))
        .addMethod(
          getFactoryResetMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.google.protobuf.Empty>(
                service, METHODID_FACTORY_RESET)))
        .build();
  }

  private static abstract class RouterBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    RouterBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.router.v1.RouterOuterClass.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Router");
    }
  }

  private static final class RouterFileDescriptorSupplier
      extends RouterBaseDescriptorSupplier {
    RouterFileDescriptorSupplier() {}
  }

  private static final class RouterMethodDescriptorSupplier
      extends RouterBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    RouterMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (RouterGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new RouterFileDescriptorSupplier())
              .addMethod(getSetEthernetMethod())
              .addMethod(getIndexEthernetMethod())
              .addMethod(getRebootMethod())
              .addMethod(getApplyMethod())
              .addMethod(getFactoryResetMethod())
              .build();
        }
      }
    }
    return result;
  }
}
