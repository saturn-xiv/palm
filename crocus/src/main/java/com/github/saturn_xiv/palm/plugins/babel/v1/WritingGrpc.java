package com.github.saturn_xiv.palm.plugins.babel.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class WritingGrpc {

  private WritingGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.babel.v1.Writing";

  // Static method descriptors that strictly reflect the proto.
  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static WritingStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<WritingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<WritingStub>() {
        @java.lang.Override
        public WritingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new WritingStub(channel, callOptions);
        }
      };
    return WritingStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static WritingBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<WritingBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<WritingBlockingV2Stub>() {
        @java.lang.Override
        public WritingBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new WritingBlockingV2Stub(channel, callOptions);
        }
      };
    return WritingBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static WritingBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<WritingBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<WritingBlockingStub>() {
        @java.lang.Override
        public WritingBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new WritingBlockingStub(channel, callOptions);
        }
      };
    return WritingBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static WritingFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<WritingFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<WritingFutureStub>() {
        @java.lang.Override
        public WritingFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new WritingFutureStub(channel, callOptions);
        }
      };
    return WritingFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {
  }

  /**
   * Base class for the server implementation of the service Writing.
   */
  public static abstract class WritingImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return WritingGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Writing.
   */
  public static final class WritingStub
      extends io.grpc.stub.AbstractAsyncStub<WritingStub> {
    private WritingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected WritingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new WritingStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Writing.
   */
  public static final class WritingBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<WritingBlockingV2Stub> {
    private WritingBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected WritingBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new WritingBlockingV2Stub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Writing.
   */
  public static final class WritingBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<WritingBlockingStub> {
    private WritingBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected WritingBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new WritingBlockingStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Writing.
   */
  public static final class WritingFutureStub
      extends io.grpc.stub.AbstractFutureStub<WritingFutureStub> {
    private WritingFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected WritingFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new WritingFutureStub(channel, callOptions);
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

  private static abstract class WritingBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    WritingBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.babel.v1.BabelProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Writing");
    }
  }

  private static final class WritingFileDescriptorSupplier
      extends WritingBaseDescriptorSupplier {
    WritingFileDescriptorSupplier() {}
  }

  private static final class WritingMethodDescriptorSupplier
      extends WritingBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    WritingMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (WritingGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new WritingFileDescriptorSupplier())
              .build();
        }
      }
    }
    return result;
  }
}
