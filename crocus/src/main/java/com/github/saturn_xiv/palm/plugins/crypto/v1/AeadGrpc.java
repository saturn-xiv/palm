package com.github.saturn_xiv.palm.plugins.crypto.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class AeadGrpc {

  private AeadGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.crypto.v1.Aead";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptRequest,
      com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptResponse> getEncryptMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Encrypt",
      requestType = com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptRequest,
      com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptResponse> getEncryptMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptRequest, com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptResponse> getEncryptMethod;
    if ((getEncryptMethod = AeadGrpc.getEncryptMethod) == null) {
      synchronized (AeadGrpc.class) {
        if ((getEncryptMethod = AeadGrpc.getEncryptMethod) == null) {
          AeadGrpc.getEncryptMethod = getEncryptMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptRequest, com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Encrypt"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptResponse.getDefaultInstance()))
              .setSchemaDescriptor(new AeadMethodDescriptorSupplier("Encrypt"))
              .build();
        }
      }
    }
    return getEncryptMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptRequest,
      com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptResponse> getDecryptMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Decrypt",
      requestType = com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptRequest,
      com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptResponse> getDecryptMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptRequest, com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptResponse> getDecryptMethod;
    if ((getDecryptMethod = AeadGrpc.getDecryptMethod) == null) {
      synchronized (AeadGrpc.class) {
        if ((getDecryptMethod = AeadGrpc.getDecryptMethod) == null) {
          AeadGrpc.getDecryptMethod = getDecryptMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptRequest, com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Decrypt"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptResponse.getDefaultInstance()))
              .setSchemaDescriptor(new AeadMethodDescriptorSupplier("Decrypt"))
              .build();
        }
      }
    }
    return getDecryptMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static AeadStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<AeadStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<AeadStub>() {
        @java.lang.Override
        public AeadStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new AeadStub(channel, callOptions);
        }
      };
    return AeadStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static AeadBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<AeadBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<AeadBlockingV2Stub>() {
        @java.lang.Override
        public AeadBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new AeadBlockingV2Stub(channel, callOptions);
        }
      };
    return AeadBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static AeadBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<AeadBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<AeadBlockingStub>() {
        @java.lang.Override
        public AeadBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new AeadBlockingStub(channel, callOptions);
        }
      };
    return AeadBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static AeadFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<AeadFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<AeadFutureStub>() {
        @java.lang.Override
        public AeadFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new AeadFutureStub(channel, callOptions);
        }
      };
    return AeadFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void encrypt(com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getEncryptMethod(), responseObserver);
    }

    /**
     */
    default void decrypt(com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDecryptMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service Aead.
   */
  public static abstract class AeadImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return AeadGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Aead.
   */
  public static final class AeadStub
      extends io.grpc.stub.AbstractAsyncStub<AeadStub> {
    private AeadStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected AeadStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new AeadStub(channel, callOptions);
    }

    /**
     */
    public void encrypt(com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getEncryptMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void decrypt(com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDecryptMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Aead.
   */
  public static final class AeadBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<AeadBlockingV2Stub> {
    private AeadBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected AeadBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new AeadBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptResponse encrypt(com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getEncryptMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptResponse decrypt(com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getDecryptMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Aead.
   */
  public static final class AeadBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<AeadBlockingStub> {
    private AeadBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected AeadBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new AeadBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptResponse encrypt(com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getEncryptMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptResponse decrypt(com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDecryptMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Aead.
   */
  public static final class AeadFutureStub
      extends io.grpc.stub.AbstractFutureStub<AeadFutureStub> {
    private AeadFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected AeadFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new AeadFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptResponse> encrypt(
        com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getEncryptMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptResponse> decrypt(
        com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDecryptMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_ENCRYPT = 0;
  private static final int METHODID_DECRYPT = 1;

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
        case METHODID_ENCRYPT:
          serviceImpl.encrypt((com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptResponse>) responseObserver);
          break;
        case METHODID_DECRYPT:
          serviceImpl.decrypt((com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptResponse>) responseObserver);
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
          getEncryptMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptRequest,
              com.github.saturn_xiv.palm.plugins.crypto.v1.AeadEncryptResponse>(
                service, METHODID_ENCRYPT)))
        .addMethod(
          getDecryptMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptRequest,
              com.github.saturn_xiv.palm.plugins.crypto.v1.AeadDecryptResponse>(
                service, METHODID_DECRYPT)))
        .build();
  }

  private static abstract class AeadBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    AeadBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.crypto.v1.CryptoProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Aead");
    }
  }

  private static final class AeadFileDescriptorSupplier
      extends AeadBaseDescriptorSupplier {
    AeadFileDescriptorSupplier() {}
  }

  private static final class AeadMethodDescriptorSupplier
      extends AeadBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    AeadMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (AeadGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new AeadFileDescriptorSupplier())
              .addMethod(getEncryptMethod())
              .addMethod(getDecryptMethod())
              .build();
        }
      }
    }
    return result;
  }
}
