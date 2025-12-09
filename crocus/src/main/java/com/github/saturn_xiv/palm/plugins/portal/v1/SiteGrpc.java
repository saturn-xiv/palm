package com.github.saturn_xiv.palm.plugins.portal.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class SiteGrpc {

  private SiteGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.portal.v1.Site";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.CurrenciesResponse> getCurrenciesMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Currencies",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.CurrenciesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.CurrenciesResponse> getCurrenciesMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.CurrenciesResponse> getCurrenciesMethod;
    if ((getCurrenciesMethod = SiteGrpc.getCurrenciesMethod) == null) {
      synchronized (SiteGrpc.class) {
        if ((getCurrenciesMethod = SiteGrpc.getCurrenciesMethod) == null) {
          SiteGrpc.getCurrenciesMethod = getCurrenciesMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.CurrenciesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Currencies"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.CurrenciesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SiteMethodDescriptorSupplier("Currencies"))
              .build();
        }
      }
    }
    return getCurrenciesMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static SiteStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SiteStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SiteStub>() {
        @java.lang.Override
        public SiteStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SiteStub(channel, callOptions);
        }
      };
    return SiteStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static SiteBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SiteBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SiteBlockingV2Stub>() {
        @java.lang.Override
        public SiteBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SiteBlockingV2Stub(channel, callOptions);
        }
      };
    return SiteBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static SiteBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SiteBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SiteBlockingStub>() {
        @java.lang.Override
        public SiteBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SiteBlockingStub(channel, callOptions);
        }
      };
    return SiteBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static SiteFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SiteFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SiteFutureStub>() {
        @java.lang.Override
        public SiteFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SiteFutureStub(channel, callOptions);
        }
      };
    return SiteFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void currencies(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.CurrenciesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCurrenciesMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service Site.
   */
  public static abstract class SiteImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return SiteGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Site.
   */
  public static final class SiteStub
      extends io.grpc.stub.AbstractAsyncStub<SiteStub> {
    private SiteStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SiteStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SiteStub(channel, callOptions);
    }

    /**
     */
    public void currencies(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.CurrenciesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCurrenciesMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Site.
   */
  public static final class SiteBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<SiteBlockingV2Stub> {
    private SiteBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SiteBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SiteBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.CurrenciesResponse currencies(com.google.protobuf.Empty request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCurrenciesMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Site.
   */
  public static final class SiteBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<SiteBlockingStub> {
    private SiteBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SiteBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SiteBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.CurrenciesResponse currencies(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCurrenciesMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Site.
   */
  public static final class SiteFutureStub
      extends io.grpc.stub.AbstractFutureStub<SiteFutureStub> {
    private SiteFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SiteFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SiteFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.CurrenciesResponse> currencies(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCurrenciesMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_CURRENCIES = 0;

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
        case METHODID_CURRENCIES:
          serviceImpl.currencies((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.CurrenciesResponse>) responseObserver);
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
          getCurrenciesMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.portal.v1.CurrenciesResponse>(
                service, METHODID_CURRENCIES)))
        .build();
  }

  private static abstract class SiteBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    SiteBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.portal.v1.PortalProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Site");
    }
  }

  private static final class SiteFileDescriptorSupplier
      extends SiteBaseDescriptorSupplier {
    SiteFileDescriptorSupplier() {}
  }

  private static final class SiteMethodDescriptorSupplier
      extends SiteBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    SiteMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (SiteGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new SiteFileDescriptorSupplier())
              .addMethod(getCurrenciesMethod())
              .build();
        }
      }
    }
    return result;
  }
}
