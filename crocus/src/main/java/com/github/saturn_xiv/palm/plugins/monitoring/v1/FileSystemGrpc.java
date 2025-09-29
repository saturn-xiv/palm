package com.github.saturn_xiv.palm.plugins.monitoring.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class FileSystemGrpc {

  private FileSystemGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.monitoring.v1.FileSystem";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsRequest,
      com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsResponse> getLogsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Logs",
      requestType = com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsRequest,
      com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsResponse> getLogsMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsRequest, com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsResponse> getLogsMethod;
    if ((getLogsMethod = FileSystemGrpc.getLogsMethod) == null) {
      synchronized (FileSystemGrpc.class) {
        if ((getLogsMethod = FileSystemGrpc.getLogsMethod) == null) {
          FileSystemGrpc.getLogsMethod = getLogsMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsRequest, com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Logs"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new FileSystemMethodDescriptorSupplier("Logs"))
              .build();
        }
      }
    }
    return getLogsMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static FileSystemStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<FileSystemStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<FileSystemStub>() {
        @java.lang.Override
        public FileSystemStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new FileSystemStub(channel, callOptions);
        }
      };
    return FileSystemStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static FileSystemBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<FileSystemBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<FileSystemBlockingV2Stub>() {
        @java.lang.Override
        public FileSystemBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new FileSystemBlockingV2Stub(channel, callOptions);
        }
      };
    return FileSystemBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static FileSystemBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<FileSystemBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<FileSystemBlockingStub>() {
        @java.lang.Override
        public FileSystemBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new FileSystemBlockingStub(channel, callOptions);
        }
      };
    return FileSystemBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static FileSystemFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<FileSystemFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<FileSystemFutureStub>() {
        @java.lang.Override
        public FileSystemFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new FileSystemFutureStub(channel, callOptions);
        }
      };
    return FileSystemFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void logs(com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getLogsMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service FileSystem.
   */
  public static abstract class FileSystemImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return FileSystemGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service FileSystem.
   */
  public static final class FileSystemStub
      extends io.grpc.stub.AbstractAsyncStub<FileSystemStub> {
    private FileSystemStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected FileSystemStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new FileSystemStub(channel, callOptions);
    }

    /**
     */
    public void logs(com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getLogsMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service FileSystem.
   */
  public static final class FileSystemBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<FileSystemBlockingV2Stub> {
    private FileSystemBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected FileSystemBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new FileSystemBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsResponse logs(com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getLogsMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service FileSystem.
   */
  public static final class FileSystemBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<FileSystemBlockingStub> {
    private FileSystemBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected FileSystemBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new FileSystemBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsResponse logs(com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getLogsMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service FileSystem.
   */
  public static final class FileSystemFutureStub
      extends io.grpc.stub.AbstractFutureStub<FileSystemFutureStub> {
    private FileSystemFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected FileSystemFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new FileSystemFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsResponse> logs(
        com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getLogsMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_LOGS = 0;

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
          serviceImpl.logs((com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsResponse>) responseObserver);
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
              com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsRequest,
              com.github.saturn_xiv.palm.plugins.monitoring.v1.FileSystemLogsResponse>(
                service, METHODID_LOGS)))
        .build();
  }

  private static abstract class FileSystemBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    FileSystemBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.monitoring.v1.Monitoring.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("FileSystem");
    }
  }

  private static final class FileSystemFileDescriptorSupplier
      extends FileSystemBaseDescriptorSupplier {
    FileSystemFileDescriptorSupplier() {}
  }

  private static final class FileSystemMethodDescriptorSupplier
      extends FileSystemBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    FileSystemMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (FileSystemGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new FileSystemFileDescriptorSupplier())
              .addMethod(getLogsMethod())
              .build();
        }
      }
    }
    return result;
  }
}
