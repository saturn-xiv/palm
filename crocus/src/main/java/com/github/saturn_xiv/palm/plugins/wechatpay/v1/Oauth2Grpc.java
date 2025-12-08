package com.github.saturn_xiv.palm.plugins.wechatpay.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 * <pre>
 * ----------------------------------------------------------------------------
 * </pre>
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class Oauth2Grpc {

  private Oauth2Grpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.wechatpay.v1.Oauth2";

  // Static method descriptors that strictly reflect the proto.
  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static Oauth2Stub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<Oauth2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<Oauth2Stub>() {
        @java.lang.Override
        public Oauth2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new Oauth2Stub(channel, callOptions);
        }
      };
    return Oauth2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static Oauth2BlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<Oauth2BlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<Oauth2BlockingV2Stub>() {
        @java.lang.Override
        public Oauth2BlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new Oauth2BlockingV2Stub(channel, callOptions);
        }
      };
    return Oauth2BlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static Oauth2BlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<Oauth2BlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<Oauth2BlockingStub>() {
        @java.lang.Override
        public Oauth2BlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new Oauth2BlockingStub(channel, callOptions);
        }
      };
    return Oauth2BlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static Oauth2FutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<Oauth2FutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<Oauth2FutureStub>() {
        @java.lang.Override
        public Oauth2FutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new Oauth2FutureStub(channel, callOptions);
        }
      };
    return Oauth2FutureStub.newStub(factory, channel);
  }

  /**
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public interface AsyncService {
  }

  /**
   * Base class for the server implementation of the service Oauth2.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static abstract class Oauth2ImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return Oauth2Grpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Oauth2.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class Oauth2Stub
      extends io.grpc.stub.AbstractAsyncStub<Oauth2Stub> {
    private Oauth2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected Oauth2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new Oauth2Stub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Oauth2.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class Oauth2BlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<Oauth2BlockingV2Stub> {
    private Oauth2BlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected Oauth2BlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new Oauth2BlockingV2Stub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Oauth2.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class Oauth2BlockingStub
      extends io.grpc.stub.AbstractBlockingStub<Oauth2BlockingStub> {
    private Oauth2BlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected Oauth2BlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new Oauth2BlockingStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Oauth2.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class Oauth2FutureStub
      extends io.grpc.stub.AbstractFutureStub<Oauth2FutureStub> {
    private Oauth2FutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected Oauth2FutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new Oauth2FutureStub(channel, callOptions);
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

  private static abstract class Oauth2BaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    Oauth2BaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.wechatpay.v1.WechatPayProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Oauth2");
    }
  }

  private static final class Oauth2FileDescriptorSupplier
      extends Oauth2BaseDescriptorSupplier {
    Oauth2FileDescriptorSupplier() {}
  }

  private static final class Oauth2MethodDescriptorSupplier
      extends Oauth2BaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    Oauth2MethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (Oauth2Grpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new Oauth2FileDescriptorSupplier())
              .build();
        }
      }
    }
    return result;
  }
}
