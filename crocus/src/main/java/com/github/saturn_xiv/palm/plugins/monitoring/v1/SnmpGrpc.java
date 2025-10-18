package com.github.saturn_xiv.palm.plugins.monitoring.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 * <pre>
 * ----------------------------------------------------------------------------
 * </pre>
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class SnmpGrpc {

  private SnmpGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.monitoring.v1.Snmp";

  // Static method descriptors that strictly reflect the proto.
  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static SnmpStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SnmpStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SnmpStub>() {
        @java.lang.Override
        public SnmpStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SnmpStub(channel, callOptions);
        }
      };
    return SnmpStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static SnmpBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SnmpBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SnmpBlockingV2Stub>() {
        @java.lang.Override
        public SnmpBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SnmpBlockingV2Stub(channel, callOptions);
        }
      };
    return SnmpBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static SnmpBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SnmpBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SnmpBlockingStub>() {
        @java.lang.Override
        public SnmpBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SnmpBlockingStub(channel, callOptions);
        }
      };
    return SnmpBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static SnmpFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SnmpFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SnmpFutureStub>() {
        @java.lang.Override
        public SnmpFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SnmpFutureStub(channel, callOptions);
        }
      };
    return SnmpFutureStub.newStub(factory, channel);
  }

  /**
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public interface AsyncService {
  }

  /**
   * Base class for the server implementation of the service Snmp.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static abstract class SnmpImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return SnmpGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Snmp.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class SnmpStub
      extends io.grpc.stub.AbstractAsyncStub<SnmpStub> {
    private SnmpStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SnmpStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SnmpStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Snmp.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class SnmpBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<SnmpBlockingV2Stub> {
    private SnmpBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SnmpBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SnmpBlockingV2Stub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Snmp.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class SnmpBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<SnmpBlockingStub> {
    private SnmpBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SnmpBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SnmpBlockingStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Snmp.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class SnmpFutureStub
      extends io.grpc.stub.AbstractFutureStub<SnmpFutureStub> {
    private SnmpFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SnmpFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SnmpFutureStub(channel, callOptions);
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

  private static abstract class SnmpBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    SnmpBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.monitoring.v1.Monitoring.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Snmp");
    }
  }

  private static final class SnmpFileDescriptorSupplier
      extends SnmpBaseDescriptorSupplier {
    SnmpFileDescriptorSupplier() {}
  }

  private static final class SnmpMethodDescriptorSupplier
      extends SnmpBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    SnmpMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (SnmpGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new SnmpFileDescriptorSupplier())
              .build();
        }
      }
    }
    return result;
  }
}
