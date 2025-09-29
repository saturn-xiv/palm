package com.github.saturn_xiv.palm.plugins.wechat.pay.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class WeChatPayRefundGrpc {

  private WeChatPayRefundGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.wechat.pay.v1.WeChatPayRefund";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCreateRefundRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse> getCreateMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Create",
      requestType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCreateRefundRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCreateRefundRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse> getCreateMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCreateRefundRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse> getCreateMethod;
    if ((getCreateMethod = WeChatPayRefundGrpc.getCreateMethod) == null) {
      synchronized (WeChatPayRefundGrpc.class) {
        if ((getCreateMethod = WeChatPayRefundGrpc.getCreateMethod) == null) {
          WeChatPayRefundGrpc.getCreateMethod = getCreateMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCreateRefundRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Create"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCreateRefundRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse.getDefaultInstance()))
              .setSchemaDescriptor(new WeChatPayRefundMethodDescriptorSupplier("Create"))
              .build();
        }
      }
    }
    return getCreateMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryRefundRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse> getQueryMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Query",
      requestType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryRefundRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryRefundRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse> getQueryMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryRefundRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse> getQueryMethod;
    if ((getQueryMethod = WeChatPayRefundGrpc.getQueryMethod) == null) {
      synchronized (WeChatPayRefundGrpc.class) {
        if ((getQueryMethod = WeChatPayRefundGrpc.getQueryMethod) == null) {
          WeChatPayRefundGrpc.getQueryMethod = getQueryMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryRefundRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Query"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryRefundRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse.getDefaultInstance()))
              .setSchemaDescriptor(new WeChatPayRefundMethodDescriptorSupplier("Query"))
              .build();
        }
      }
    }
    return getQueryMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static WeChatPayRefundStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<WeChatPayRefundStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<WeChatPayRefundStub>() {
        @java.lang.Override
        public WeChatPayRefundStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new WeChatPayRefundStub(channel, callOptions);
        }
      };
    return WeChatPayRefundStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static WeChatPayRefundBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<WeChatPayRefundBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<WeChatPayRefundBlockingV2Stub>() {
        @java.lang.Override
        public WeChatPayRefundBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new WeChatPayRefundBlockingV2Stub(channel, callOptions);
        }
      };
    return WeChatPayRefundBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static WeChatPayRefundBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<WeChatPayRefundBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<WeChatPayRefundBlockingStub>() {
        @java.lang.Override
        public WeChatPayRefundBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new WeChatPayRefundBlockingStub(channel, callOptions);
        }
      };
    return WeChatPayRefundBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static WeChatPayRefundFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<WeChatPayRefundFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<WeChatPayRefundFutureStub>() {
        @java.lang.Override
        public WeChatPayRefundFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new WeChatPayRefundFutureStub(channel, callOptions);
        }
      };
    return WeChatPayRefundFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void create(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCreateRefundRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateMethod(), responseObserver);
    }

    /**
     */
    default void query(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryRefundRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getQueryMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service WeChatPayRefund.
   */
  public static abstract class WeChatPayRefundImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return WeChatPayRefundGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service WeChatPayRefund.
   */
  public static final class WeChatPayRefundStub
      extends io.grpc.stub.AbstractAsyncStub<WeChatPayRefundStub> {
    private WeChatPayRefundStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected WeChatPayRefundStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new WeChatPayRefundStub(channel, callOptions);
    }

    /**
     */
    public void create(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCreateRefundRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void query(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryRefundRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getQueryMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service WeChatPayRefund.
   */
  public static final class WeChatPayRefundBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<WeChatPayRefundBlockingV2Stub> {
    private WeChatPayRefundBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected WeChatPayRefundBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new WeChatPayRefundBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse create(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCreateRefundRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse query(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryRefundRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getQueryMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service WeChatPayRefund.
   */
  public static final class WeChatPayRefundBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<WeChatPayRefundBlockingStub> {
    private WeChatPayRefundBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected WeChatPayRefundBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new WeChatPayRefundBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse create(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCreateRefundRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse query(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryRefundRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getQueryMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service WeChatPayRefund.
   */
  public static final class WeChatPayRefundFutureStub
      extends io.grpc.stub.AbstractFutureStub<WeChatPayRefundFutureStub> {
    private WeChatPayRefundFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected WeChatPayRefundFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new WeChatPayRefundFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse> create(
        com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCreateRefundRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse> query(
        com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryRefundRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getQueryMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_CREATE = 0;
  private static final int METHODID_QUERY = 1;

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
        case METHODID_CREATE:
          serviceImpl.create((com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCreateRefundRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse>) responseObserver);
          break;
        case METHODID_QUERY:
          serviceImpl.query((com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryRefundRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse>) responseObserver);
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
          getCreateMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCreateRefundRequest,
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse>(
                service, METHODID_CREATE)))
        .addMethod(
          getQueryMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryRefundRequest,
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayRefundResponse>(
                service, METHODID_QUERY)))
        .build();
  }

  private static abstract class WeChatPayRefundBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    WeChatPayRefundBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WechatPay.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("WeChatPayRefund");
    }
  }

  private static final class WeChatPayRefundFileDescriptorSupplier
      extends WeChatPayRefundBaseDescriptorSupplier {
    WeChatPayRefundFileDescriptorSupplier() {}
  }

  private static final class WeChatPayRefundMethodDescriptorSupplier
      extends WeChatPayRefundBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    WeChatPayRefundMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (WeChatPayRefundGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new WeChatPayRefundFileDescriptorSupplier())
              .addMethod(getCreateMethod())
              .addMethod(getQueryMethod())
              .build();
        }
      }
    }
    return result;
  }
}
