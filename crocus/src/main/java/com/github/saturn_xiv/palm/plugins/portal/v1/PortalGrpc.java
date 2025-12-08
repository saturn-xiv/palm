package com.github.saturn_xiv.palm.plugins.portal.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class PortalGrpc {

  private PortalGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.portal.v1.Portal";

  // Static method descriptors that strictly reflect the proto.
  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static PortalStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PortalStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PortalStub>() {
        @java.lang.Override
        public PortalStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PortalStub(channel, callOptions);
        }
      };
    return PortalStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static PortalBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PortalBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PortalBlockingV2Stub>() {
        @java.lang.Override
        public PortalBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PortalBlockingV2Stub(channel, callOptions);
        }
      };
    return PortalBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static PortalBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PortalBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PortalBlockingStub>() {
        @java.lang.Override
        public PortalBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PortalBlockingStub(channel, callOptions);
        }
      };
    return PortalBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static PortalFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PortalFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PortalFutureStub>() {
        @java.lang.Override
        public PortalFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PortalFutureStub(channel, callOptions);
        }
      };
    return PortalFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {
  }

  /**
   * Base class for the server implementation of the service Portal.
   */
  public static abstract class PortalImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return PortalGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Portal.
   */
  public static final class PortalStub
      extends io.grpc.stub.AbstractAsyncStub<PortalStub> {
    private PortalStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PortalStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PortalStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Portal.
   */
  public static final class PortalBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<PortalBlockingV2Stub> {
    private PortalBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PortalBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PortalBlockingV2Stub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Portal.
   */
  public static final class PortalBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<PortalBlockingStub> {
    private PortalBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PortalBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PortalBlockingStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Portal.
   */
  public static final class PortalFutureStub
      extends io.grpc.stub.AbstractFutureStub<PortalFutureStub> {
    private PortalFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PortalFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PortalFutureStub(channel, callOptions);
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

  private static abstract class PortalBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    PortalBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.portal.v1.PortalProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Portal");
    }
  }

  private static final class PortalFileDescriptorSupplier
      extends PortalBaseDescriptorSupplier {
    PortalFileDescriptorSupplier() {}
  }

  private static final class PortalMethodDescriptorSupplier
      extends PortalBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    PortalMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (PortalGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new PortalFileDescriptorSupplier())
              .build();
        }
      }
    }
    return result;
  }
}
