package com.github.saturn_xiv.palm.plugins.monitoring.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class DockerGrpc {

  private DockerGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.monitoring.v1.Docker";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest,
      com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerContainersResponse> getContainersMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Containers",
      requestType = com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerContainersResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest,
      com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerContainersResponse> getContainersMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest, com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerContainersResponse> getContainersMethod;
    if ((getContainersMethod = DockerGrpc.getContainersMethod) == null) {
      synchronized (DockerGrpc.class) {
        if ((getContainersMethod = DockerGrpc.getContainersMethod) == null) {
          DockerGrpc.getContainersMethod = getContainersMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest, com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerContainersResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Containers"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerContainersResponse.getDefaultInstance()))
              .setSchemaDescriptor(new DockerMethodDescriptorSupplier("Containers"))
              .build();
        }
      }
    }
    return getContainersMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest,
      com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerStatisticsResponse> getStatisticsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Statistics",
      requestType = com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerStatisticsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest,
      com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerStatisticsResponse> getStatisticsMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest, com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerStatisticsResponse> getStatisticsMethod;
    if ((getStatisticsMethod = DockerGrpc.getStatisticsMethod) == null) {
      synchronized (DockerGrpc.class) {
        if ((getStatisticsMethod = DockerGrpc.getStatisticsMethod) == null) {
          DockerGrpc.getStatisticsMethod = getStatisticsMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest, com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerStatisticsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Statistics"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerStatisticsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new DockerMethodDescriptorSupplier("Statistics"))
              .build();
        }
      }
    }
    return getStatisticsMethod;
  }

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
    if ((getLogsMethod = DockerGrpc.getLogsMethod) == null) {
      synchronized (DockerGrpc.class) {
        if ((getLogsMethod = DockerGrpc.getLogsMethod) == null) {
          DockerGrpc.getLogsMethod = getLogsMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest, com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanLogsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Logs"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanLogsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new DockerMethodDescriptorSupplier("Logs"))
              .build();
        }
      }
    }
    return getLogsMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static DockerStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<DockerStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<DockerStub>() {
        @java.lang.Override
        public DockerStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new DockerStub(channel, callOptions);
        }
      };
    return DockerStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static DockerBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<DockerBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<DockerBlockingV2Stub>() {
        @java.lang.Override
        public DockerBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new DockerBlockingV2Stub(channel, callOptions);
        }
      };
    return DockerBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static DockerBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<DockerBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<DockerBlockingStub>() {
        @java.lang.Override
        public DockerBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new DockerBlockingStub(channel, callOptions);
        }
      };
    return DockerBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static DockerFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<DockerFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<DockerFutureStub>() {
        @java.lang.Override
        public DockerFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new DockerFutureStub(channel, callOptions);
        }
      };
    return DockerFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void containers(com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerContainersResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getContainersMethod(), responseObserver);
    }

    /**
     */
    default void statistics(com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerStatisticsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getStatisticsMethod(), responseObserver);
    }

    /**
     */
    default void logs(com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanLogsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getLogsMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service Docker.
   */
  public static abstract class DockerImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return DockerGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Docker.
   */
  public static final class DockerStub
      extends io.grpc.stub.AbstractAsyncStub<DockerStub> {
    private DockerStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected DockerStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new DockerStub(channel, callOptions);
    }

    /**
     */
    public void containers(com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerContainersResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getContainersMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void statistics(com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerStatisticsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getStatisticsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void logs(com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanLogsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getLogsMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Docker.
   */
  public static final class DockerBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<DockerBlockingV2Stub> {
    private DockerBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected DockerBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new DockerBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerContainersResponse containers(com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getContainersMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerStatisticsResponse statistics(com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getStatisticsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanLogsResponse logs(com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getLogsMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Docker.
   */
  public static final class DockerBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<DockerBlockingStub> {
    private DockerBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected DockerBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new DockerBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerContainersResponse containers(com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getContainersMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerStatisticsResponse statistics(com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getStatisticsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanLogsResponse logs(com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getLogsMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Docker.
   */
  public static final class DockerFutureStub
      extends io.grpc.stub.AbstractFutureStub<DockerFutureStub> {
    private DockerFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected DockerFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new DockerFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerContainersResponse> containers(
        com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getContainersMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerStatisticsResponse> statistics(
        com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getStatisticsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanLogsResponse> logs(
        com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getLogsMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_CONTAINERS = 0;
  private static final int METHODID_STATISTICS = 1;
  private static final int METHODID_LOGS = 2;

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
        case METHODID_CONTAINERS:
          serviceImpl.containers((com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerContainersResponse>) responseObserver);
          break;
        case METHODID_STATISTICS:
          serviceImpl.statistics((com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerStatisticsResponse>) responseObserver);
          break;
        case METHODID_LOGS:
          serviceImpl.logs((com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanLogsResponse>) responseObserver);
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
          getContainersMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest,
              com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerContainersResponse>(
                service, METHODID_CONTAINERS)))
        .addMethod(
          getStatisticsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest,
              com.github.saturn_xiv.palm.plugins.monitoring.v1.DockerStatisticsResponse>(
                service, METHODID_STATISTICS)))
        .addMethod(
          getLogsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanQueryRequest,
              com.github.saturn_xiv.palm.plugins.monitoring.v1.PodmanLogsResponse>(
                service, METHODID_LOGS)))
        .build();
  }

  private static abstract class DockerBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    DockerBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.monitoring.v1.Monitoring.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Docker");
    }
  }

  private static final class DockerFileDescriptorSupplier
      extends DockerBaseDescriptorSupplier {
    DockerFileDescriptorSupplier() {}
  }

  private static final class DockerMethodDescriptorSupplier
      extends DockerBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    DockerMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (DockerGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new DockerFileDescriptorSupplier())
              .addMethod(getContainersMethod())
              .addMethod(getStatisticsMethod())
              .addMethod(getLogsMethod())
              .build();
        }
      }
    }
    return result;
  }
}
