package com.github.saturn_xiv.palm.plugins.portal.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 * <pre>
 * ----------------------------------------------------------------------------
 * </pre>
 */
@javax.annotation.Generated(
    value = "by gRPC proto compiler (version 1.71.0)",
    comments = "Source: portal.proto")
@io.grpc.stub.annotations.GrpcGenerated
public final class CategoryGrpc {

  private CategoryGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.portal.v1.Category";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse> getIndexMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Index",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse> getIndexMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse> getIndexMethod;
    if ((getIndexMethod = CategoryGrpc.getIndexMethod) == null) {
      synchronized (CategoryGrpc.class) {
        if ((getIndexMethod = CategoryGrpc.getIndexMethod) == null) {
          CategoryGrpc.getIndexMethod = getIndexMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Index"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CategoryMethodDescriptorSupplier("Index"))
              .build();
        }
      }
    }
    return getIndexMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.ByCodeRequest,
      com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse> getByCodeMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ByCode",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.ByCodeRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.ByCodeRequest,
      com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse> getByCodeMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.ByCodeRequest, com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse> getByCodeMethod;
    if ((getByCodeMethod = CategoryGrpc.getByCodeMethod) == null) {
      synchronized (CategoryGrpc.class) {
        if ((getByCodeMethod = CategoryGrpc.getByCodeMethod) == null) {
          CategoryGrpc.getByCodeMethod = getByCodeMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.ByCodeRequest, com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ByCode"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.ByCodeRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CategoryMethodDescriptorSupplier("ByCode"))
              .build();
        }
      }
    }
    return getByCodeMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static CategoryStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CategoryStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CategoryStub>() {
        @java.lang.Override
        public CategoryStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CategoryStub(channel, callOptions);
        }
      };
    return CategoryStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static CategoryBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CategoryBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CategoryBlockingV2Stub>() {
        @java.lang.Override
        public CategoryBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CategoryBlockingV2Stub(channel, callOptions);
        }
      };
    return CategoryBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static CategoryBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CategoryBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CategoryBlockingStub>() {
        @java.lang.Override
        public CategoryBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CategoryBlockingStub(channel, callOptions);
        }
      };
    return CategoryBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static CategoryFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CategoryFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CategoryFutureStub>() {
        @java.lang.Override
        public CategoryFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CategoryFutureStub(channel, callOptions);
        }
      };
    return CategoryFutureStub.newStub(factory, channel);
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
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getIndexMethod(), responseObserver);
    }

    /**
     */
    default void byCode(com.github.saturn_xiv.palm.plugins.portal.v1.ByCodeRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getByCodeMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service Category.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static abstract class CategoryImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return CategoryGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Category.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class CategoryStub
      extends io.grpc.stub.AbstractAsyncStub<CategoryStub> {
    private CategoryStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CategoryStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CategoryStub(channel, callOptions);
    }

    /**
     */
    public void index(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void byCode(com.github.saturn_xiv.palm.plugins.portal.v1.ByCodeRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getByCodeMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Category.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class CategoryBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<CategoryBlockingV2Stub> {
    private CategoryBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CategoryBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CategoryBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse index(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getIndexMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse byCode(com.github.saturn_xiv.palm.plugins.portal.v1.ByCodeRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getByCodeMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Category.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class CategoryBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<CategoryBlockingStub> {
    private CategoryBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CategoryBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CategoryBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse index(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getIndexMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse byCode(com.github.saturn_xiv.palm.plugins.portal.v1.ByCodeRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getByCodeMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Category.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class CategoryFutureStub
      extends io.grpc.stub.AbstractFutureStub<CategoryFutureStub> {
    private CategoryFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CategoryFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CategoryFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse> index(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse> byCode(
        com.github.saturn_xiv.palm.plugins.portal.v1.ByCodeRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getByCodeMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_INDEX = 0;
  private static final int METHODID_BY_CODE = 1;

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
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse>) responseObserver);
          break;
        case METHODID_BY_CODE:
          serviceImpl.byCode((com.github.saturn_xiv.palm.plugins.portal.v1.ByCodeRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse>) responseObserver);
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
              com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse>(
                service, METHODID_INDEX)))
        .addMethod(
          getByCodeMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.ByCodeRequest,
              com.github.saturn_xiv.palm.plugins.portal.v1.CategoryIndexResponse>(
                service, METHODID_BY_CODE)))
        .build();
  }

  private static abstract class CategoryBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    CategoryBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.portal.v1.Portal.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Category");
    }
  }

  private static final class CategoryFileDescriptorSupplier
      extends CategoryBaseDescriptorSupplier {
    CategoryFileDescriptorSupplier() {}
  }

  private static final class CategoryMethodDescriptorSupplier
      extends CategoryBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    CategoryMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (CategoryGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new CategoryFileDescriptorSupplier())
              .addMethod(getIndexMethod())
              .addMethod(getByCodeMethod())
              .build();
        }
      }
    }
    return result;
  }
}
