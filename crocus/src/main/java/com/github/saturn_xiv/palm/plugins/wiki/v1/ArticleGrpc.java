package com.github.saturn_xiv.palm.plugins.wiki.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@javax.annotation.Generated(
    value = "by gRPC proto compiler (version 1.71.0)",
    comments = "Source: wiki.proto")
@io.grpc.stub.annotations.GrpcGenerated
public final class ArticleGrpc {

  private ArticleGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.wiki.v1.Article";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.google.protobuf.Empty> getSaveMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Save",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.google.protobuf.Empty> getSaveMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.google.protobuf.Empty> getSaveMethod;
    if ((getSaveMethod = ArticleGrpc.getSaveMethod) == null) {
      synchronized (ArticleGrpc.class) {
        if ((getSaveMethod = ArticleGrpc.getSaveMethod) == null) {
          ArticleGrpc.getSaveMethod = getSaveMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Save"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new ArticleMethodDescriptorSupplier("Save"))
              .build();
        }
      }
    }
    return getSaveMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleRequest,
      com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleResponse> getShowByTitleMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ShowByTitle",
      requestType = com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleRequest,
      com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleResponse> getShowByTitleMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleRequest, com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleResponse> getShowByTitleMethod;
    if ((getShowByTitleMethod = ArticleGrpc.getShowByTitleMethod) == null) {
      synchronized (ArticleGrpc.class) {
        if ((getShowByTitleMethod = ArticleGrpc.getShowByTitleMethod) == null) {
          ArticleGrpc.getShowByTitleMethod = getShowByTitleMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleRequest, com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ShowByTitle"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ArticleMethodDescriptorSupplier("ShowByTitle"))
              .build();
        }
      }
    }
    return getShowByTitleMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page,
      com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleIndexResponse> getIndexMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Index",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.Page.class,
      responseType = com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleIndexResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page,
      com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleIndexResponse> getIndexMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page, com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleIndexResponse> getIndexMethod;
    if ((getIndexMethod = ArticleGrpc.getIndexMethod) == null) {
      synchronized (ArticleGrpc.class) {
        if ((getIndexMethod = ArticleGrpc.getIndexMethod) == null) {
          ArticleGrpc.getIndexMethod = getIndexMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.Page, com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleIndexResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Index"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.Page.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleIndexResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ArticleMethodDescriptorSupplier("Index"))
              .build();
        }
      }
    }
    return getIndexMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page,
      com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleHistoryResponse> getHistoryMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "History",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.Page.class,
      responseType = com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleHistoryResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page,
      com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleHistoryResponse> getHistoryMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page, com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleHistoryResponse> getHistoryMethod;
    if ((getHistoryMethod = ArticleGrpc.getHistoryMethod) == null) {
      synchronized (ArticleGrpc.class) {
        if ((getHistoryMethod = ArticleGrpc.getHistoryMethod) == null) {
          ArticleGrpc.getHistoryMethod = getHistoryMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.Page, com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleHistoryResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "History"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.Page.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleHistoryResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ArticleMethodDescriptorSupplier("History"))
              .build();
        }
      }
    }
    return getHistoryMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static ArticleStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<ArticleStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<ArticleStub>() {
        @java.lang.Override
        public ArticleStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new ArticleStub(channel, callOptions);
        }
      };
    return ArticleStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static ArticleBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<ArticleBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<ArticleBlockingV2Stub>() {
        @java.lang.Override
        public ArticleBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new ArticleBlockingV2Stub(channel, callOptions);
        }
      };
    return ArticleBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static ArticleBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<ArticleBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<ArticleBlockingStub>() {
        @java.lang.Override
        public ArticleBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new ArticleBlockingStub(channel, callOptions);
        }
      };
    return ArticleBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static ArticleFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<ArticleFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<ArticleFutureStub>() {
        @java.lang.Override
        public ArticleFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new ArticleFutureStub(channel, callOptions);
        }
      };
    return ArticleFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void save(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSaveMethod(), responseObserver);
    }

    /**
     */
    default void showByTitle(com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getShowByTitleMethod(), responseObserver);
    }

    /**
     */
    default void index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleIndexResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getIndexMethod(), responseObserver);
    }

    /**
     */
    default void history(com.github.saturn_xiv.palm.plugins.portal.v1.Page request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleHistoryResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getHistoryMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service Article.
   */
  public static abstract class ArticleImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return ArticleGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Article.
   */
  public static final class ArticleStub
      extends io.grpc.stub.AbstractAsyncStub<ArticleStub> {
    private ArticleStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected ArticleStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new ArticleStub(channel, callOptions);
    }

    /**
     */
    public void save(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSaveMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void showByTitle(com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getShowByTitleMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleIndexResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void history(com.github.saturn_xiv.palm.plugins.portal.v1.Page request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleHistoryResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getHistoryMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Article.
   */
  public static final class ArticleBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<ArticleBlockingV2Stub> {
    private ArticleBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected ArticleBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new ArticleBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.google.protobuf.Empty save(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSaveMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleResponse showByTitle(com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getShowByTitleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleIndexResponse index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getIndexMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleHistoryResponse history(com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getHistoryMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Article.
   */
  public static final class ArticleBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<ArticleBlockingStub> {
    private ArticleBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected ArticleBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new ArticleBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.google.protobuf.Empty save(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSaveMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleResponse showByTitle(com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getShowByTitleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleIndexResponse index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getIndexMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleHistoryResponse history(com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getHistoryMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Article.
   */
  public static final class ArticleFutureStub
      extends io.grpc.stub.AbstractFutureStub<ArticleFutureStub> {
    private ArticleFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected ArticleFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new ArticleFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> save(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSaveMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleResponse> showByTitle(
        com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getShowByTitleMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleIndexResponse> index(
        com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleHistoryResponse> history(
        com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getHistoryMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_SAVE = 0;
  private static final int METHODID_SHOW_BY_TITLE = 1;
  private static final int METHODID_INDEX = 2;
  private static final int METHODID_HISTORY = 3;

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
        case METHODID_SAVE:
          serviceImpl.save((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_SHOW_BY_TITLE:
          serviceImpl.showByTitle((com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleResponse>) responseObserver);
          break;
        case METHODID_INDEX:
          serviceImpl.index((com.github.saturn_xiv.palm.plugins.portal.v1.Page) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleIndexResponse>) responseObserver);
          break;
        case METHODID_HISTORY:
          serviceImpl.history((com.github.saturn_xiv.palm.plugins.portal.v1.Page) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleHistoryResponse>) responseObserver);
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
          getSaveMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.google.protobuf.Empty>(
                service, METHODID_SAVE)))
        .addMethod(
          getShowByTitleMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleRequest,
              com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleShowByTitleResponse>(
                service, METHODID_SHOW_BY_TITLE)))
        .addMethod(
          getIndexMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.Page,
              com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleIndexResponse>(
                service, METHODID_INDEX)))
        .addMethod(
          getHistoryMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.Page,
              com.github.saturn_xiv.palm.plugins.wiki.v1.ArticleHistoryResponse>(
                service, METHODID_HISTORY)))
        .build();
  }

  private static abstract class ArticleBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    ArticleBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.wiki.v1.Wiki.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Article");
    }
  }

  private static final class ArticleFileDescriptorSupplier
      extends ArticleBaseDescriptorSupplier {
    ArticleFileDescriptorSupplier() {}
  }

  private static final class ArticleMethodDescriptorSupplier
      extends ArticleBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    ArticleMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (ArticleGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new ArticleFileDescriptorSupplier())
              .addMethod(getSaveMethod())
              .addMethod(getShowByTitleMethod())
              .addMethod(getIndexMethod())
              .addMethod(getHistoryMethod())
              .build();
        }
      }
    }
    return result;
  }
}
