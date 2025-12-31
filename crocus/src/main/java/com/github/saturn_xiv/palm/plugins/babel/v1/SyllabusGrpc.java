package com.github.saturn_xiv.palm.plugins.babel.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class SyllabusGrpc {

  private SyllabusGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.babel.v1.Syllabus";

  // Static method descriptors that strictly reflect the proto.
  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static SyllabusStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SyllabusStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SyllabusStub>() {
        @java.lang.Override
        public SyllabusStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SyllabusStub(channel, callOptions);
        }
      };
    return SyllabusStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static SyllabusBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SyllabusBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SyllabusBlockingV2Stub>() {
        @java.lang.Override
        public SyllabusBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SyllabusBlockingV2Stub(channel, callOptions);
        }
      };
    return SyllabusBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static SyllabusBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SyllabusBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SyllabusBlockingStub>() {
        @java.lang.Override
        public SyllabusBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SyllabusBlockingStub(channel, callOptions);
        }
      };
    return SyllabusBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static SyllabusFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SyllabusFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SyllabusFutureStub>() {
        @java.lang.Override
        public SyllabusFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SyllabusFutureStub(channel, callOptions);
        }
      };
    return SyllabusFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {
  }

  /**
   * Base class for the server implementation of the service Syllabus.
   */
  public static abstract class SyllabusImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return SyllabusGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Syllabus.
   */
  public static final class SyllabusStub
      extends io.grpc.stub.AbstractAsyncStub<SyllabusStub> {
    private SyllabusStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SyllabusStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SyllabusStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Syllabus.
   */
  public static final class SyllabusBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<SyllabusBlockingV2Stub> {
    private SyllabusBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SyllabusBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SyllabusBlockingV2Stub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Syllabus.
   */
  public static final class SyllabusBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<SyllabusBlockingStub> {
    private SyllabusBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SyllabusBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SyllabusBlockingStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Syllabus.
   */
  public static final class SyllabusFutureStub
      extends io.grpc.stub.AbstractFutureStub<SyllabusFutureStub> {
    private SyllabusFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SyllabusFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SyllabusFutureStub(channel, callOptions);
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

  private static abstract class SyllabusBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    SyllabusBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.babel.v1.BabelProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Syllabus");
    }
  }

  private static final class SyllabusFileDescriptorSupplier
      extends SyllabusBaseDescriptorSupplier {
    SyllabusFileDescriptorSupplier() {}
  }

  private static final class SyllabusMethodDescriptorSupplier
      extends SyllabusBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    SyllabusMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (SyllabusGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new SyllabusFileDescriptorSupplier())
              .build();
        }
      }
    }
    return result;
  }
}
