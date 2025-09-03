package com.github.saturn_xiv.palm.plugins.wechat.pay.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@javax.annotation.Generated(
    value = "by gRPC proto compiler (version 1.71.0)",
    comments = "Source: wechat-pay.proto")
@io.grpc.stub.annotations.GrpcGenerated
public final class WeChatPayBillGrpc {

  private WeChatPayBillGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.wechat.pay.v1.WeChatPayBill";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeBillRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse> getTradeMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Trade",
      requestType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeBillRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeBillRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse> getTradeMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeBillRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse> getTradeMethod;
    if ((getTradeMethod = WeChatPayBillGrpc.getTradeMethod) == null) {
      synchronized (WeChatPayBillGrpc.class) {
        if ((getTradeMethod = WeChatPayBillGrpc.getTradeMethod) == null) {
          WeChatPayBillGrpc.getTradeMethod = getTradeMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeBillRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Trade"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeBillRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse.getDefaultInstance()))
              .setSchemaDescriptor(new WeChatPayBillMethodDescriptorSupplier("Trade"))
              .build();
        }
      }
    }
    return getTradeMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayFundFlowBillRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse> getFundFlowMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "FundFlow",
      requestType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayFundFlowBillRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayFundFlowBillRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse> getFundFlowMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayFundFlowBillRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse> getFundFlowMethod;
    if ((getFundFlowMethod = WeChatPayBillGrpc.getFundFlowMethod) == null) {
      synchronized (WeChatPayBillGrpc.class) {
        if ((getFundFlowMethod = WeChatPayBillGrpc.getFundFlowMethod) == null) {
          WeChatPayBillGrpc.getFundFlowMethod = getFundFlowMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayFundFlowBillRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "FundFlow"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayFundFlowBillRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse.getDefaultInstance()))
              .setSchemaDescriptor(new WeChatPayBillMethodDescriptorSupplier("FundFlow"))
              .build();
        }
      }
    }
    return getFundFlowMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static WeChatPayBillStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<WeChatPayBillStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<WeChatPayBillStub>() {
        @java.lang.Override
        public WeChatPayBillStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new WeChatPayBillStub(channel, callOptions);
        }
      };
    return WeChatPayBillStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static WeChatPayBillBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<WeChatPayBillBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<WeChatPayBillBlockingV2Stub>() {
        @java.lang.Override
        public WeChatPayBillBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new WeChatPayBillBlockingV2Stub(channel, callOptions);
        }
      };
    return WeChatPayBillBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static WeChatPayBillBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<WeChatPayBillBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<WeChatPayBillBlockingStub>() {
        @java.lang.Override
        public WeChatPayBillBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new WeChatPayBillBlockingStub(channel, callOptions);
        }
      };
    return WeChatPayBillBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static WeChatPayBillFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<WeChatPayBillFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<WeChatPayBillFutureStub>() {
        @java.lang.Override
        public WeChatPayBillFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new WeChatPayBillFutureStub(channel, callOptions);
        }
      };
    return WeChatPayBillFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void trade(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeBillRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getTradeMethod(), responseObserver);
    }

    /**
     */
    default void fundFlow(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayFundFlowBillRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getFundFlowMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service WeChatPayBill.
   */
  public static abstract class WeChatPayBillImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return WeChatPayBillGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service WeChatPayBill.
   */
  public static final class WeChatPayBillStub
      extends io.grpc.stub.AbstractAsyncStub<WeChatPayBillStub> {
    private WeChatPayBillStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected WeChatPayBillStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new WeChatPayBillStub(channel, callOptions);
    }

    /**
     */
    public void trade(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeBillRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getTradeMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void fundFlow(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayFundFlowBillRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getFundFlowMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service WeChatPayBill.
   */
  public static final class WeChatPayBillBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<WeChatPayBillBlockingV2Stub> {
    private WeChatPayBillBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected WeChatPayBillBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new WeChatPayBillBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse trade(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeBillRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getTradeMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse fundFlow(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayFundFlowBillRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getFundFlowMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service WeChatPayBill.
   */
  public static final class WeChatPayBillBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<WeChatPayBillBlockingStub> {
    private WeChatPayBillBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected WeChatPayBillBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new WeChatPayBillBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse trade(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeBillRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getTradeMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse fundFlow(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayFundFlowBillRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getFundFlowMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service WeChatPayBill.
   */
  public static final class WeChatPayBillFutureStub
      extends io.grpc.stub.AbstractFutureStub<WeChatPayBillFutureStub> {
    private WeChatPayBillFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected WeChatPayBillFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new WeChatPayBillFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse> trade(
        com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeBillRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getTradeMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse> fundFlow(
        com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayFundFlowBillRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getFundFlowMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_TRADE = 0;
  private static final int METHODID_FUND_FLOW = 1;

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
        case METHODID_TRADE:
          serviceImpl.trade((com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeBillRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse>) responseObserver);
          break;
        case METHODID_FUND_FLOW:
          serviceImpl.fundFlow((com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayFundFlowBillRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse>) responseObserver);
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
          getTradeMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeBillRequest,
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse>(
                service, METHODID_TRADE)))
        .addMethod(
          getFundFlowMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayFundFlowBillRequest,
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayBillResponse>(
                service, METHODID_FUND_FLOW)))
        .build();
  }

  private static abstract class WeChatPayBillBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    WeChatPayBillBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WechatPay.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("WeChatPayBill");
    }
  }

  private static final class WeChatPayBillFileDescriptorSupplier
      extends WeChatPayBillBaseDescriptorSupplier {
    WeChatPayBillFileDescriptorSupplier() {}
  }

  private static final class WeChatPayBillMethodDescriptorSupplier
      extends WeChatPayBillBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    WeChatPayBillMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (WeChatPayBillGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new WeChatPayBillFileDescriptorSupplier())
              .addMethod(getTradeMethod())
              .addMethod(getFundFlowMethod())
              .build();
        }
      }
    }
    return result;
  }
}
