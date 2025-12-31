package com.github.saturn_xiv.palm.plugins.babel.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class ExaminationGrpc {

  private ExaminationGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.babel.v1.Examination";

  // Static method descriptors that strictly reflect the proto.
  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static ExaminationStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<ExaminationStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<ExaminationStub>() {
        @java.lang.Override
        public ExaminationStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new ExaminationStub(channel, callOptions);
        }
      };
    return ExaminationStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static ExaminationBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<ExaminationBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<ExaminationBlockingV2Stub>() {
        @java.lang.Override
        public ExaminationBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new ExaminationBlockingV2Stub(channel, callOptions);
        }
      };
    return ExaminationBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static ExaminationBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<ExaminationBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<ExaminationBlockingStub>() {
        @java.lang.Override
        public ExaminationBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new ExaminationBlockingStub(channel, callOptions);
        }
      };
    return ExaminationBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static ExaminationFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<ExaminationFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<ExaminationFutureStub>() {
        @java.lang.Override
        public ExaminationFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new ExaminationFutureStub(channel, callOptions);
        }
      };
    return ExaminationFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {
  }

  /**
   * Base class for the server implementation of the service Examination.
   */
  public static abstract class ExaminationImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return ExaminationGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Examination.
   */
  public static final class ExaminationStub
      extends io.grpc.stub.AbstractAsyncStub<ExaminationStub> {
    private ExaminationStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected ExaminationStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new ExaminationStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Examination.
   */
  public static final class ExaminationBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<ExaminationBlockingV2Stub> {
    private ExaminationBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected ExaminationBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new ExaminationBlockingV2Stub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Examination.
   */
  public static final class ExaminationBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<ExaminationBlockingStub> {
    private ExaminationBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected ExaminationBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new ExaminationBlockingStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Examination.
   */
  public static final class ExaminationFutureStub
      extends io.grpc.stub.AbstractFutureStub<ExaminationFutureStub> {
    private ExaminationFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected ExaminationFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new ExaminationFutureStub(channel, callOptions);
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

  private static abstract class ExaminationBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    ExaminationBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.babel.v1.BabelProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Examination");
    }
  }

  private static final class ExaminationFileDescriptorSupplier
      extends ExaminationBaseDescriptorSupplier {
    ExaminationFileDescriptorSupplier() {}
  }

  private static final class ExaminationMethodDescriptorSupplier
      extends ExaminationBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    ExaminationMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (ExaminationGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new ExaminationFileDescriptorSupplier())
              .build();
        }
      }
    }
    return result;
  }
}
