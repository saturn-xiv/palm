package com.github.saturn_xiv.palm.plugins.wechatpay.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 * <pre>
 * ----------------------------------------------------------------------------
 * </pre>
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class PayGrpc {

  private PayGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.wechatpay.v1.Pay";

  // Static method descriptors that strictly reflect the proto.
  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static PayStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PayStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PayStub>() {
        @java.lang.Override
        public PayStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PayStub(channel, callOptions);
        }
      };
    return PayStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static PayBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PayBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PayBlockingV2Stub>() {
        @java.lang.Override
        public PayBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PayBlockingV2Stub(channel, callOptions);
        }
      };
    return PayBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static PayBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PayBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PayBlockingStub>() {
        @java.lang.Override
        public PayBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PayBlockingStub(channel, callOptions);
        }
      };
    return PayBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static PayFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PayFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PayFutureStub>() {
        @java.lang.Override
        public PayFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PayFutureStub(channel, callOptions);
        }
      };
    return PayFutureStub.newStub(factory, channel);
  }

  /**
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public interface AsyncService {
  }

  /**
   * Base class for the server implementation of the service Pay.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static abstract class PayImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return PayGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Pay.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class PayStub
      extends io.grpc.stub.AbstractAsyncStub<PayStub> {
    private PayStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PayStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PayStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Pay.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class PayBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<PayBlockingV2Stub> {
    private PayBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PayBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PayBlockingV2Stub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Pay.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class PayBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<PayBlockingStub> {
    private PayBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PayBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PayBlockingStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Pay.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class PayFutureStub
      extends io.grpc.stub.AbstractFutureStub<PayFutureStub> {
    private PayFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PayFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PayFutureStub(channel, callOptions);
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

  private static abstract class PayBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    PayBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.wechatpay.v1.WechatPayProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Pay");
    }
  }

  private static final class PayFileDescriptorSupplier
      extends PayBaseDescriptorSupplier {
    PayFileDescriptorSupplier() {}
  }

  private static final class PayMethodDescriptorSupplier
      extends PayBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    PayMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (PayGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new PayFileDescriptorSupplier())
              .build();
        }
      }
    }
    return result;
  }
}
