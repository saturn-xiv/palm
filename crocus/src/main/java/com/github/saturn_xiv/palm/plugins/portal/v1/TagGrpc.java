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
public final class TagGrpc {

  private TagGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.portal.v1.Tag";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.TagIndexResponse> getIndexMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Index",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.TagIndexResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.TagIndexResponse> getIndexMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.TagIndexResponse> getIndexMethod;
    if ((getIndexMethod = TagGrpc.getIndexMethod) == null) {
      synchronized (TagGrpc.class) {
        if ((getIndexMethod = TagGrpc.getIndexMethod) == null) {
          TagGrpc.getIndexMethod = getIndexMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.TagIndexResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Index"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.TagIndexResponse.getDefaultInstance()))
              .setSchemaDescriptor(new TagMethodDescriptorSupplier("Index"))
              .build();
        }
      }
    }
    return getIndexMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static TagStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<TagStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<TagStub>() {
        @java.lang.Override
        public TagStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new TagStub(channel, callOptions);
        }
      };
    return TagStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static TagBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<TagBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<TagBlockingV2Stub>() {
        @java.lang.Override
        public TagBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new TagBlockingV2Stub(channel, callOptions);
        }
      };
    return TagBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static TagBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<TagBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<TagBlockingStub>() {
        @java.lang.Override
        public TagBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new TagBlockingStub(channel, callOptions);
        }
      };
    return TagBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static TagFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<TagFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<TagFutureStub>() {
        @java.lang.Override
        public TagFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new TagFutureStub(channel, callOptions);
        }
      };
    return TagFutureStub.newStub(factory, channel);
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
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.TagIndexResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getIndexMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service Tag.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static abstract class TagImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return TagGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Tag.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class TagStub
      extends io.grpc.stub.AbstractAsyncStub<TagStub> {
    private TagStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected TagStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new TagStub(channel, callOptions);
    }

    /**
     */
    public void index(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.TagIndexResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Tag.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class TagBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<TagBlockingV2Stub> {
    private TagBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected TagBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new TagBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.TagIndexResponse index(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getIndexMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Tag.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class TagBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<TagBlockingStub> {
    private TagBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected TagBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new TagBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.TagIndexResponse index(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getIndexMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Tag.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class TagFutureStub
      extends io.grpc.stub.AbstractFutureStub<TagFutureStub> {
    private TagFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected TagFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new TagFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.TagIndexResponse> index(
        com.google.protobuf.Empty request) {
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
          serviceImpl.index((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.TagIndexResponse>) responseObserver);
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
              com.github.saturn_xiv.palm.plugins.portal.v1.TagIndexResponse>(
                service, METHODID_INDEX)))
        .build();
  }

  private static abstract class TagBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    TagBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.portal.v1.Portal.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Tag");
    }
  }

  private static final class TagFileDescriptorSupplier
      extends TagBaseDescriptorSupplier {
    TagFileDescriptorSupplier() {}
  }

  private static final class TagMethodDescriptorSupplier
      extends TagBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    TagMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (TagGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new TagFileDescriptorSupplier())
              .addMethod(getIndexMethod())
              .build();
        }
      }
    }
    return result;
  }
}
