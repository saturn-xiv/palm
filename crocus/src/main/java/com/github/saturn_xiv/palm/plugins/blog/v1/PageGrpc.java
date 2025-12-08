package com.github.saturn_xiv.palm.plugins.blog.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 * <pre>
 * ----------------------------------------------------------------------------
 * </pre>
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class PageGrpc {

  private PageGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.blog.v1.Page";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page,
      com.github.saturn_xiv.palm.plugins.blog.v1.IndexPageResponse> getIndexMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Index",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.Page.class,
      responseType = com.github.saturn_xiv.palm.plugins.blog.v1.IndexPageResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page,
      com.github.saturn_xiv.palm.plugins.blog.v1.IndexPageResponse> getIndexMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page, com.github.saturn_xiv.palm.plugins.blog.v1.IndexPageResponse> getIndexMethod;
    if ((getIndexMethod = PageGrpc.getIndexMethod) == null) {
      synchronized (PageGrpc.class) {
        if ((getIndexMethod = PageGrpc.getIndexMethod) == null) {
          PageGrpc.getIndexMethod = getIndexMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.Page, com.github.saturn_xiv.palm.plugins.blog.v1.IndexPageResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Index"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.Page.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.blog.v1.IndexPageResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PageMethodDescriptorSupplier("Index"))
              .build();
        }
      }
    }
    return getIndexMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static PageStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PageStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PageStub>() {
        @java.lang.Override
        public PageStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PageStub(channel, callOptions);
        }
      };
    return PageStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static PageBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PageBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PageBlockingV2Stub>() {
        @java.lang.Override
        public PageBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PageBlockingV2Stub(channel, callOptions);
        }
      };
    return PageBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static PageBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PageBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PageBlockingStub>() {
        @java.lang.Override
        public PageBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PageBlockingStub(channel, callOptions);
        }
      };
    return PageBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static PageFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PageFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PageFutureStub>() {
        @java.lang.Override
        public PageFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PageFutureStub(channel, callOptions);
        }
      };
    return PageFutureStub.newStub(factory, channel);
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
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.blog.v1.IndexPageResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getIndexMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service Page.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static abstract class PageImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return PageGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Page.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class PageStub
      extends io.grpc.stub.AbstractAsyncStub<PageStub> {
    private PageStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PageStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PageStub(channel, callOptions);
    }

    /**
     */
    public void index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.blog.v1.IndexPageResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Page.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class PageBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<PageBlockingV2Stub> {
    private PageBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PageBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PageBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.blog.v1.IndexPageResponse index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getIndexMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Page.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class PageBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<PageBlockingStub> {
    private PageBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PageBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PageBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.blog.v1.IndexPageResponse index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getIndexMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Page.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class PageFutureStub
      extends io.grpc.stub.AbstractFutureStub<PageFutureStub> {
    private PageFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PageFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PageFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.blog.v1.IndexPageResponse> index(
        com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_INDEX = 0;

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
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.blog.v1.IndexPageResponse>) responseObserver);
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
              com.github.saturn_xiv.palm.plugins.blog.v1.IndexPageResponse>(
                service, METHODID_INDEX)))
        .build();
  }

  private static abstract class PageBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    PageBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.blog.v1.BlogProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Page");
    }
  }

  private static final class PageFileDescriptorSupplier
      extends PageBaseDescriptorSupplier {
    PageFileDescriptorSupplier() {}
  }

  private static final class PageMethodDescriptorSupplier
      extends PageBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    PageMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (PageGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new PageFileDescriptorSupplier())
              .addMethod(getIndexMethod())
              .build();
        }
      }
    }
    return result;
  }
}
