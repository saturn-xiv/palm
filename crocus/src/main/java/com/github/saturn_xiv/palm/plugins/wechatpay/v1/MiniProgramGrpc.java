package com.github.saturn_xiv.palm.plugins.wechatpay.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 * <pre>
 * ----------------------------------------------------------------------------
 * </pre>
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class MiniProgramGrpc {

  private MiniProgramGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.wechatpay.v1.MiniProgram";

  // Static method descriptors that strictly reflect the proto.
  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static MiniProgramStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<MiniProgramStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<MiniProgramStub>() {
        @java.lang.Override
        public MiniProgramStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new MiniProgramStub(channel, callOptions);
        }
      };
    return MiniProgramStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static MiniProgramBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<MiniProgramBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<MiniProgramBlockingV2Stub>() {
        @java.lang.Override
        public MiniProgramBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new MiniProgramBlockingV2Stub(channel, callOptions);
        }
      };
    return MiniProgramBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static MiniProgramBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<MiniProgramBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<MiniProgramBlockingStub>() {
        @java.lang.Override
        public MiniProgramBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new MiniProgramBlockingStub(channel, callOptions);
        }
      };
    return MiniProgramBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static MiniProgramFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<MiniProgramFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<MiniProgramFutureStub>() {
        @java.lang.Override
        public MiniProgramFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new MiniProgramFutureStub(channel, callOptions);
        }
      };
    return MiniProgramFutureStub.newStub(factory, channel);
  }

  /**
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public interface AsyncService {
  }

  /**
   * Base class for the server implementation of the service MiniProgram.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static abstract class MiniProgramImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return MiniProgramGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service MiniProgram.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class MiniProgramStub
      extends io.grpc.stub.AbstractAsyncStub<MiniProgramStub> {
    private MiniProgramStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected MiniProgramStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new MiniProgramStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service MiniProgram.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class MiniProgramBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<MiniProgramBlockingV2Stub> {
    private MiniProgramBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected MiniProgramBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new MiniProgramBlockingV2Stub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service MiniProgram.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class MiniProgramBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<MiniProgramBlockingStub> {
    private MiniProgramBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected MiniProgramBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new MiniProgramBlockingStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service MiniProgram.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class MiniProgramFutureStub
      extends io.grpc.stub.AbstractFutureStub<MiniProgramFutureStub> {
    private MiniProgramFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected MiniProgramFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new MiniProgramFutureStub(channel, callOptions);
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

  private static abstract class MiniProgramBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    MiniProgramBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.wechatpay.v1.WechatPayProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("MiniProgram");
    }
  }

  private static final class MiniProgramFileDescriptorSupplier
      extends MiniProgramBaseDescriptorSupplier {
    MiniProgramFileDescriptorSupplier() {}
  }

  private static final class MiniProgramMethodDescriptorSupplier
      extends MiniProgramBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    MiniProgramMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (MiniProgramGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new MiniProgramFileDescriptorSupplier())
              .build();
        }
      }
    }
    return result;
  }
}
