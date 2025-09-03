package com.github.saturn_xiv.palm.plugins.monitoring.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 * <pre>
 * ----------------------------------------------------------------------------
 * </pre>
 */
@javax.annotation.Generated(
    value = "by gRPC proto compiler (version 1.71.0)",
    comments = "Source: monitoring.proto")
@io.grpc.stub.annotations.GrpcGenerated
public final class SystemdGrpc {

  private SystemdGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.monitoring.v1.Systemd";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalRequest,
      com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalResponse> getJournalMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Journal",
      requestType = com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalRequest,
      com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalResponse> getJournalMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalRequest, com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalResponse> getJournalMethod;
    if ((getJournalMethod = SystemdGrpc.getJournalMethod) == null) {
      synchronized (SystemdGrpc.class) {
        if ((getJournalMethod = SystemdGrpc.getJournalMethod) == null) {
          SystemdGrpc.getJournalMethod = getJournalMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalRequest, com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Journal"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SystemdMethodDescriptorSupplier("Journal"))
              .build();
        }
      }
    }
    return getJournalMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static SystemdStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SystemdStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SystemdStub>() {
        @java.lang.Override
        public SystemdStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SystemdStub(channel, callOptions);
        }
      };
    return SystemdStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static SystemdBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SystemdBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SystemdBlockingV2Stub>() {
        @java.lang.Override
        public SystemdBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SystemdBlockingV2Stub(channel, callOptions);
        }
      };
    return SystemdBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static SystemdBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SystemdBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SystemdBlockingStub>() {
        @java.lang.Override
        public SystemdBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SystemdBlockingStub(channel, callOptions);
        }
      };
    return SystemdBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static SystemdFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SystemdFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SystemdFutureStub>() {
        @java.lang.Override
        public SystemdFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SystemdFutureStub(channel, callOptions);
        }
      };
    return SystemdFutureStub.newStub(factory, channel);
  }

  /**
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public interface AsyncService {

    /**
     */
    default void journal(com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getJournalMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service Systemd.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static abstract class SystemdImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return SystemdGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Systemd.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class SystemdStub
      extends io.grpc.stub.AbstractAsyncStub<SystemdStub> {
    private SystemdStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SystemdStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SystemdStub(channel, callOptions);
    }

    /**
     */
    public void journal(com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getJournalMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Systemd.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class SystemdBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<SystemdBlockingV2Stub> {
    private SystemdBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SystemdBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SystemdBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalResponse journal(com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getJournalMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Systemd.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class SystemdBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<SystemdBlockingStub> {
    private SystemdBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SystemdBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SystemdBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalResponse journal(com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getJournalMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Systemd.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class SystemdFutureStub
      extends io.grpc.stub.AbstractFutureStub<SystemdFutureStub> {
    private SystemdFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SystemdFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SystemdFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalResponse> journal(
        com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getJournalMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_JOURNAL = 0;

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
        case METHODID_JOURNAL:
          serviceImpl.journal((com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalResponse>) responseObserver);
          break;
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
        .addMethod(
          getJournalMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalRequest,
              com.github.saturn_xiv.palm.plugins.monitoring.v1.SystemdJournalResponse>(
                service, METHODID_JOURNAL)))
        .build();
  }

  private static abstract class SystemdBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    SystemdBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.monitoring.v1.Monitoring.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Systemd");
    }
  }

  private static final class SystemdFileDescriptorSupplier
      extends SystemdBaseDescriptorSupplier {
    SystemdFileDescriptorSupplier() {}
  }

  private static final class SystemdMethodDescriptorSupplier
      extends SystemdBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    SystemdMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (SystemdGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new SystemdFileDescriptorSupplier())
              .addMethod(getJournalMethod())
              .build();
        }
      }
    }
    return result;
  }
}
