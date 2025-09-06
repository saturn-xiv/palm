package com.github.saturn_xiv.palm.plugins.bbs.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 * <pre>
 * ----------------------------------------------------------------------------
 * </pre>
 */
@javax.annotation.Generated(
    value = "by gRPC proto compiler (version 1.71.0)",
    comments = "Source: bbs.proto")
@io.grpc.stub.annotations.GrpcGenerated
public final class BoardGrpc {

  private BoardGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.bbs.v1.Board";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse> getIndexMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Index",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse> getIndexMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse> getIndexMethod;
    if ((getIndexMethod = BoardGrpc.getIndexMethod) == null) {
      synchronized (BoardGrpc.class) {
        if ((getIndexMethod = BoardGrpc.getIndexMethod) == null) {
          BoardGrpc.getIndexMethod = getIndexMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Index"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BoardMethodDescriptorSupplier("Index"))
              .build();
        }
      }
    }
    return getIndexMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.ByLangRequest,
      com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse> getByLangMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ByLang",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.ByLangRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.ByLangRequest,
      com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse> getByLangMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.ByLangRequest, com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse> getByLangMethod;
    if ((getByLangMethod = BoardGrpc.getByLangMethod) == null) {
      synchronized (BoardGrpc.class) {
        if ((getByLangMethod = BoardGrpc.getByLangMethod) == null) {
          BoardGrpc.getByLangMethod = getByLangMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.ByLangRequest, com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ByLang"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.ByLangRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BoardMethodDescriptorSupplier("ByLang"))
              .build();
        }
      }
    }
    return getByLangMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.BoardCreateRequest,
      com.google.protobuf.Empty> getCreateMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Create",
      requestType = com.github.saturn_xiv.palm.plugins.bbs.v1.BoardCreateRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.BoardCreateRequest,
      com.google.protobuf.Empty> getCreateMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.BoardCreateRequest, com.google.protobuf.Empty> getCreateMethod;
    if ((getCreateMethod = BoardGrpc.getCreateMethod) == null) {
      synchronized (BoardGrpc.class) {
        if ((getCreateMethod = BoardGrpc.getCreateMethod) == null) {
          BoardGrpc.getCreateMethod = getCreateMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.bbs.v1.BoardCreateRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Create"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.bbs.v1.BoardCreateRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new BoardMethodDescriptorSupplier("Create"))
              .build();
        }
      }
    }
    return getCreateMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static BoardStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<BoardStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<BoardStub>() {
        @java.lang.Override
        public BoardStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new BoardStub(channel, callOptions);
        }
      };
    return BoardStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static BoardBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<BoardBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<BoardBlockingV2Stub>() {
        @java.lang.Override
        public BoardBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new BoardBlockingV2Stub(channel, callOptions);
        }
      };
    return BoardBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static BoardBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<BoardBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<BoardBlockingStub>() {
        @java.lang.Override
        public BoardBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new BoardBlockingStub(channel, callOptions);
        }
      };
    return BoardBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static BoardFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<BoardFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<BoardFutureStub>() {
        @java.lang.Override
        public BoardFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new BoardFutureStub(channel, callOptions);
        }
      };
    return BoardFutureStub.newStub(factory, channel);
  }

  /**
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public interface AsyncService {

    /**
     */
    default void index(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getIndexMethod(), responseObserver);
    }

    /**
     */
    default void byLang(com.github.saturn_xiv.palm.plugins.portal.v1.ByLangRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getByLangMethod(), responseObserver);
    }

    /**
     */
    default void create(com.github.saturn_xiv.palm.plugins.bbs.v1.BoardCreateRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service Board.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static abstract class BoardImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return BoardGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Board.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class BoardStub
      extends io.grpc.stub.AbstractAsyncStub<BoardStub> {
    private BoardStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected BoardStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new BoardStub(channel, callOptions);
    }

    /**
     */
    public void index(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void byLang(com.github.saturn_xiv.palm.plugins.portal.v1.ByLangRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getByLangMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void create(com.github.saturn_xiv.palm.plugins.bbs.v1.BoardCreateRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Board.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class BoardBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<BoardBlockingV2Stub> {
    private BoardBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected BoardBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new BoardBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse index(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getIndexMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse byLang(com.github.saturn_xiv.palm.plugins.portal.v1.ByLangRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getByLangMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty create(com.github.saturn_xiv.palm.plugins.bbs.v1.BoardCreateRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Board.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class BoardBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<BoardBlockingStub> {
    private BoardBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected BoardBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new BoardBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse index(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getIndexMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse byLang(com.github.saturn_xiv.palm.plugins.portal.v1.ByLangRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getByLangMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty create(com.github.saturn_xiv.palm.plugins.bbs.v1.BoardCreateRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Board.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class BoardFutureStub
      extends io.grpc.stub.AbstractFutureStub<BoardFutureStub> {
    private BoardFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected BoardFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new BoardFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse> index(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse> byLang(
        com.github.saturn_xiv.palm.plugins.portal.v1.ByLangRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getByLangMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> create(
        com.github.saturn_xiv.palm.plugins.bbs.v1.BoardCreateRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_INDEX = 0;
  private static final int METHODID_BY_LANG = 1;
  private static final int METHODID_CREATE = 2;

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
        case METHODID_INDEX:
          serviceImpl.index((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse>) responseObserver);
          break;
        case METHODID_BY_LANG:
          serviceImpl.byLang((com.github.saturn_xiv.palm.plugins.portal.v1.ByLangRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse>) responseObserver);
          break;
        case METHODID_CREATE:
          serviceImpl.create((com.github.saturn_xiv.palm.plugins.bbs.v1.BoardCreateRequest) request,
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
          getIndexMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse>(
                service, METHODID_INDEX)))
        .addMethod(
          getByLangMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.ByLangRequest,
              com.github.saturn_xiv.palm.plugins.bbs.v1.BoardIndexResponse>(
                service, METHODID_BY_LANG)))
        .addMethod(
          getCreateMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.bbs.v1.BoardCreateRequest,
              com.google.protobuf.Empty>(
                service, METHODID_CREATE)))
        .build();
  }

  private static abstract class BoardBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    BoardBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.bbs.v1.Bbs.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Board");
    }
  }

  private static final class BoardFileDescriptorSupplier
      extends BoardBaseDescriptorSupplier {
    BoardFileDescriptorSupplier() {}
  }

  private static final class BoardMethodDescriptorSupplier
      extends BoardBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    BoardMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (BoardGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new BoardFileDescriptorSupplier())
              .addMethod(getIndexMethod())
              .addMethod(getByLangMethod())
              .addMethod(getCreateMethod())
              .build();
        }
      }
    }
    return result;
  }
}
