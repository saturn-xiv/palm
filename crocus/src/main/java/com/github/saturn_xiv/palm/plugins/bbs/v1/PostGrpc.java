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
public final class PostGrpc {

  private PostGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.bbs.v1.Post";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page,
      com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse> getIndexMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Index",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.Page.class,
      responseType = com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page,
      com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse> getIndexMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page, com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse> getIndexMethod;
    if ((getIndexMethod = PostGrpc.getIndexMethod) == null) {
      synchronized (PostGrpc.class) {
        if ((getIndexMethod = PostGrpc.getIndexMethod) == null) {
          PostGrpc.getIndexMethod = getIndexMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.Page, com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Index"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.Page.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PostMethodDescriptorSupplier("Index"))
              .build();
        }
      }
    }
    return getIndexMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.PostByArticleRequest,
      com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse> getByArticleMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ByArticle",
      requestType = com.github.saturn_xiv.palm.plugins.bbs.v1.PostByArticleRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.PostByArticleRequest,
      com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse> getByArticleMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.PostByArticleRequest, com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse> getByArticleMethod;
    if ((getByArticleMethod = PostGrpc.getByArticleMethod) == null) {
      synchronized (PostGrpc.class) {
        if ((getByArticleMethod = PostGrpc.getByArticleMethod) == null) {
          PostGrpc.getByArticleMethod = getByArticleMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.bbs.v1.PostByArticleRequest, com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ByArticle"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.bbs.v1.PostByArticleRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PostMethodDescriptorSupplier("ByArticle"))
              .build();
        }
      }
    }
    return getByArticleMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.PostByUserRequest,
      com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse> getByUserMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ByUser",
      requestType = com.github.saturn_xiv.palm.plugins.bbs.v1.PostByUserRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.PostByUserRequest,
      com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse> getByUserMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.PostByUserRequest, com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse> getByUserMethod;
    if ((getByUserMethod = PostGrpc.getByUserMethod) == null) {
      synchronized (PostGrpc.class) {
        if ((getByUserMethod = PostGrpc.getByUserMethod) == null) {
          PostGrpc.getByUserMethod = getByUserMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.bbs.v1.PostByUserRequest, com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ByUser"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.bbs.v1.PostByUserRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PostMethodDescriptorSupplier("ByUser"))
              .build();
        }
      }
    }
    return getByUserMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.CreatePostRequest,
      com.google.protobuf.Empty> getCreateMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Create",
      requestType = com.github.saturn_xiv.palm.plugins.bbs.v1.CreatePostRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.CreatePostRequest,
      com.google.protobuf.Empty> getCreateMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.CreatePostRequest, com.google.protobuf.Empty> getCreateMethod;
    if ((getCreateMethod = PostGrpc.getCreateMethod) == null) {
      synchronized (PostGrpc.class) {
        if ((getCreateMethod = PostGrpc.getCreateMethod) == null) {
          PostGrpc.getCreateMethod = getCreateMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.bbs.v1.CreatePostRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Create"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.bbs.v1.CreatePostRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new PostMethodDescriptorSupplier("Create"))
              .build();
        }
      }
    }
    return getCreateMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.UpdatePostRequest,
      com.google.protobuf.Empty> getUpdateMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Update",
      requestType = com.github.saturn_xiv.palm.plugins.bbs.v1.UpdatePostRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.UpdatePostRequest,
      com.google.protobuf.Empty> getUpdateMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.UpdatePostRequest, com.google.protobuf.Empty> getUpdateMethod;
    if ((getUpdateMethod = PostGrpc.getUpdateMethod) == null) {
      synchronized (PostGrpc.class) {
        if ((getUpdateMethod = PostGrpc.getUpdateMethod) == null) {
          PostGrpc.getUpdateMethod = getUpdateMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.bbs.v1.UpdatePostRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Update"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.bbs.v1.UpdatePostRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new PostMethodDescriptorSupplier("Update"))
              .build();
        }
      }
    }
    return getUpdateMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest,
      com.google.protobuf.Empty> getDeleteMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Delete",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest,
      com.google.protobuf.Empty> getDeleteMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest, com.google.protobuf.Empty> getDeleteMethod;
    if ((getDeleteMethod = PostGrpc.getDeleteMethod) == null) {
      synchronized (PostGrpc.class) {
        if ((getDeleteMethod = PostGrpc.getDeleteMethod) == null) {
          PostGrpc.getDeleteMethod = getDeleteMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Delete"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new PostMethodDescriptorSupplier("Delete"))
              .build();
        }
      }
    }
    return getDeleteMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static PostStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PostStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PostStub>() {
        @java.lang.Override
        public PostStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PostStub(channel, callOptions);
        }
      };
    return PostStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static PostBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PostBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PostBlockingV2Stub>() {
        @java.lang.Override
        public PostBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PostBlockingV2Stub(channel, callOptions);
        }
      };
    return PostBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static PostBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PostBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PostBlockingStub>() {
        @java.lang.Override
        public PostBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PostBlockingStub(channel, callOptions);
        }
      };
    return PostBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static PostFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PostFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PostFutureStub>() {
        @java.lang.Override
        public PostFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PostFutureStub(channel, callOptions);
        }
      };
    return PostFutureStub.newStub(factory, channel);
  }

  /**
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public interface AsyncService {

    /**
     */
    default void index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getIndexMethod(), responseObserver);
    }

    /**
     */
    default void byArticle(com.github.saturn_xiv.palm.plugins.bbs.v1.PostByArticleRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getByArticleMethod(), responseObserver);
    }

    /**
     */
    default void byUser(com.github.saturn_xiv.palm.plugins.bbs.v1.PostByUserRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getByUserMethod(), responseObserver);
    }

    /**
     */
    default void create(com.github.saturn_xiv.palm.plugins.bbs.v1.CreatePostRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateMethod(), responseObserver);
    }

    /**
     */
    default void update(com.github.saturn_xiv.palm.plugins.bbs.v1.UpdatePostRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getUpdateMethod(), responseObserver);
    }

    /**
     */
    default void delete(com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeleteMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service Post.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static abstract class PostImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return PostGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Post.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class PostStub
      extends io.grpc.stub.AbstractAsyncStub<PostStub> {
    private PostStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PostStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PostStub(channel, callOptions);
    }

    /**
     */
    public void index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void byArticle(com.github.saturn_xiv.palm.plugins.bbs.v1.PostByArticleRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getByArticleMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void byUser(com.github.saturn_xiv.palm.plugins.bbs.v1.PostByUserRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getByUserMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void create(com.github.saturn_xiv.palm.plugins.bbs.v1.CreatePostRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void update(com.github.saturn_xiv.palm.plugins.bbs.v1.UpdatePostRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getUpdateMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void delete(com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeleteMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Post.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class PostBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<PostBlockingV2Stub> {
    private PostBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PostBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PostBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getIndexMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse byArticle(com.github.saturn_xiv.palm.plugins.bbs.v1.PostByArticleRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getByArticleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse byUser(com.github.saturn_xiv.palm.plugins.bbs.v1.PostByUserRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getByUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty create(com.github.saturn_xiv.palm.plugins.bbs.v1.CreatePostRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty update(com.github.saturn_xiv.palm.plugins.bbs.v1.UpdatePostRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getUpdateMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty delete(com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeleteMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Post.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class PostBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<PostBlockingStub> {
    private PostBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PostBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PostBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getIndexMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse byArticle(com.github.saturn_xiv.palm.plugins.bbs.v1.PostByArticleRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getByArticleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse byUser(com.github.saturn_xiv.palm.plugins.bbs.v1.PostByUserRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getByUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty create(com.github.saturn_xiv.palm.plugins.bbs.v1.CreatePostRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty update(com.github.saturn_xiv.palm.plugins.bbs.v1.UpdatePostRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getUpdateMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty delete(com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeleteMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Post.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class PostFutureStub
      extends io.grpc.stub.AbstractFutureStub<PostFutureStub> {
    private PostFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PostFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PostFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse> index(
        com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse> byArticle(
        com.github.saturn_xiv.palm.plugins.bbs.v1.PostByArticleRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getByArticleMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse> byUser(
        com.github.saturn_xiv.palm.plugins.bbs.v1.PostByUserRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getByUserMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> create(
        com.github.saturn_xiv.palm.plugins.bbs.v1.CreatePostRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> update(
        com.github.saturn_xiv.palm.plugins.bbs.v1.UpdatePostRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getUpdateMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> delete(
        com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeleteMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_INDEX = 0;
  private static final int METHODID_BY_ARTICLE = 1;
  private static final int METHODID_BY_USER = 2;
  private static final int METHODID_CREATE = 3;
  private static final int METHODID_UPDATE = 4;
  private static final int METHODID_DELETE = 5;

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
          serviceImpl.index((com.github.saturn_xiv.palm.plugins.portal.v1.Page) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse>) responseObserver);
          break;
        case METHODID_BY_ARTICLE:
          serviceImpl.byArticle((com.github.saturn_xiv.palm.plugins.bbs.v1.PostByArticleRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse>) responseObserver);
          break;
        case METHODID_BY_USER:
          serviceImpl.byUser((com.github.saturn_xiv.palm.plugins.bbs.v1.PostByUserRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse>) responseObserver);
          break;
        case METHODID_CREATE:
          serviceImpl.create((com.github.saturn_xiv.palm.plugins.bbs.v1.CreatePostRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_UPDATE:
          serviceImpl.update((com.github.saturn_xiv.palm.plugins.bbs.v1.UpdatePostRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_DELETE:
          serviceImpl.delete((com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest) request,
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
              com.github.saturn_xiv.palm.plugins.portal.v1.Page,
              com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse>(
                service, METHODID_INDEX)))
        .addMethod(
          getByArticleMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.bbs.v1.PostByArticleRequest,
              com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse>(
                service, METHODID_BY_ARTICLE)))
        .addMethod(
          getByUserMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.bbs.v1.PostByUserRequest,
              com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse>(
                service, METHODID_BY_USER)))
        .addMethod(
          getCreateMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.bbs.v1.CreatePostRequest,
              com.google.protobuf.Empty>(
                service, METHODID_CREATE)))
        .addMethod(
          getUpdateMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.bbs.v1.UpdatePostRequest,
              com.google.protobuf.Empty>(
                service, METHODID_UPDATE)))
        .addMethod(
          getDeleteMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest,
              com.google.protobuf.Empty>(
                service, METHODID_DELETE)))
        .build();
  }

  private static abstract class PostBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    PostBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.bbs.v1.Bbs.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Post");
    }
  }

  private static final class PostFileDescriptorSupplier
      extends PostBaseDescriptorSupplier {
    PostFileDescriptorSupplier() {}
  }

  private static final class PostMethodDescriptorSupplier
      extends PostBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    PostMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (PostGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new PostFileDescriptorSupplier())
              .addMethod(getIndexMethod())
              .addMethod(getByArticleMethod())
              .addMethod(getByUserMethod())
              .addMethod(getCreateMethod())
              .addMethod(getUpdateMethod())
              .addMethod(getDeleteMethod())
              .build();
        }
      }
    }
    return result;
  }
}
