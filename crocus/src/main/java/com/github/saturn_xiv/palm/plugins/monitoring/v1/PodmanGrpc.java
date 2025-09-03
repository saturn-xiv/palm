package com.github.saturn_xiv.palm.plugins.monitoring.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@javax.annotation.Generated(
    value = "by gRPC proto compiler (version 1.71.0)",
    comments = "Source: monitoring.proto")
@io.grpc.stub.annotations.GrpcGenerated
public final class PodmanGrpc {

  private PodmanGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.monitoring.v1.Podman";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest,
      com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanLogsResponse> getLogsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Logs",
      requestType = com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanLogsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest,
      com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanLogsResponse> getLogsMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest, com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanLogsResponse> getLogsMethod;
    if ((getLogsMethod = PodmanGrpc.getLogsMethod) == null) {
      synchronized (PodmanGrpc.class) {
        if ((getLogsMethod = PodmanGrpc.getLogsMethod) == null) {
          PodmanGrpc.getLogsMethod = getLogsMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest, com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanLogsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Logs"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanLogsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PodmanMethodDescriptorSupplier("Logs"))
              .build();
        }
      }
    }
    return getLogsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest,
      com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanContainersResponse> getContainersMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Containers",
      requestType = com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanContainersResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest,
      com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanContainersResponse> getContainersMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest, com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanContainersResponse> getContainersMethod;
    if ((getContainersMethod = PodmanGrpc.getContainersMethod) == null) {
      synchronized (PodmanGrpc.class) {
        if ((getContainersMethod = PodmanGrpc.getContainersMethod) == null) {
          PodmanGrpc.getContainersMethod = getContainersMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest, com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanContainersResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Containers"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanContainersResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PodmanMethodDescriptorSupplier("Containers"))
              .build();
        }
      }
    }
    return getContainersMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest,
      com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanStatisticsResponse> getStatisticsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Statistics",
      requestType = com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanStatisticsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest,
      com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanStatisticsResponse> getStatisticsMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest, com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanStatisticsResponse> getStatisticsMethod;
    if ((getStatisticsMethod = PodmanGrpc.getStatisticsMethod) == null) {
      synchronized (PodmanGrpc.class) {
        if ((getStatisticsMethod = PodmanGrpc.getStatisticsMethod) == null) {
          PodmanGrpc.getStatisticsMethod = getStatisticsMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest, com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanStatisticsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Statistics"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanStatisticsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PodmanMethodDescriptorSupplier("Statistics"))
              .build();
        }
      }
    }
    return getStatisticsMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static PodmanStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PodmanStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PodmanStub>() {
        @java.lang.Override
        public PodmanStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PodmanStub(channel, callOptions);
        }
      };
    return PodmanStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static PodmanBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PodmanBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PodmanBlockingV2Stub>() {
        @java.lang.Override
        public PodmanBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PodmanBlockingV2Stub(channel, callOptions);
        }
      };
    return PodmanBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static PodmanBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PodmanBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PodmanBlockingStub>() {
        @java.lang.Override
        public PodmanBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PodmanBlockingStub(channel, callOptions);
        }
      };
    return PodmanBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static PodmanFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PodmanFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PodmanFutureStub>() {
        @java.lang.Override
        public PodmanFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PodmanFutureStub(channel, callOptions);
        }
      };
    return PodmanFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void logs(com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanLogsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getLogsMethod(), responseObserver);
    }

    /**
     */
    default void containers(com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanContainersResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getContainersMethod(), responseObserver);
    }

    /**
     */
    default void statistics(com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanStatisticsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getStatisticsMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service Podman.
   */
  public static abstract class PodmanImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return PodmanGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Podman.
   */
  public static final class PodmanStub
      extends io.grpc.stub.AbstractAsyncStub<PodmanStub> {
    private PodmanStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PodmanStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PodmanStub(channel, callOptions);
    }

    /**
     */
    public void logs(com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanLogsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getLogsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void containers(com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanContainersResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getContainersMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void statistics(com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanStatisticsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getStatisticsMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Podman.
   */
  public static final class PodmanBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<PodmanBlockingV2Stub> {
    private PodmanBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PodmanBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PodmanBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanLogsResponse logs(com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getLogsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanContainersResponse containers(com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getContainersMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanStatisticsResponse statistics(com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getStatisticsMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Podman.
   */
  public static final class PodmanBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<PodmanBlockingStub> {
    private PodmanBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PodmanBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PodmanBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanLogsResponse logs(com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getLogsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanContainersResponse containers(com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getContainersMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanStatisticsResponse statistics(com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getStatisticsMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Podman.
   */
  public static final class PodmanFutureStub
      extends io.grpc.stub.AbstractFutureStub<PodmanFutureStub> {
    private PodmanFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PodmanFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PodmanFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanLogsResponse> logs(
        com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getLogsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanContainersResponse> containers(
        com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getContainersMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanStatisticsResponse> statistics(
        com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getStatisticsMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_LOGS = 0;
  private static final int METHODID_CONTAINERS = 1;
  private static final int METHODID_STATISTICS = 2;

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
        case METHODID_LOGS:
          serviceImpl.logs((com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanLogsResponse>) responseObserver);
          break;
        case METHODID_CONTAINERS:
          serviceImpl.containers((com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanContainersResponse>) responseObserver);
          break;
        case METHODID_STATISTICS:
          serviceImpl.statistics((com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanStatisticsResponse>) responseObserver);
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
          getLogsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest,
              com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanLogsResponse>(
                service, METHODID_LOGS)))
        .addMethod(
          getContainersMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest,
              com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanContainersResponse>(
                service, METHODID_CONTAINERS)))
        .addMethod(
          getStatisticsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest,
              com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanStatisticsResponse>(
                service, METHODID_STATISTICS)))
        .build();
  }

  private static abstract class PodmanBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    PodmanBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.monitoring.v1.Monitoring.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Podman");
    }
  }

  private static final class PodmanFileDescriptorSupplier
      extends PodmanBaseDescriptorSupplier {
    PodmanFileDescriptorSupplier() {}
  }

  private static final class PodmanMethodDescriptorSupplier
      extends PodmanBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    PodmanMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (PodmanGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new PodmanFileDescriptorSupplier())
              .addMethod(getLogsMethod())
              .addMethod(getContainersMethod())
              .addMethod(getStatisticsMethod())
              .build();
        }
      }
    }
    return result;
  }
}
