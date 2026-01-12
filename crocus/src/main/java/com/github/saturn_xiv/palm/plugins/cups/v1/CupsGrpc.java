package com.github.saturn_xiv.palm.plugins.cups.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class CupsGrpc {

  private CupsGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.cups.v1.Cups";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.cups.v1.CupsPrintersResponse> getPrintersMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Printers",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.cups.v1.CupsPrintersResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.cups.v1.CupsPrintersResponse> getPrintersMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.cups.v1.CupsPrintersResponse> getPrintersMethod;
    if ((getPrintersMethod = CupsGrpc.getPrintersMethod) == null) {
      synchronized (CupsGrpc.class) {
        if ((getPrintersMethod = CupsGrpc.getPrintersMethod) == null) {
          CupsGrpc.getPrintersMethod = getPrintersMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.cups.v1.CupsPrintersResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Printers"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.cups.v1.CupsPrintersResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CupsMethodDescriptorSupplier("Printers"))
              .build();
        }
      }
    }
    return getPrintersMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static CupsStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CupsStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CupsStub>() {
        @java.lang.Override
        public CupsStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CupsStub(channel, callOptions);
        }
      };
    return CupsStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static CupsBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CupsBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CupsBlockingV2Stub>() {
        @java.lang.Override
        public CupsBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CupsBlockingV2Stub(channel, callOptions);
        }
      };
    return CupsBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static CupsBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CupsBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CupsBlockingStub>() {
        @java.lang.Override
        public CupsBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CupsBlockingStub(channel, callOptions);
        }
      };
    return CupsBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static CupsFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CupsFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CupsFutureStub>() {
        @java.lang.Override
        public CupsFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CupsFutureStub(channel, callOptions);
        }
      };
    return CupsFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void printers(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.cups.v1.CupsPrintersResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getPrintersMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service Cups.
   */
  public static abstract class CupsImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return CupsGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Cups.
   */
  public static final class CupsStub
      extends io.grpc.stub.AbstractAsyncStub<CupsStub> {
    private CupsStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CupsStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CupsStub(channel, callOptions);
    }

    /**
     */
    public void printers(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.cups.v1.CupsPrintersResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getPrintersMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Cups.
   */
  public static final class CupsBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<CupsBlockingV2Stub> {
    private CupsBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CupsBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CupsBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.cups.v1.CupsPrintersResponse printers(com.google.protobuf.Empty request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getPrintersMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Cups.
   */
  public static final class CupsBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<CupsBlockingStub> {
    private CupsBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CupsBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CupsBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.cups.v1.CupsPrintersResponse printers(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getPrintersMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Cups.
   */
  public static final class CupsFutureStub
      extends io.grpc.stub.AbstractFutureStub<CupsFutureStub> {
    private CupsFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CupsFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CupsFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.cups.v1.CupsPrintersResponse> printers(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getPrintersMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_PRINTERS = 0;

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
        case METHODID_PRINTERS:
          serviceImpl.printers((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.cups.v1.CupsPrintersResponse>) responseObserver);
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
          getPrintersMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.cups.v1.CupsPrintersResponse>(
                service, METHODID_PRINTERS)))
        .build();
  }

  private static abstract class CupsBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    CupsBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.cups.v1.CupsProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Cups");
    }
  }

  private static final class CupsFileDescriptorSupplier
      extends CupsBaseDescriptorSupplier {
    CupsFileDescriptorSupplier() {}
  }

  private static final class CupsMethodDescriptorSupplier
      extends CupsBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    CupsMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (CupsGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new CupsFileDescriptorSupplier())
              .addMethod(getPrintersMethod())
              .build();
        }
      }
    }
    return result;
  }
}
