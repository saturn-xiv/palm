package com.github.saturn_xiv.palm.plugins.dictionary.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class DictionaryGrpc {

  private DictionaryGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.dictionary.v1.Dictionary";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchRequest,
      com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchResponse> getSearchMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Search",
      requestType = com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchRequest,
      com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchResponse> getSearchMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchRequest, com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchResponse> getSearchMethod;
    if ((getSearchMethod = DictionaryGrpc.getSearchMethod) == null) {
      synchronized (DictionaryGrpc.class) {
        if ((getSearchMethod = DictionaryGrpc.getSearchMethod) == null) {
          DictionaryGrpc.getSearchMethod = getSearchMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchRequest, com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Search"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchResponse.getDefaultInstance()))
              .setSchemaDescriptor(new DictionaryMethodDescriptorSupplier("Search"))
              .build();
        }
      }
    }
    return getSearchMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectoryIndexResponse> getIndexMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Index",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectoryIndexResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectoryIndexResponse> getIndexMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectoryIndexResponse> getIndexMethod;
    if ((getIndexMethod = DictionaryGrpc.getIndexMethod) == null) {
      synchronized (DictionaryGrpc.class) {
        if ((getIndexMethod = DictionaryGrpc.getIndexMethod) == null) {
          DictionaryGrpc.getIndexMethod = getIndexMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectoryIndexResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Index"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectoryIndexResponse.getDefaultInstance()))
              .setSchemaDescriptor(new DictionaryMethodDescriptorSupplier("Index"))
              .build();
        }
      }
    }
    return getIndexMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static DictionaryStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<DictionaryStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<DictionaryStub>() {
        @java.lang.Override
        public DictionaryStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new DictionaryStub(channel, callOptions);
        }
      };
    return DictionaryStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static DictionaryBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<DictionaryBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<DictionaryBlockingV2Stub>() {
        @java.lang.Override
        public DictionaryBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new DictionaryBlockingV2Stub(channel, callOptions);
        }
      };
    return DictionaryBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static DictionaryBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<DictionaryBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<DictionaryBlockingStub>() {
        @java.lang.Override
        public DictionaryBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new DictionaryBlockingStub(channel, callOptions);
        }
      };
    return DictionaryBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static DictionaryFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<DictionaryFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<DictionaryFutureStub>() {
        @java.lang.Override
        public DictionaryFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new DictionaryFutureStub(channel, callOptions);
        }
      };
    return DictionaryFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void search(com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSearchMethod(), responseObserver);
    }

    /**
     */
    default void index(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectoryIndexResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getIndexMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service Dictionary.
   */
  public static abstract class DictionaryImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return DictionaryGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Dictionary.
   */
  public static final class DictionaryStub
      extends io.grpc.stub.AbstractAsyncStub<DictionaryStub> {
    private DictionaryStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected DictionaryStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new DictionaryStub(channel, callOptions);
    }

    /**
     */
    public void search(com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSearchMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void index(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectoryIndexResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Dictionary.
   */
  public static final class DictionaryBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<DictionaryBlockingV2Stub> {
    private DictionaryBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected DictionaryBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new DictionaryBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchResponse search(com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getSearchMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectoryIndexResponse index(com.google.protobuf.Empty request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getIndexMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Dictionary.
   */
  public static final class DictionaryBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<DictionaryBlockingStub> {
    private DictionaryBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected DictionaryBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new DictionaryBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchResponse search(com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSearchMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectoryIndexResponse index(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getIndexMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Dictionary.
   */
  public static final class DictionaryFutureStub
      extends io.grpc.stub.AbstractFutureStub<DictionaryFutureStub> {
    private DictionaryFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected DictionaryFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new DictionaryFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchResponse> search(
        com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSearchMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectoryIndexResponse> index(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_SEARCH = 0;
  private static final int METHODID_INDEX = 1;

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
        case METHODID_SEARCH:
          serviceImpl.search((com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchResponse>) responseObserver);
          break;
        case METHODID_INDEX:
          serviceImpl.index((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectoryIndexResponse>) responseObserver);
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
          getSearchMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchRequest,
              com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectorySearchResponse>(
                service, METHODID_SEARCH)))
        .addMethod(
          getIndexMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.dictionary.v1.DirectoryIndexResponse>(
                service, METHODID_INDEX)))
        .build();
  }

  private static abstract class DictionaryBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    DictionaryBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.dictionary.v1.DictionaryOuterClass.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Dictionary");
    }
  }

  private static final class DictionaryFileDescriptorSupplier
      extends DictionaryBaseDescriptorSupplier {
    DictionaryFileDescriptorSupplier() {}
  }

  private static final class DictionaryMethodDescriptorSupplier
      extends DictionaryBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    DictionaryMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (DictionaryGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new DictionaryFileDescriptorSupplier())
              .addMethod(getSearchMethod())
              .addMethod(getIndexMethod())
              .build();
        }
      }
    }
    return result;
  }
}
