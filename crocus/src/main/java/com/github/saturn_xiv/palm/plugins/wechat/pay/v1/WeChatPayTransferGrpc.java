package com.github.saturn_xiv.palm.plugins.wechat.pay.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@javax.annotation.Generated(
    value = "by gRPC proto compiler (version 1.71.0)",
    comments = "Source: wechat-pay.proto")
@io.grpc.stub.annotations.GrpcGenerated
public final class WeChatPayTransferGrpc {

  private WeChatPayTransferGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.wechat.pay.v1.WeChatPayTransfer";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferResponse> getExecuteBatchMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ExecuteBatch",
      requestType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferResponse> getExecuteBatchMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferResponse> getExecuteBatchMethod;
    if ((getExecuteBatchMethod = WeChatPayTransferGrpc.getExecuteBatchMethod) == null) {
      synchronized (WeChatPayTransferGrpc.class) {
        if ((getExecuteBatchMethod = WeChatPayTransferGrpc.getExecuteBatchMethod) == null) {
          WeChatPayTransferGrpc.getExecuteBatchMethod = getExecuteBatchMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ExecuteBatch"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferResponse.getDefaultInstance()))
              .setSchemaDescriptor(new WeChatPayTransferMethodDescriptorSupplier("ExecuteBatch"))
              .build();
        }
      }
    }
    return getExecuteBatchMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferResponse> getQueryBatchMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "QueryBatch",
      requestType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferResponse> getQueryBatchMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferResponse> getQueryBatchMethod;
    if ((getQueryBatchMethod = WeChatPayTransferGrpc.getQueryBatchMethod) == null) {
      synchronized (WeChatPayTransferGrpc.class) {
        if ((getQueryBatchMethod = WeChatPayTransferGrpc.getQueryBatchMethod) == null) {
          WeChatPayTransferGrpc.getQueryBatchMethod = getQueryBatchMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "QueryBatch"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferResponse.getDefaultInstance()))
              .setSchemaDescriptor(new WeChatPayTransferMethodDescriptorSupplier("QueryBatch"))
              .build();
        }
      }
    }
    return getQueryBatchMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailResponse> getQueryDetailMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "QueryDetail",
      requestType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailResponse> getQueryDetailMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailResponse> getQueryDetailMethod;
    if ((getQueryDetailMethod = WeChatPayTransferGrpc.getQueryDetailMethod) == null) {
      synchronized (WeChatPayTransferGrpc.class) {
        if ((getQueryDetailMethod = WeChatPayTransferGrpc.getQueryDetailMethod) == null) {
          WeChatPayTransferGrpc.getQueryDetailMethod = getQueryDetailMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "QueryDetail"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailResponse.getDefaultInstance()))
              .setSchemaDescriptor(new WeChatPayTransferMethodDescriptorSupplier("QueryDetail"))
              .build();
        }
      }
    }
    return getQueryDetailMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetBillReceiptRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse> getGetBillReceiptMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetBillReceipt",
      requestType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetBillReceiptRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetBillReceiptRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse> getGetBillReceiptMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetBillReceiptRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse> getGetBillReceiptMethod;
    if ((getGetBillReceiptMethod = WeChatPayTransferGrpc.getGetBillReceiptMethod) == null) {
      synchronized (WeChatPayTransferGrpc.class) {
        if ((getGetBillReceiptMethod = WeChatPayTransferGrpc.getGetBillReceiptMethod) == null) {
          WeChatPayTransferGrpc.getGetBillReceiptMethod = getGetBillReceiptMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetBillReceiptRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetBillReceipt"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetBillReceiptRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse.getDefaultInstance()))
              .setSchemaDescriptor(new WeChatPayTransferMethodDescriptorSupplier("GetBillReceipt"))
              .build();
        }
      }
    }
    return getGetBillReceiptMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetElectronicReceiptRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse> getGetElectronicReceiptMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetElectronicReceipt",
      requestType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetElectronicReceiptRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetElectronicReceiptRequest,
      com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse> getGetElectronicReceiptMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetElectronicReceiptRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse> getGetElectronicReceiptMethod;
    if ((getGetElectronicReceiptMethod = WeChatPayTransferGrpc.getGetElectronicReceiptMethod) == null) {
      synchronized (WeChatPayTransferGrpc.class) {
        if ((getGetElectronicReceiptMethod = WeChatPayTransferGrpc.getGetElectronicReceiptMethod) == null) {
          WeChatPayTransferGrpc.getGetElectronicReceiptMethod = getGetElectronicReceiptMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetElectronicReceiptRequest, com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetElectronicReceipt"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetElectronicReceiptRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse.getDefaultInstance()))
              .setSchemaDescriptor(new WeChatPayTransferMethodDescriptorSupplier("GetElectronicReceipt"))
              .build();
        }
      }
    }
    return getGetElectronicReceiptMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static WeChatPayTransferStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<WeChatPayTransferStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<WeChatPayTransferStub>() {
        @java.lang.Override
        public WeChatPayTransferStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new WeChatPayTransferStub(channel, callOptions);
        }
      };
    return WeChatPayTransferStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static WeChatPayTransferBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<WeChatPayTransferBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<WeChatPayTransferBlockingV2Stub>() {
        @java.lang.Override
        public WeChatPayTransferBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new WeChatPayTransferBlockingV2Stub(channel, callOptions);
        }
      };
    return WeChatPayTransferBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static WeChatPayTransferBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<WeChatPayTransferBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<WeChatPayTransferBlockingStub>() {
        @java.lang.Override
        public WeChatPayTransferBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new WeChatPayTransferBlockingStub(channel, callOptions);
        }
      };
    return WeChatPayTransferBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static WeChatPayTransferFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<WeChatPayTransferFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<WeChatPayTransferFutureStub>() {
        @java.lang.Override
        public WeChatPayTransferFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new WeChatPayTransferFutureStub(channel, callOptions);
        }
      };
    return WeChatPayTransferFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void executeBatch(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getExecuteBatchMethod(), responseObserver);
    }

    /**
     */
    default void queryBatch(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getQueryBatchMethod(), responseObserver);
    }

    /**
     */
    default void queryDetail(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getQueryDetailMethod(), responseObserver);
    }

    /**
     */
    default void getBillReceipt(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetBillReceiptRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetBillReceiptMethod(), responseObserver);
    }

    /**
     */
    default void getElectronicReceipt(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetElectronicReceiptRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetElectronicReceiptMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service WeChatPayTransfer.
   */
  public static abstract class WeChatPayTransferImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return WeChatPayTransferGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service WeChatPayTransfer.
   */
  public static final class WeChatPayTransferStub
      extends io.grpc.stub.AbstractAsyncStub<WeChatPayTransferStub> {
    private WeChatPayTransferStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected WeChatPayTransferStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new WeChatPayTransferStub(channel, callOptions);
    }

    /**
     */
    public void executeBatch(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getExecuteBatchMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void queryBatch(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getQueryBatchMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void queryDetail(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getQueryDetailMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getBillReceipt(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetBillReceiptRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetBillReceiptMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getElectronicReceipt(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetElectronicReceiptRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetElectronicReceiptMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service WeChatPayTransfer.
   */
  public static final class WeChatPayTransferBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<WeChatPayTransferBlockingV2Stub> {
    private WeChatPayTransferBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected WeChatPayTransferBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new WeChatPayTransferBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferResponse executeBatch(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getExecuteBatchMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferResponse queryBatch(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getQueryBatchMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailResponse queryDetail(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getQueryDetailMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse getBillReceipt(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetBillReceiptRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetBillReceiptMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse getElectronicReceipt(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetElectronicReceiptRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetElectronicReceiptMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service WeChatPayTransfer.
   */
  public static final class WeChatPayTransferBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<WeChatPayTransferBlockingStub> {
    private WeChatPayTransferBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected WeChatPayTransferBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new WeChatPayTransferBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferResponse executeBatch(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getExecuteBatchMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferResponse queryBatch(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getQueryBatchMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailResponse queryDetail(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getQueryDetailMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse getBillReceipt(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetBillReceiptRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetBillReceiptMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse getElectronicReceipt(com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetElectronicReceiptRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetElectronicReceiptMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service WeChatPayTransfer.
   */
  public static final class WeChatPayTransferFutureStub
      extends io.grpc.stub.AbstractFutureStub<WeChatPayTransferFutureStub> {
    private WeChatPayTransferFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected WeChatPayTransferFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new WeChatPayTransferFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferResponse> executeBatch(
        com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getExecuteBatchMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferResponse> queryBatch(
        com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getQueryBatchMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailResponse> queryDetail(
        com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getQueryDetailMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse> getBillReceipt(
        com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetBillReceiptRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetBillReceiptMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse> getElectronicReceipt(
        com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetElectronicReceiptRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetElectronicReceiptMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_EXECUTE_BATCH = 0;
  private static final int METHODID_QUERY_BATCH = 1;
  private static final int METHODID_QUERY_DETAIL = 2;
  private static final int METHODID_GET_BILL_RECEIPT = 3;
  private static final int METHODID_GET_ELECTRONIC_RECEIPT = 4;

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
        case METHODID_EXECUTE_BATCH:
          serviceImpl.executeBatch((com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferResponse>) responseObserver);
          break;
        case METHODID_QUERY_BATCH:
          serviceImpl.queryBatch((com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferResponse>) responseObserver);
          break;
        case METHODID_QUERY_DETAIL:
          serviceImpl.queryDetail((com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailResponse>) responseObserver);
          break;
        case METHODID_GET_BILL_RECEIPT:
          serviceImpl.getBillReceipt((com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetBillReceiptRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse>) responseObserver);
          break;
        case METHODID_GET_ELECTRONIC_RECEIPT:
          serviceImpl.getElectronicReceipt((com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetElectronicReceiptRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse>) responseObserver);
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
          getExecuteBatchMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferRequest,
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayExecuteBatchTransferResponse>(
                service, METHODID_EXECUTE_BATCH)))
        .addMethod(
          getQueryBatchMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferRequest,
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryBatchTransferResponse>(
                service, METHODID_QUERY_BATCH)))
        .addMethod(
          getQueryDetailMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailRequest,
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayQueryTransferDetailResponse>(
                service, METHODID_QUERY_DETAIL)))
        .addMethod(
          getGetBillReceiptMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetBillReceiptRequest,
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse>(
                service, METHODID_GET_BILL_RECEIPT)))
        .addMethod(
          getGetElectronicReceiptMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetElectronicReceiptRequest,
              com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WeChatPayTransferGetReceiptResponse>(
                service, METHODID_GET_ELECTRONIC_RECEIPT)))
        .build();
  }

  private static abstract class WeChatPayTransferBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    WeChatPayTransferBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.wechat.pay.v1.WechatPay.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("WeChatPayTransfer");
    }
  }

  private static final class WeChatPayTransferFileDescriptorSupplier
      extends WeChatPayTransferBaseDescriptorSupplier {
    WeChatPayTransferFileDescriptorSupplier() {}
  }

  private static final class WeChatPayTransferMethodDescriptorSupplier
      extends WeChatPayTransferBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    WeChatPayTransferMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (WeChatPayTransferGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new WeChatPayTransferFileDescriptorSupplier())
              .addMethod(getExecuteBatchMethod())
              .addMethod(getQueryBatchMethod())
              .addMethod(getQueryDetailMethod())
              .addMethod(getGetBillReceiptMethod())
              .addMethod(getGetElectronicReceiptMethod())
              .build();
        }
      }
    }
    return result;
  }
}
