package com.github.saturn_xiv.palm.plugins.babel.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class VocabularyGrpc {

  private VocabularyGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.babel.v1.Vocabulary";

  // Static method descriptors that strictly reflect the proto.
  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static VocabularyStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<VocabularyStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<VocabularyStub>() {
        @java.lang.Override
        public VocabularyStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new VocabularyStub(channel, callOptions);
        }
      };
    return VocabularyStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static VocabularyBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<VocabularyBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<VocabularyBlockingV2Stub>() {
        @java.lang.Override
        public VocabularyBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new VocabularyBlockingV2Stub(channel, callOptions);
        }
      };
    return VocabularyBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static VocabularyBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<VocabularyBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<VocabularyBlockingStub>() {
        @java.lang.Override
        public VocabularyBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new VocabularyBlockingStub(channel, callOptions);
        }
      };
    return VocabularyBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static VocabularyFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<VocabularyFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<VocabularyFutureStub>() {
        @java.lang.Override
        public VocabularyFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new VocabularyFutureStub(channel, callOptions);
        }
      };
    return VocabularyFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {
  }

  /**
   * Base class for the server implementation of the service Vocabulary.
   */
  public static abstract class VocabularyImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return VocabularyGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Vocabulary.
   */
  public static final class VocabularyStub
      extends io.grpc.stub.AbstractAsyncStub<VocabularyStub> {
    private VocabularyStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected VocabularyStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new VocabularyStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Vocabulary.
   */
  public static final class VocabularyBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<VocabularyBlockingV2Stub> {
    private VocabularyBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected VocabularyBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new VocabularyBlockingV2Stub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Vocabulary.
   */
  public static final class VocabularyBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<VocabularyBlockingStub> {
    private VocabularyBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected VocabularyBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new VocabularyBlockingStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Vocabulary.
   */
  public static final class VocabularyFutureStub
      extends io.grpc.stub.AbstractFutureStub<VocabularyFutureStub> {
    private VocabularyFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected VocabularyFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new VocabularyFutureStub(channel, callOptions);
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

  private static abstract class VocabularyBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    VocabularyBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.babel.v1.BabelProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Vocabulary");
    }
  }

  private static final class VocabularyFileDescriptorSupplier
      extends VocabularyBaseDescriptorSupplier {
    VocabularyFileDescriptorSupplier() {}
  }

  private static final class VocabularyMethodDescriptorSupplier
      extends VocabularyBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    VocabularyMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (VocabularyGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new VocabularyFileDescriptorSupplier())
              .build();
        }
      }
    }
    return result;
  }
}
