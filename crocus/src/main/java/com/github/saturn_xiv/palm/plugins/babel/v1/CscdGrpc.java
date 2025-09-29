package com.github.saturn_xiv.palm.plugins.babel.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class CscdGrpc {

  private CscdGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.babel.v1.Cscd";

  // Static method descriptors that strictly reflect the proto.
  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static CscdStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CscdStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CscdStub>() {
        @java.lang.Override
        public CscdStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CscdStub(channel, callOptions);
        }
      };
    return CscdStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static CscdBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CscdBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CscdBlockingV2Stub>() {
        @java.lang.Override
        public CscdBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CscdBlockingV2Stub(channel, callOptions);
        }
      };
    return CscdBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static CscdBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CscdBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CscdBlockingStub>() {
        @java.lang.Override
        public CscdBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CscdBlockingStub(channel, callOptions);
        }
      };
    return CscdBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static CscdFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CscdFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CscdFutureStub>() {
        @java.lang.Override
        public CscdFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CscdFutureStub(channel, callOptions);
        }
      };
    return CscdFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {
  }

  /**
   * Base class for the server implementation of the service Cscd.
   */
  public static abstract class CscdImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return CscdGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Cscd.
   */
  public static final class CscdStub
      extends io.grpc.stub.AbstractAsyncStub<CscdStub> {
    private CscdStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CscdStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CscdStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Cscd.
   */
  public static final class CscdBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<CscdBlockingV2Stub> {
    private CscdBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CscdBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CscdBlockingV2Stub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Cscd.
   */
  public static final class CscdBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<CscdBlockingStub> {
    private CscdBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CscdBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CscdBlockingStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Cscd.
   */
  public static final class CscdFutureStub
      extends io.grpc.stub.AbstractFutureStub<CscdFutureStub> {
    private CscdFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CscdFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CscdFutureStub(channel, callOptions);
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

  private static abstract class CscdBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    CscdBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.babel.v1.Babel.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Cscd");
    }
  }

  private static final class CscdFileDescriptorSupplier
      extends CscdBaseDescriptorSupplier {
    CscdFileDescriptorSupplier() {}
  }

  private static final class CscdMethodDescriptorSupplier
      extends CscdBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    CscdMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (CscdGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new CscdFileDescriptorSupplier())
              .build();
        }
      }
    }
    return result;
  }
}
