package com.github.saturn_xiv.palm.plugins.forum.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class ForumGrpc {

  private ForumGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.forum.v1.Forum";

  // Static method descriptors that strictly reflect the proto.
  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static ForumStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<ForumStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<ForumStub>() {
        @java.lang.Override
        public ForumStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new ForumStub(channel, callOptions);
        }
      };
    return ForumStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static ForumBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<ForumBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<ForumBlockingV2Stub>() {
        @java.lang.Override
        public ForumBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new ForumBlockingV2Stub(channel, callOptions);
        }
      };
    return ForumBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static ForumBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<ForumBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<ForumBlockingStub>() {
        @java.lang.Override
        public ForumBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new ForumBlockingStub(channel, callOptions);
        }
      };
    return ForumBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static ForumFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<ForumFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<ForumFutureStub>() {
        @java.lang.Override
        public ForumFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new ForumFutureStub(channel, callOptions);
        }
      };
    return ForumFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {
  }

  /**
   * Base class for the server implementation of the service Forum.
   */
  public static abstract class ForumImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return ForumGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Forum.
   */
  public static final class ForumStub
      extends io.grpc.stub.AbstractAsyncStub<ForumStub> {
    private ForumStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected ForumStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new ForumStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Forum.
   */
  public static final class ForumBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<ForumBlockingV2Stub> {
    private ForumBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected ForumBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new ForumBlockingV2Stub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Forum.
   */
  public static final class ForumBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<ForumBlockingStub> {
    private ForumBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected ForumBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new ForumBlockingStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Forum.
   */
  public static final class ForumFutureStub
      extends io.grpc.stub.AbstractFutureStub<ForumFutureStub> {
    private ForumFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected ForumFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new ForumFutureStub(channel, callOptions);
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

  private static abstract class ForumBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    ForumBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.forum.v1.ForumProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Forum");
    }
  }

  private static final class ForumFileDescriptorSupplier
      extends ForumBaseDescriptorSupplier {
    ForumFileDescriptorSupplier() {}
  }

  private static final class ForumMethodDescriptorSupplier
      extends ForumBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    ForumMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (ForumGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new ForumFileDescriptorSupplier())
              .build();
        }
      }
    }
    return result;
  }
}
