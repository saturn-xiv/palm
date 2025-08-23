package com.github.saturn_xiv.palm.plugins.wechat.pay.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@javax.annotation.Generated(
    value = "by gRPC proto compiler (version 1.68.1)",
    comments = "Source: wechat-pay.proto")
@io.grpc.stub.annotations.GrpcGenerated
public final class WeChatPayNativeGrpc {

  private WeChatPayNativeGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.wechat.pay.v1.WeChatPayNative";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayPrepayRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayNativeQrCodeUrlResponse> getPrepayMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Prepay",
      requestType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayPrepayRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayNativeQrCodeUrlResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayPrepayRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayNativeQrCodeUrlResponse> getPrepayMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayPrepayRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayNativeQrCodeUrlResponse> getPrepayMethod;
    if ((getPrepayMethod = WeChatPayNativeGrpc.getPrepayMethod) == null) {
      synchronized (WeChatPayNativeGrpc.class) {
        if ((getPrepayMethod = WeChatPayNativeGrpc.getPrepayMethod) == null) {
          WeChatPayNativeGrpc.getPrepayMethod = getPrepayMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayPrepayRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayNativeQrCodeUrlResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Prepay"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayPrepayRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayNativeQrCodeUrlResponse.getDefaultInstance()))
              .setSchemaDescriptor(new WeChatPayNativeMethodDescriptorSupplier("Prepay"))
              .build();
        }
      }
    }
    return getPrepayMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static WeChatPayNativeStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<WeChatPayNativeStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<WeChatPayNativeStub>() {
        @java.lang.Override
        public WeChatPayNativeStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new WeChatPayNativeStub(channel, callOptions);
        }
      };
    return WeChatPayNativeStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static WeChatPayNativeBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<WeChatPayNativeBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<WeChatPayNativeBlockingStub>() {
        @java.lang.Override
        public WeChatPayNativeBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new WeChatPayNativeBlockingStub(channel, callOptions);
        }
      };
    return WeChatPayNativeBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static WeChatPayNativeFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<WeChatPayNativeFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<WeChatPayNativeFutureStub>() {
        @java.lang.Override
        public WeChatPayNativeFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new WeChatPayNativeFutureStub(channel, callOptions);
        }
      };
    return WeChatPayNativeFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void prepay(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayPrepayRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayNativeQrCodeUrlResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getPrepayMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service WeChatPayNative.
   */
  public static abstract class WeChatPayNativeImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return WeChatPayNativeGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service WeChatPayNative.
   */
  public static final class WeChatPayNativeStub
      extends io.grpc.stub.AbstractAsyncStub<WeChatPayNativeStub> {
    private WeChatPayNativeStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected WeChatPayNativeStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new WeChatPayNativeStub(channel, callOptions);
    }

    /**
     */
    public void prepay(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayPrepayRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayNativeQrCodeUrlResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getPrepayMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service WeChatPayNative.
   */
  public static final class WeChatPayNativeBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<WeChatPayNativeBlockingStub> {
    private WeChatPayNativeBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected WeChatPayNativeBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new WeChatPayNativeBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayNativeQrCodeUrlResponse prepay(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayPrepayRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getPrepayMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service WeChatPayNative.
   */
  public static final class WeChatPayNativeFutureStub
      extends io.grpc.stub.AbstractFutureStub<WeChatPayNativeFutureStub> {
    private WeChatPayNativeFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected WeChatPayNativeFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new WeChatPayNativeFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayNativeQrCodeUrlResponse> prepay(
        com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayPrepayRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getPrepayMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_PREPAY = 0;

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
        case METHODID_PREPAY:
          serviceImpl.prepay((com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayPrepayRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayNativeQrCodeUrlResponse>) responseObserver);
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
          getPrepayMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayPrepayRequest,
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayNativeQrCodeUrlResponse>(
                service, METHODID_PREPAY)))
        .build();
  }

  private static abstract class WeChatPayNativeBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    WeChatPayNativeBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WechatPay.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("WeChatPayNative");
    }
  }

  private static final class WeChatPayNativeFileDescriptorSupplier
      extends WeChatPayNativeBaseDescriptorSupplier {
    WeChatPayNativeFileDescriptorSupplier() {}
  }

  private static final class WeChatPayNativeMethodDescriptorSupplier
      extends WeChatPayNativeBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    WeChatPayNativeMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (WeChatPayNativeGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new WeChatPayNativeFileDescriptorSupplier())
              .addMethod(getPrepayMethod())
              .build();
        }
      }
    }
    return result;
  }
}
