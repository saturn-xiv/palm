package com.github.saturn_xiv.palm.plugins.crypto.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 * <pre>
 * ----------------------------------------------------------------------------
 * </pre>
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class HMacGrpc {

  private HMacGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.crypto.v1.HMac";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeRequest,
      com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeResponse> getComputeMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Compute",
      requestType = com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeRequest,
      com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeResponse> getComputeMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeRequest, com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeResponse> getComputeMethod;
    if ((getComputeMethod = HMacGrpc.getComputeMethod) == null) {
      synchronized (HMacGrpc.class) {
        if ((getComputeMethod = HMacGrpc.getComputeMethod) == null) {
          HMacGrpc.getComputeMethod = getComputeMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeRequest, com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Compute"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeResponse.getDefaultInstance()))
              .setSchemaDescriptor(new HMacMethodDescriptorSupplier("Compute"))
              .build();
        }
      }
    }
    return getComputeMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.crypto.v1.HMacVerifyRequest,
      com.google.protobuf.Empty> getVerifyMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Verify",
      requestType = com.github.saturn_xiv.palm.plugins.crypto.v1.HMacVerifyRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.crypto.v1.HMacVerifyRequest,
      com.google.protobuf.Empty> getVerifyMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.crypto.v1.HMacVerifyRequest, com.google.protobuf.Empty> getVerifyMethod;
    if ((getVerifyMethod = HMacGrpc.getVerifyMethod) == null) {
      synchronized (HMacGrpc.class) {
        if ((getVerifyMethod = HMacGrpc.getVerifyMethod) == null) {
          HMacGrpc.getVerifyMethod = getVerifyMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.crypto.v1.HMacVerifyRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Verify"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.crypto.v1.HMacVerifyRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new HMacMethodDescriptorSupplier("Verify"))
              .build();
        }
      }
    }
    return getVerifyMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static HMacStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<HMacStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<HMacStub>() {
        @java.lang.Override
        public HMacStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new HMacStub(channel, callOptions);
        }
      };
    return HMacStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static HMacBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<HMacBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<HMacBlockingV2Stub>() {
        @java.lang.Override
        public HMacBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new HMacBlockingV2Stub(channel, callOptions);
        }
      };
    return HMacBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static HMacBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<HMacBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<HMacBlockingStub>() {
        @java.lang.Override
        public HMacBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new HMacBlockingStub(channel, callOptions);
        }
      };
    return HMacBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static HMacFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<HMacFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<HMacFutureStub>() {
        @java.lang.Override
        public HMacFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new HMacFutureStub(channel, callOptions);
        }
      };
    return HMacFutureStub.newStub(factory, channel);
  }

  /**
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public interface AsyncService {

    /**
     */
    default void compute(com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getComputeMethod(), responseObserver);
    }

    /**
     */
    default void verify(com.github.saturn_xiv.palm.plugins.crypto.v1.HMacVerifyRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getVerifyMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service HMac.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static abstract class HMacImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return HMacGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service HMac.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class HMacStub
      extends io.grpc.stub.AbstractAsyncStub<HMacStub> {
    private HMacStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected HMacStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new HMacStub(channel, callOptions);
    }

    /**
     */
    public void compute(com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getComputeMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void verify(com.github.saturn_xiv.palm.plugins.crypto.v1.HMacVerifyRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getVerifyMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service HMac.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class HMacBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<HMacBlockingV2Stub> {
    private HMacBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected HMacBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new HMacBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeResponse compute(com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getComputeMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty verify(com.github.saturn_xiv.palm.plugins.crypto.v1.HMacVerifyRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getVerifyMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service HMac.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class HMacBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<HMacBlockingStub> {
    private HMacBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected HMacBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new HMacBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeResponse compute(com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getComputeMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty verify(com.github.saturn_xiv.palm.plugins.crypto.v1.HMacVerifyRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getVerifyMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service HMac.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class HMacFutureStub
      extends io.grpc.stub.AbstractFutureStub<HMacFutureStub> {
    private HMacFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected HMacFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new HMacFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeResponse> compute(
        com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getComputeMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> verify(
        com.github.saturn_xiv.palm.plugins.crypto.v1.HMacVerifyRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getVerifyMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_COMPUTE = 0;
  private static final int METHODID_VERIFY = 1;

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
        case METHODID_COMPUTE:
          serviceImpl.compute((com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeResponse>) responseObserver);
          break;
        case METHODID_VERIFY:
          serviceImpl.verify((com.github.saturn_xiv.palm.plugins.crypto.v1.HMacVerifyRequest) request,
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
          getComputeMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeRequest,
              com.github.saturn_xiv.palm.plugins.crypto.v1.HMacComputeResponse>(
                service, METHODID_COMPUTE)))
        .addMethod(
          getVerifyMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.crypto.v1.HMacVerifyRequest,
              com.google.protobuf.Empty>(
                service, METHODID_VERIFY)))
        .build();
  }

  private static abstract class HMacBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    HMacBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.crypto.v1.CryptoProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("HMac");
    }
  }

  private static final class HMacFileDescriptorSupplier
      extends HMacBaseDescriptorSupplier {
    HMacFileDescriptorSupplier() {}
  }

  private static final class HMacMethodDescriptorSupplier
      extends HMacBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    HMacMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (HMacGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new HMacFileDescriptorSupplier())
              .addMethod(getComputeMethod())
              .addMethod(getVerifyMethod())
              .build();
        }
      }
    }
    return result;
  }
}
