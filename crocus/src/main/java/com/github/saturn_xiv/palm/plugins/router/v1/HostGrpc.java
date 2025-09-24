package com.github.saturn_xiv.palm.plugins.router.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@javax.annotation.Generated(
    value = "by gRPC proto compiler (version 1.71.0)",
    comments = "Source: router.proto")
@io.grpc.stub.annotations.GrpcGenerated
public final class HostGrpc {

  private HostGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.router.v1.Host";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.router.v1.HostIndexResponse> getIndexMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Index",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.router.v1.HostIndexResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.router.v1.HostIndexResponse> getIndexMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.router.v1.HostIndexResponse> getIndexMethod;
    if ((getIndexMethod = HostGrpc.getIndexMethod) == null) {
      synchronized (HostGrpc.class) {
        if ((getIndexMethod = HostGrpc.getIndexMethod) == null) {
          HostGrpc.getIndexMethod = getIndexMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.router.v1.HostIndexResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Index"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.router.v1.HostIndexResponse.getDefaultInstance()))
              .setSchemaDescriptor(new HostMethodDescriptorSupplier("Index"))
              .build();
        }
      }
    }
    return getIndexMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.router.v1.HostSetDescriptionRequest,
      com.google.protobuf.Empty> getSetDescriptionMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SetDescription",
      requestType = com.github.saturn_xiv.palm.plugins.router.v1.HostSetDescriptionRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.router.v1.HostSetDescriptionRequest,
      com.google.protobuf.Empty> getSetDescriptionMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.router.v1.HostSetDescriptionRequest, com.google.protobuf.Empty> getSetDescriptionMethod;
    if ((getSetDescriptionMethod = HostGrpc.getSetDescriptionMethod) == null) {
      synchronized (HostGrpc.class) {
        if ((getSetDescriptionMethod = HostGrpc.getSetDescriptionMethod) == null) {
          HostGrpc.getSetDescriptionMethod = getSetDescriptionMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.router.v1.HostSetDescriptionRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SetDescription"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.router.v1.HostSetDescriptionRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new HostMethodDescriptorSupplier("SetDescription"))
              .build();
        }
      }
    }
    return getSetDescriptionMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest,
      com.google.protobuf.Empty> getEnableMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Enable",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest,
      com.google.protobuf.Empty> getEnableMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest, com.google.protobuf.Empty> getEnableMethod;
    if ((getEnableMethod = HostGrpc.getEnableMethod) == null) {
      synchronized (HostGrpc.class) {
        if ((getEnableMethod = HostGrpc.getEnableMethod) == null) {
          HostGrpc.getEnableMethod = getEnableMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Enable"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new HostMethodDescriptorSupplier("Enable"))
              .build();
        }
      }
    }
    return getEnableMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest,
      com.google.protobuf.Empty> getDisableMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Disable",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest,
      com.google.protobuf.Empty> getDisableMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest, com.google.protobuf.Empty> getDisableMethod;
    if ((getDisableMethod = HostGrpc.getDisableMethod) == null) {
      synchronized (HostGrpc.class) {
        if ((getDisableMethod = HostGrpc.getDisableMethod) == null) {
          HostGrpc.getDisableMethod = getDisableMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Disable"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new HostMethodDescriptorSupplier("Disable"))
              .build();
        }
      }
    }
    return getDisableMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.router.v1.HostBlockRequest,
      com.google.protobuf.Empty> getBlockMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Block",
      requestType = com.github.saturn_xiv.palm.plugins.router.v1.HostBlockRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.router.v1.HostBlockRequest,
      com.google.protobuf.Empty> getBlockMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.router.v1.HostBlockRequest, com.google.protobuf.Empty> getBlockMethod;
    if ((getBlockMethod = HostGrpc.getBlockMethod) == null) {
      synchronized (HostGrpc.class) {
        if ((getBlockMethod = HostGrpc.getBlockMethod) == null) {
          HostGrpc.getBlockMethod = getBlockMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.router.v1.HostBlockRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Block"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.router.v1.HostBlockRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new HostMethodDescriptorSupplier("Block"))
              .build();
        }
      }
    }
    return getBlockMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.router.v1.HostSetStaticIpAddressRequest,
      com.google.protobuf.Empty> getSetStaticIpAddressMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SetStaticIpAddress",
      requestType = com.github.saturn_xiv.palm.plugins.router.v1.HostSetStaticIpAddressRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.router.v1.HostSetStaticIpAddressRequest,
      com.google.protobuf.Empty> getSetStaticIpAddressMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.router.v1.HostSetStaticIpAddressRequest, com.google.protobuf.Empty> getSetStaticIpAddressMethod;
    if ((getSetStaticIpAddressMethod = HostGrpc.getSetStaticIpAddressMethod) == null) {
      synchronized (HostGrpc.class) {
        if ((getSetStaticIpAddressMethod = HostGrpc.getSetStaticIpAddressMethod) == null) {
          HostGrpc.getSetStaticIpAddressMethod = getSetStaticIpAddressMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.router.v1.HostSetStaticIpAddressRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SetStaticIpAddress"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.router.v1.HostSetStaticIpAddressRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new HostMethodDescriptorSupplier("SetStaticIpAddress"))
              .build();
        }
      }
    }
    return getSetStaticIpAddressMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.router.v1.HostSetDhcpAddressRequest,
      com.google.protobuf.Empty> getSetDhcpAddressMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SetDhcpAddress",
      requestType = com.github.saturn_xiv.palm.plugins.router.v1.HostSetDhcpAddressRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.router.v1.HostSetDhcpAddressRequest,
      com.google.protobuf.Empty> getSetDhcpAddressMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.router.v1.HostSetDhcpAddressRequest, com.google.protobuf.Empty> getSetDhcpAddressMethod;
    if ((getSetDhcpAddressMethod = HostGrpc.getSetDhcpAddressMethod) == null) {
      synchronized (HostGrpc.class) {
        if ((getSetDhcpAddressMethod = HostGrpc.getSetDhcpAddressMethod) == null) {
          HostGrpc.getSetDhcpAddressMethod = getSetDhcpAddressMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.router.v1.HostSetDhcpAddressRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SetDhcpAddress"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.router.v1.HostSetDhcpAddressRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new HostMethodDescriptorSupplier("SetDhcpAddress"))
              .build();
        }
      }
    }
    return getSetDhcpAddressMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static HostStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<HostStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<HostStub>() {
        @java.lang.Override
        public HostStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new HostStub(channel, callOptions);
        }
      };
    return HostStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static HostBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<HostBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<HostBlockingV2Stub>() {
        @java.lang.Override
        public HostBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new HostBlockingV2Stub(channel, callOptions);
        }
      };
    return HostBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static HostBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<HostBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<HostBlockingStub>() {
        @java.lang.Override
        public HostBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new HostBlockingStub(channel, callOptions);
        }
      };
    return HostBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static HostFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<HostFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<HostFutureStub>() {
        @java.lang.Override
        public HostFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new HostFutureStub(channel, callOptions);
        }
      };
    return HostFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void index(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.router.v1.HostIndexResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getIndexMethod(), responseObserver);
    }

    /**
     */
    default void setDescription(com.github.saturn_xiv.palm.plugins.router.v1.HostSetDescriptionRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSetDescriptionMethod(), responseObserver);
    }

    /**
     */
    default void enable(com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getEnableMethod(), responseObserver);
    }

    /**
     */
    default void disable(com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDisableMethod(), responseObserver);
    }

    /**
     */
    default void block(com.github.saturn_xiv.palm.plugins.router.v1.HostBlockRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getBlockMethod(), responseObserver);
    }

    /**
     */
    default void setStaticIpAddress(com.github.saturn_xiv.palm.plugins.router.v1.HostSetStaticIpAddressRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSetStaticIpAddressMethod(), responseObserver);
    }

    /**
     */
    default void setDhcpAddress(com.github.saturn_xiv.palm.plugins.router.v1.HostSetDhcpAddressRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSetDhcpAddressMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service Host.
   */
  public static abstract class HostImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return HostGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Host.
   */
  public static final class HostStub
      extends io.grpc.stub.AbstractAsyncStub<HostStub> {
    private HostStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected HostStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new HostStub(channel, callOptions);
    }

    /**
     */
    public void index(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.router.v1.HostIndexResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void setDescription(com.github.saturn_xiv.palm.plugins.router.v1.HostSetDescriptionRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSetDescriptionMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void enable(com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getEnableMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void disable(com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDisableMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void block(com.github.saturn_xiv.palm.plugins.router.v1.HostBlockRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getBlockMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void setStaticIpAddress(com.github.saturn_xiv.palm.plugins.router.v1.HostSetStaticIpAddressRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSetStaticIpAddressMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void setDhcpAddress(com.github.saturn_xiv.palm.plugins.router.v1.HostSetDhcpAddressRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSetDhcpAddressMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Host.
   */
  public static final class HostBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<HostBlockingV2Stub> {
    private HostBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected HostBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new HostBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.router.v1.HostIndexResponse index(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getIndexMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setDescription(com.github.saturn_xiv.palm.plugins.router.v1.HostSetDescriptionRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetDescriptionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty enable(com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getEnableMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty disable(com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDisableMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty block(com.github.saturn_xiv.palm.plugins.router.v1.HostBlockRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getBlockMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setStaticIpAddress(com.github.saturn_xiv.palm.plugins.router.v1.HostSetStaticIpAddressRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetStaticIpAddressMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setDhcpAddress(com.github.saturn_xiv.palm.plugins.router.v1.HostSetDhcpAddressRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetDhcpAddressMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Host.
   */
  public static final class HostBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<HostBlockingStub> {
    private HostBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected HostBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new HostBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.router.v1.HostIndexResponse index(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getIndexMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setDescription(com.github.saturn_xiv.palm.plugins.router.v1.HostSetDescriptionRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetDescriptionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty enable(com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getEnableMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty disable(com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDisableMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty block(com.github.saturn_xiv.palm.plugins.router.v1.HostBlockRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getBlockMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setStaticIpAddress(com.github.saturn_xiv.palm.plugins.router.v1.HostSetStaticIpAddressRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetStaticIpAddressMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setDhcpAddress(com.github.saturn_xiv.palm.plugins.router.v1.HostSetDhcpAddressRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetDhcpAddressMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Host.
   */
  public static final class HostFutureStub
      extends io.grpc.stub.AbstractFutureStub<HostFutureStub> {
    private HostFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected HostFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new HostFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.router.v1.HostIndexResponse> index(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> setDescription(
        com.github.saturn_xiv.palm.plugins.router.v1.HostSetDescriptionRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSetDescriptionMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> enable(
        com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getEnableMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> disable(
        com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDisableMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> block(
        com.github.saturn_xiv.palm.plugins.router.v1.HostBlockRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getBlockMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> setStaticIpAddress(
        com.github.saturn_xiv.palm.plugins.router.v1.HostSetStaticIpAddressRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSetStaticIpAddressMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> setDhcpAddress(
        com.github.saturn_xiv.palm.plugins.router.v1.HostSetDhcpAddressRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSetDhcpAddressMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_INDEX = 0;
  private static final int METHODID_SET_DESCRIPTION = 1;
  private static final int METHODID_ENABLE = 2;
  private static final int METHODID_DISABLE = 3;
  private static final int METHODID_BLOCK = 4;
  private static final int METHODID_SET_STATIC_IP_ADDRESS = 5;
  private static final int METHODID_SET_DHCP_ADDRESS = 6;

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
        case METHODID_INDEX:
          serviceImpl.index((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.router.v1.HostIndexResponse>) responseObserver);
          break;
        case METHODID_SET_DESCRIPTION:
          serviceImpl.setDescription((com.github.saturn_xiv.palm.plugins.router.v1.HostSetDescriptionRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_ENABLE:
          serviceImpl.enable((com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_DISABLE:
          serviceImpl.disable((com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_BLOCK:
          serviceImpl.block((com.github.saturn_xiv.palm.plugins.router.v1.HostBlockRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_SET_STATIC_IP_ADDRESS:
          serviceImpl.setStaticIpAddress((com.github.saturn_xiv.palm.plugins.router.v1.HostSetStaticIpAddressRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_SET_DHCP_ADDRESS:
          serviceImpl.setDhcpAddress((com.github.saturn_xiv.palm.plugins.router.v1.HostSetDhcpAddressRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
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
          getIndexMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.router.v1.HostIndexResponse>(
                service, METHODID_INDEX)))
        .addMethod(
          getSetDescriptionMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.router.v1.HostSetDescriptionRequest,
              com.google.protobuf.Empty>(
                service, METHODID_SET_DESCRIPTION)))
        .addMethod(
          getEnableMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest,
              com.google.protobuf.Empty>(
                service, METHODID_ENABLE)))
        .addMethod(
          getDisableMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest,
              com.google.protobuf.Empty>(
                service, METHODID_DISABLE)))
        .addMethod(
          getBlockMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.router.v1.HostBlockRequest,
              com.google.protobuf.Empty>(
                service, METHODID_BLOCK)))
        .addMethod(
          getSetStaticIpAddressMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.router.v1.HostSetStaticIpAddressRequest,
              com.google.protobuf.Empty>(
                service, METHODID_SET_STATIC_IP_ADDRESS)))
        .addMethod(
          getSetDhcpAddressMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.router.v1.HostSetDhcpAddressRequest,
              com.google.protobuf.Empty>(
                service, METHODID_SET_DHCP_ADDRESS)))
        .build();
  }

  private static abstract class HostBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    HostBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.router.v1.RouterOuterClass.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Host");
    }
  }

  private static final class HostFileDescriptorSupplier
      extends HostBaseDescriptorSupplier {
    HostFileDescriptorSupplier() {}
  }

  private static final class HostMethodDescriptorSupplier
      extends HostBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    HostMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (HostGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new HostFileDescriptorSupplier())
              .addMethod(getIndexMethod())
              .addMethod(getSetDescriptionMethod())
              .addMethod(getEnableMethod())
              .addMethod(getDisableMethod())
              .addMethod(getBlockMethod())
              .addMethod(getSetStaticIpAddressMethod())
              .addMethod(getSetDhcpAddressMethod())
              .build();
        }
      }
    }
    return result;
  }
}
