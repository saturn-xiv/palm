package com.github.saturn_xiv.palm.plugins.s3.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class S3Grpc {

  private S3Grpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.s3.v1.S3";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest,
      com.google.protobuf.Empty> getMakeBucketMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "MakeBucket",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest,
      com.google.protobuf.Empty> getMakeBucketMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest, com.google.protobuf.Empty> getMakeBucketMethod;
    if ((getMakeBucketMethod = S3Grpc.getMakeBucketMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getMakeBucketMethod = S3Grpc.getMakeBucketMethod) == null) {
          S3Grpc.getMakeBucketMethod = getMakeBucketMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "MakeBucket"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("MakeBucket"))
              .build();
        }
      }
    }
    return getMakeBucketMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse> getBucketExistsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "BucketExists",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse> getBucketExistsMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsRequest, com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse> getBucketExistsMethod;
    if ((getBucketExistsMethod = S3Grpc.getBucketExistsMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getBucketExistsMethod = S3Grpc.getBucketExistsMethod) == null) {
          S3Grpc.getBucketExistsMethod = getBucketExistsMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsRequest, com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "BucketExists"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("BucketExists"))
              .build();
        }
      }
    }
    return getBucketExistsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketResponse> getListBucketMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListBucket",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketResponse> getListBucketMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketResponse> getListBucketMethod;
    if ((getListBucketMethod = S3Grpc.getListBucketMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getListBucketMethod = S3Grpc.getListBucketMethod) == null) {
          S3Grpc.getListBucketMethod = getListBucketMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListBucket"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketResponse.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("ListBucket"))
              .build();
        }
      }
    }
    return getListBucketMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectResponse> getPutObjectMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "PutObject",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectResponse> getPutObjectMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectRequest, com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectResponse> getPutObjectMethod;
    if ((getPutObjectMethod = S3Grpc.getPutObjectMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getPutObjectMethod = S3Grpc.getPutObjectMethod) == null) {
          S3Grpc.getPutObjectMethod = getPutObjectMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectRequest, com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "PutObject"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectResponse.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("PutObject"))
              .build();
        }
      }
    }
    return getPutObjectMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.RemoveObjectRequest,
      com.google.protobuf.Empty> getRemoveObjectMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RemoveObject",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.RemoveObjectRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.RemoveObjectRequest,
      com.google.protobuf.Empty> getRemoveObjectMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.RemoveObjectRequest, com.google.protobuf.Empty> getRemoveObjectMethod;
    if ((getRemoveObjectMethod = S3Grpc.getRemoveObjectMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getRemoveObjectMethod = S3Grpc.getRemoveObjectMethod) == null) {
          S3Grpc.getRemoveObjectMethod = getRemoveObjectMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.RemoveObjectRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RemoveObject"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.RemoveObjectRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("RemoveObject"))
              .build();
        }
      }
    }
    return getRemoveObjectMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static S3Stub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<S3Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<S3Stub>() {
        @java.lang.Override
        public S3Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new S3Stub(channel, callOptions);
        }
      };
    return S3Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static S3BlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<S3BlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<S3BlockingV2Stub>() {
        @java.lang.Override
        public S3BlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new S3BlockingV2Stub(channel, callOptions);
        }
      };
    return S3BlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static S3BlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<S3BlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<S3BlockingStub>() {
        @java.lang.Override
        public S3BlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new S3BlockingStub(channel, callOptions);
        }
      };
    return S3BlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static S3FutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<S3FutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<S3FutureStub>() {
        @java.lang.Override
        public S3FutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new S3FutureStub(channel, callOptions);
        }
      };
    return S3FutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void makeBucket(com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getMakeBucketMethod(), responseObserver);
    }

    /**
     */
    default void bucketExists(com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getBucketExistsMethod(), responseObserver);
    }

    /**
     */
    default void listBucket(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListBucketMethod(), responseObserver);
    }

    /**
     */
    default void putObject(com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getPutObjectMethod(), responseObserver);
    }

    /**
     */
    default void removeObject(com.github.saturn_xiv.palm.plugins.s3.v1.RemoveObjectRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRemoveObjectMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service S3.
   */
  public static abstract class S3ImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return S3Grpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service S3.
   */
  public static final class S3Stub
      extends io.grpc.stub.AbstractAsyncStub<S3Stub> {
    private S3Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected S3Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new S3Stub(channel, callOptions);
    }

    /**
     */
    public void makeBucket(com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getMakeBucketMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void bucketExists(com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getBucketExistsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void listBucket(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListBucketMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void putObject(com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getPutObjectMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void removeObject(com.github.saturn_xiv.palm.plugins.s3.v1.RemoveObjectRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRemoveObjectMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service S3.
   */
  public static final class S3BlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<S3BlockingV2Stub> {
    private S3BlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected S3BlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new S3BlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.google.protobuf.Empty makeBucket(com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getMakeBucketMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse bucketExists(com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getBucketExistsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketResponse listBucket(com.google.protobuf.Empty request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListBucketMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectResponse putObject(com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getPutObjectMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty removeObject(com.github.saturn_xiv.palm.plugins.s3.v1.RemoveObjectRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRemoveObjectMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service S3.
   */
  public static final class S3BlockingStub
      extends io.grpc.stub.AbstractBlockingStub<S3BlockingStub> {
    private S3BlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected S3BlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new S3BlockingStub(channel, callOptions);
    }

    /**
     */
    public com.google.protobuf.Empty makeBucket(com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getMakeBucketMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse bucketExists(com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getBucketExistsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketResponse listBucket(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListBucketMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectResponse putObject(com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getPutObjectMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty removeObject(com.github.saturn_xiv.palm.plugins.s3.v1.RemoveObjectRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRemoveObjectMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service S3.
   */
  public static final class S3FutureStub
      extends io.grpc.stub.AbstractFutureStub<S3FutureStub> {
    private S3FutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected S3FutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new S3FutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> makeBucket(
        com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getMakeBucketMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse> bucketExists(
        com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getBucketExistsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketResponse> listBucket(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListBucketMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectResponse> putObject(
        com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getPutObjectMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> removeObject(
        com.github.saturn_xiv.palm.plugins.s3.v1.RemoveObjectRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRemoveObjectMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_MAKE_BUCKET = 0;
  private static final int METHODID_BUCKET_EXISTS = 1;
  private static final int METHODID_LIST_BUCKET = 2;
  private static final int METHODID_PUT_OBJECT = 3;
  private static final int METHODID_REMOVE_OBJECT = 4;

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
        case METHODID_MAKE_BUCKET:
          serviceImpl.makeBucket((com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_BUCKET_EXISTS:
          serviceImpl.bucketExists((com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse>) responseObserver);
          break;
        case METHODID_LIST_BUCKET:
          serviceImpl.listBucket((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketResponse>) responseObserver);
          break;
        case METHODID_PUT_OBJECT:
          serviceImpl.putObject((com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectResponse>) responseObserver);
          break;
        case METHODID_REMOVE_OBJECT:
          serviceImpl.removeObject((com.github.saturn_xiv.palm.plugins.s3.v1.RemoveObjectRequest) request,
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
          getMakeBucketMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest,
              com.google.protobuf.Empty>(
                service, METHODID_MAKE_BUCKET)))
        .addMethod(
          getBucketExistsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsRequest,
              com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse>(
                service, METHODID_BUCKET_EXISTS)))
        .addMethod(
          getListBucketMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketResponse>(
                service, METHODID_LIST_BUCKET)))
        .addMethod(
          getPutObjectMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectRequest,
              com.github.saturn_xiv.palm.plugins.s3.v1.PutObjectResponse>(
                service, METHODID_PUT_OBJECT)))
        .addMethod(
          getRemoveObjectMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.RemoveObjectRequest,
              com.google.protobuf.Empty>(
                service, METHODID_REMOVE_OBJECT)))
        .build();
  }

  private static abstract class S3BaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    S3BaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.s3.v1.S3Proto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("S3");
    }
  }

  private static final class S3FileDescriptorSupplier
      extends S3BaseDescriptorSupplier {
    S3FileDescriptorSupplier() {}
  }

  private static final class S3MethodDescriptorSupplier
      extends S3BaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    S3MethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (S3Grpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new S3FileDescriptorSupplier())
              .addMethod(getMakeBucketMethod())
              .addMethod(getBucketExistsMethod())
              .addMethod(getListBucketMethod())
              .addMethod(getPutObjectMethod())
              .addMethod(getRemoveObjectMethod())
              .build();
        }
      }
    }
    return result;
  }
}
