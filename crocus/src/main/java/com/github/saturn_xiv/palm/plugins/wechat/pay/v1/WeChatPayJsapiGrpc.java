package com.github.saturn_xiv.palm.plugins.wechat.pay.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@javax.annotation.Generated(
    value = "by gRPC proto compiler (version 1.68.1)",
    comments = "Source: wechat-pay.proto")
@io.grpc.stub.annotations.GrpcGenerated
public final class WeChatPayJsapiGrpc {

  private WeChatPayJsapiGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.wechat.pay.v1.WeChatPayJsapi";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayPrepayRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayJsapiPrepayIdResponse> getPrepayMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Prepay",
      requestType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayPrepayRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayJsapiPrepayIdResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayPrepayRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayJsapiPrepayIdResponse> getPrepayMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayPrepayRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayJsapiPrepayIdResponse> getPrepayMethod;
    if ((getPrepayMethod = WeChatPayJsapiGrpc.getPrepayMethod) == null) {
      synchronized (WeChatPayJsapiGrpc.class) {
        if ((getPrepayMethod = WeChatPayJsapiGrpc.getPrepayMethod) == null) {
          WeChatPayJsapiGrpc.getPrepayMethod = getPrepayMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayPrepayRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayJsapiPrepayIdResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Prepay"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayPrepayRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayJsapiPrepayIdResponse.getDefaultInstance()))
              .setSchemaDescriptor(new WeChatPayJsapiMethodDescriptorSupplier("Prepay"))
              .build();
        }
      }
    }
    return getPrepayMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryOrderByOutTradeNoRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeResponse> getQueryOrderByOutTradeNoMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "QueryOrderByOutTradeNo",
      requestType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryOrderByOutTradeNoRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryOrderByOutTradeNoRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeResponse> getQueryOrderByOutTradeNoMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryOrderByOutTradeNoRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeResponse> getQueryOrderByOutTradeNoMethod;
    if ((getQueryOrderByOutTradeNoMethod = WeChatPayJsapiGrpc.getQueryOrderByOutTradeNoMethod) == null) {
      synchronized (WeChatPayJsapiGrpc.class) {
        if ((getQueryOrderByOutTradeNoMethod = WeChatPayJsapiGrpc.getQueryOrderByOutTradeNoMethod) == null) {
          WeChatPayJsapiGrpc.getQueryOrderByOutTradeNoMethod = getQueryOrderByOutTradeNoMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryOrderByOutTradeNoRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "QueryOrderByOutTradeNo"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryOrderByOutTradeNoRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeResponse.getDefaultInstance()))
              .setSchemaDescriptor(new WeChatPayJsapiMethodDescriptorSupplier("QueryOrderByOutTradeNo"))
              .build();
        }
      }
    }
    return getQueryOrderByOutTradeNoMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryOrderByIdRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeResponse> getQueryOrderByIdMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "QueryOrderById",
      requestType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryOrderByIdRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryOrderByIdRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeResponse> getQueryOrderByIdMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryOrderByIdRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeResponse> getQueryOrderByIdMethod;
    if ((getQueryOrderByIdMethod = WeChatPayJsapiGrpc.getQueryOrderByIdMethod) == null) {
      synchronized (WeChatPayJsapiGrpc.class) {
        if ((getQueryOrderByIdMethod = WeChatPayJsapiGrpc.getQueryOrderByIdMethod) == null) {
          WeChatPayJsapiGrpc.getQueryOrderByIdMethod = getQueryOrderByIdMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryOrderByIdRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "QueryOrderById"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryOrderByIdRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeResponse.getDefaultInstance()))
              .setSchemaDescriptor(new WeChatPayJsapiMethodDescriptorSupplier("QueryOrderById"))
              .build();
        }
      }
    }
    return getQueryOrderByIdMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCloseOrderRequest,
      com.google.protobuf.Empty> getCloseOrderMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CloseOrder",
      requestType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCloseOrderRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCloseOrderRequest,
      com.google.protobuf.Empty> getCloseOrderMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCloseOrderRequest, com.google.protobuf.Empty> getCloseOrderMethod;
    if ((getCloseOrderMethod = WeChatPayJsapiGrpc.getCloseOrderMethod) == null) {
      synchronized (WeChatPayJsapiGrpc.class) {
        if ((getCloseOrderMethod = WeChatPayJsapiGrpc.getCloseOrderMethod) == null) {
          WeChatPayJsapiGrpc.getCloseOrderMethod = getCloseOrderMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCloseOrderRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CloseOrder"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCloseOrderRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new WeChatPayJsapiMethodDescriptorSupplier("CloseOrder"))
              .build();
        }
      }
    }
    return getCloseOrderMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static WeChatPayJsapiStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<WeChatPayJsapiStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<WeChatPayJsapiStub>() {
        @java.lang.Override
        public WeChatPayJsapiStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new WeChatPayJsapiStub(channel, callOptions);
        }
      };
    return WeChatPayJsapiStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static WeChatPayJsapiBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<WeChatPayJsapiBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<WeChatPayJsapiBlockingStub>() {
        @java.lang.Override
        public WeChatPayJsapiBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new WeChatPayJsapiBlockingStub(channel, callOptions);
        }
      };
    return WeChatPayJsapiBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static WeChatPayJsapiFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<WeChatPayJsapiFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<WeChatPayJsapiFutureStub>() {
        @java.lang.Override
        public WeChatPayJsapiFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new WeChatPayJsapiFutureStub(channel, callOptions);
        }
      };
    return WeChatPayJsapiFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void prepay(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayPrepayRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayJsapiPrepayIdResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getPrepayMethod(), responseObserver);
    }

    /**
     */
    default void queryOrderByOutTradeNo(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryOrderByOutTradeNoRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getQueryOrderByOutTradeNoMethod(), responseObserver);
    }

    /**
     */
    default void queryOrderById(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryOrderByIdRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getQueryOrderByIdMethod(), responseObserver);
    }

    /**
     */
    default void closeOrder(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCloseOrderRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCloseOrderMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service WeChatPayJsapi.
   */
  public static abstract class WeChatPayJsapiImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return WeChatPayJsapiGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service WeChatPayJsapi.
   */
  public static final class WeChatPayJsapiStub
      extends io.grpc.stub.AbstractAsyncStub<WeChatPayJsapiStub> {
    private WeChatPayJsapiStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected WeChatPayJsapiStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new WeChatPayJsapiStub(channel, callOptions);
    }

    /**
     */
    public void prepay(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayPrepayRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayJsapiPrepayIdResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getPrepayMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void queryOrderByOutTradeNo(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryOrderByOutTradeNoRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getQueryOrderByOutTradeNoMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void queryOrderById(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryOrderByIdRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getQueryOrderByIdMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void closeOrder(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCloseOrderRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCloseOrderMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service WeChatPayJsapi.
   */
  public static final class WeChatPayJsapiBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<WeChatPayJsapiBlockingStub> {
    private WeChatPayJsapiBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected WeChatPayJsapiBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new WeChatPayJsapiBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayJsapiPrepayIdResponse prepay(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayPrepayRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getPrepayMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeResponse queryOrderByOutTradeNo(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryOrderByOutTradeNoRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getQueryOrderByOutTradeNoMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeResponse queryOrderById(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryOrderByIdRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getQueryOrderByIdMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty closeOrder(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCloseOrderRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCloseOrderMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service WeChatPayJsapi.
   */
  public static final class WeChatPayJsapiFutureStub
      extends io.grpc.stub.AbstractFutureStub<WeChatPayJsapiFutureStub> {
    private WeChatPayJsapiFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected WeChatPayJsapiFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new WeChatPayJsapiFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayJsapiPrepayIdResponse> prepay(
        com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayPrepayRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getPrepayMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeResponse> queryOrderByOutTradeNo(
        com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryOrderByOutTradeNoRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getQueryOrderByOutTradeNoMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeResponse> queryOrderById(
        com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryOrderByIdRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getQueryOrderByIdMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> closeOrder(
        com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCloseOrderRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCloseOrderMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_PREPAY = 0;
  private static final int METHODID_QUERY_ORDER_BY_OUT_TRADE_NO = 1;
  private static final int METHODID_QUERY_ORDER_BY_ID = 2;
  private static final int METHODID_CLOSE_ORDER = 3;

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
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayJsapiPrepayIdResponse>) responseObserver);
          break;
        case METHODID_QUERY_ORDER_BY_OUT_TRADE_NO:
          serviceImpl.queryOrderByOutTradeNo((com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryOrderByOutTradeNoRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeResponse>) responseObserver);
          break;
        case METHODID_QUERY_ORDER_BY_ID:
          serviceImpl.queryOrderById((com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryOrderByIdRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeResponse>) responseObserver);
          break;
        case METHODID_CLOSE_ORDER:
          serviceImpl.closeOrder((com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCloseOrderRequest) request,
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
          getPrepayMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayPrepayRequest,
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayJsapiPrepayIdResponse>(
                service, METHODID_PREPAY)))
        .addMethod(
          getQueryOrderByOutTradeNoMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryOrderByOutTradeNoRequest,
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeResponse>(
                service, METHODID_QUERY_ORDER_BY_OUT_TRADE_NO)))
        .addMethod(
          getQueryOrderByIdMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryOrderByIdRequest,
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTradeResponse>(
                service, METHODID_QUERY_ORDER_BY_ID)))
        .addMethod(
          getCloseOrderMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayCloseOrderRequest,
              com.google.protobuf.Empty>(
                service, METHODID_CLOSE_ORDER)))
        .build();
  }

  private static abstract class WeChatPayJsapiBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    WeChatPayJsapiBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WechatPay.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("WeChatPayJsapi");
    }
  }

  private static final class WeChatPayJsapiFileDescriptorSupplier
      extends WeChatPayJsapiBaseDescriptorSupplier {
    WeChatPayJsapiFileDescriptorSupplier() {}
  }

  private static final class WeChatPayJsapiMethodDescriptorSupplier
      extends WeChatPayJsapiBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    WeChatPayJsapiMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (WeChatPayJsapiGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new WeChatPayJsapiFileDescriptorSupplier())
              .addMethod(getPrepayMethod())
              .addMethod(getQueryOrderByOutTradeNoMethod())
              .addMethod(getQueryOrderByIdMethod())
              .addMethod(getCloseOrderMethod())
              .build();
        }
      }
    }
    return result;
  }
}
