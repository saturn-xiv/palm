package com.github.saturn_xiv.palm.plugins.babel.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class FlashcardGrpc {

  private FlashcardGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.babel.v1.Flashcard";

  // Static method descriptors that strictly reflect the proto.
  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static FlashcardStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<FlashcardStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<FlashcardStub>() {
        @java.lang.Override
        public FlashcardStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new FlashcardStub(channel, callOptions);
        }
      };
    return FlashcardStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static FlashcardBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<FlashcardBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<FlashcardBlockingV2Stub>() {
        @java.lang.Override
        public FlashcardBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new FlashcardBlockingV2Stub(channel, callOptions);
        }
      };
    return FlashcardBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static FlashcardBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<FlashcardBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<FlashcardBlockingStub>() {
        @java.lang.Override
        public FlashcardBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new FlashcardBlockingStub(channel, callOptions);
        }
      };
    return FlashcardBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static FlashcardFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<FlashcardFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<FlashcardFutureStub>() {
        @java.lang.Override
        public FlashcardFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new FlashcardFutureStub(channel, callOptions);
        }
      };
    return FlashcardFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {
  }

  /**
   * Base class for the server implementation of the service Flashcard.
   */
  public static abstract class FlashcardImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return FlashcardGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Flashcard.
   */
  public static final class FlashcardStub
      extends io.grpc.stub.AbstractAsyncStub<FlashcardStub> {
    private FlashcardStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected FlashcardStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new FlashcardStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Flashcard.
   */
  public static final class FlashcardBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<FlashcardBlockingV2Stub> {
    private FlashcardBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected FlashcardBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new FlashcardBlockingV2Stub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Flashcard.
   */
  public static final class FlashcardBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<FlashcardBlockingStub> {
    private FlashcardBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected FlashcardBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new FlashcardBlockingStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Flashcard.
   */
  public static final class FlashcardFutureStub
      extends io.grpc.stub.AbstractFutureStub<FlashcardFutureStub> {
    private FlashcardFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected FlashcardFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new FlashcardFutureStub(channel, callOptions);
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

  private static abstract class FlashcardBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    FlashcardBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.babel.v1.BabelProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Flashcard");
    }
  }

  private static final class FlashcardFileDescriptorSupplier
      extends FlashcardBaseDescriptorSupplier {
    FlashcardFileDescriptorSupplier() {}
  }

  private static final class FlashcardMethodDescriptorSupplier
      extends FlashcardBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    FlashcardMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (FlashcardGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new FlashcardFileDescriptorSupplier())
              .build();
        }
      }
    }
    return result;
  }
}
