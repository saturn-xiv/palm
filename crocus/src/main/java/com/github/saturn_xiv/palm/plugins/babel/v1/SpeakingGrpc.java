package com.github.saturn_xiv.palm.plugins.babel.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class SpeakingGrpc {

  private SpeakingGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.babel.v1.Speaking";

  // Static method descriptors that strictly reflect the proto.
  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static SpeakingStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SpeakingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SpeakingStub>() {
        @java.lang.Override
        public SpeakingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SpeakingStub(channel, callOptions);
        }
      };
    return SpeakingStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static SpeakingBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SpeakingBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SpeakingBlockingV2Stub>() {
        @java.lang.Override
        public SpeakingBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SpeakingBlockingV2Stub(channel, callOptions);
        }
      };
    return SpeakingBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static SpeakingBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SpeakingBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SpeakingBlockingStub>() {
        @java.lang.Override
        public SpeakingBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SpeakingBlockingStub(channel, callOptions);
        }
      };
    return SpeakingBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static SpeakingFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SpeakingFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SpeakingFutureStub>() {
        @java.lang.Override
        public SpeakingFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SpeakingFutureStub(channel, callOptions);
        }
      };
    return SpeakingFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {
  }

  /**
   * Base class for the server implementation of the service Speaking.
   */
  public static abstract class SpeakingImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return SpeakingGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Speaking.
   */
  public static final class SpeakingStub
      extends io.grpc.stub.AbstractAsyncStub<SpeakingStub> {
    private SpeakingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SpeakingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SpeakingStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Speaking.
   */
  public static final class SpeakingBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<SpeakingBlockingV2Stub> {
    private SpeakingBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SpeakingBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SpeakingBlockingV2Stub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Speaking.
   */
  public static final class SpeakingBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<SpeakingBlockingStub> {
    private SpeakingBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SpeakingBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SpeakingBlockingStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Speaking.
   */
  public static final class SpeakingFutureStub
      extends io.grpc.stub.AbstractFutureStub<SpeakingFutureStub> {
    private SpeakingFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SpeakingFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SpeakingFutureStub(channel, callOptions);
    }
  }


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
        .build();
  }

  private static abstract class SpeakingBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    SpeakingBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.babel.v1.BabelProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Speaking");
    }
  }

  private static final class SpeakingFileDescriptorSupplier
      extends SpeakingBaseDescriptorSupplier {
    SpeakingFileDescriptorSupplier() {}
  }

  private static final class SpeakingMethodDescriptorSupplier
      extends SpeakingBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    SpeakingMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (SpeakingGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new SpeakingFileDescriptorSupplier())
              .build();
        }
      }
    }
    return result;
  }
}
