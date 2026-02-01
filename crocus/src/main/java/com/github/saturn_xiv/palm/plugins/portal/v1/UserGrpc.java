package com.github.saturn_xiv.palm.plugins.portal.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class UserGrpc {

  private UserGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.portal.v1.User";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page,
      com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse> getIndexMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Index",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.Page.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page,
      com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse> getIndexMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page, com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse> getIndexMethod;
    if ((getIndexMethod = UserGrpc.getIndexMethod) == null) {
      synchronized (UserGrpc.class) {
        if ((getIndexMethod = UserGrpc.getIndexMethod) == null) {
          UserGrpc.getIndexMethod = getIndexMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.Page, com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Index"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.Page.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse.getDefaultInstance()))
              .setSchemaDescriptor(new UserMethodDescriptorSupplier("Index"))
              .build();
        }
      }
    }
    return getIndexMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest,
      com.google.protobuf.Empty> getDestroyMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Destroy",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest,
      com.google.protobuf.Empty> getDestroyMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest, com.google.protobuf.Empty> getDestroyMethod;
    if ((getDestroyMethod = UserGrpc.getDestroyMethod) == null) {
      synchronized (UserGrpc.class) {
        if ((getDestroyMethod = UserGrpc.getDestroyMethod) == null) {
          UserGrpc.getDestroyMethod = getDestroyMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Destroy"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new UserMethodDescriptorSupplier("Destroy"))
              .build();
        }
      }
    }
    return getDestroyMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest,
      com.google.protobuf.Empty> getLockMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Lock",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest,
      com.google.protobuf.Empty> getLockMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest, com.google.protobuf.Empty> getLockMethod;
    if ((getLockMethod = UserGrpc.getLockMethod) == null) {
      synchronized (UserGrpc.class) {
        if ((getLockMethod = UserGrpc.getLockMethod) == null) {
          UserGrpc.getLockMethod = getLockMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Lock"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new UserMethodDescriptorSupplier("Lock"))
              .build();
        }
      }
    }
    return getLockMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest,
      com.google.protobuf.Empty> getUnlockMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Unlock",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest,
      com.google.protobuf.Empty> getUnlockMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest, com.google.protobuf.Empty> getUnlockMethod;
    if ((getUnlockMethod = UserGrpc.getUnlockMethod) == null) {
      synchronized (UserGrpc.class) {
        if ((getUnlockMethod = UserGrpc.getUnlockMethod) == null) {
          UserGrpc.getUnlockMethod = getUnlockMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Unlock"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new UserMethodDescriptorSupplier("Unlock"))
              .build();
        }
      }
    }
    return getUnlockMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page,
      com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexAttachmentResponse> getIndexAttachmentMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "IndexAttachment",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.Page.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexAttachmentResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page,
      com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexAttachmentResponse> getIndexAttachmentMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page, com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexAttachmentResponse> getIndexAttachmentMethod;
    if ((getIndexAttachmentMethod = UserGrpc.getIndexAttachmentMethod) == null) {
      synchronized (UserGrpc.class) {
        if ((getIndexAttachmentMethod = UserGrpc.getIndexAttachmentMethod) == null) {
          UserGrpc.getIndexAttachmentMethod = getIndexAttachmentMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.Page, com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexAttachmentResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "IndexAttachment"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.Page.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexAttachmentResponse.getDefaultInstance()))
              .setSchemaDescriptor(new UserMethodDescriptorSupplier("IndexAttachment"))
              .build();
        }
      }
    }
    return getIndexAttachmentMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentRequest,
      com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentUploadResponse> getCreateAttachmentMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateAttachment",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentUploadResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentRequest,
      com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentUploadResponse> getCreateAttachmentMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentRequest, com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentUploadResponse> getCreateAttachmentMethod;
    if ((getCreateAttachmentMethod = UserGrpc.getCreateAttachmentMethod) == null) {
      synchronized (UserGrpc.class) {
        if ((getCreateAttachmentMethod = UserGrpc.getCreateAttachmentMethod) == null) {
          UserGrpc.getCreateAttachmentMethod = getCreateAttachmentMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentRequest, com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentUploadResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateAttachment"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentUploadResponse.getDefaultInstance()))
              .setSchemaDescriptor(new UserMethodDescriptorSupplier("CreateAttachment"))
              .build();
        }
      }
    }
    return getCreateAttachmentMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentRequest,
      com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentResponse> getShowAttachmentMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ShowAttachment",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentRequest,
      com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentResponse> getShowAttachmentMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentRequest, com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentResponse> getShowAttachmentMethod;
    if ((getShowAttachmentMethod = UserGrpc.getShowAttachmentMethod) == null) {
      synchronized (UserGrpc.class) {
        if ((getShowAttachmentMethod = UserGrpc.getShowAttachmentMethod) == null) {
          UserGrpc.getShowAttachmentMethod = getShowAttachmentMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentRequest, com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ShowAttachment"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentResponse.getDefaultInstance()))
              .setSchemaDescriptor(new UserMethodDescriptorSupplier("ShowAttachment"))
              .build();
        }
      }
    }
    return getShowAttachmentMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest,
      com.google.protobuf.Empty> getSetAttachmentUploadedMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SetAttachmentUploaded",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest,
      com.google.protobuf.Empty> getSetAttachmentUploadedMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest, com.google.protobuf.Empty> getSetAttachmentUploadedMethod;
    if ((getSetAttachmentUploadedMethod = UserGrpc.getSetAttachmentUploadedMethod) == null) {
      synchronized (UserGrpc.class) {
        if ((getSetAttachmentUploadedMethod = UserGrpc.getSetAttachmentUploadedMethod) == null) {
          UserGrpc.getSetAttachmentUploadedMethod = getSetAttachmentUploadedMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SetAttachmentUploaded"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new UserMethodDescriptorSupplier("SetAttachmentUploaded"))
              .build();
        }
      }
    }
    return getSetAttachmentUploadedMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.UserSetAttachmentTitleRequest,
      com.google.protobuf.Empty> getSetAttachmentTitleMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SetAttachmentTitle",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.UserSetAttachmentTitleRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.UserSetAttachmentTitleRequest,
      com.google.protobuf.Empty> getSetAttachmentTitleMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.UserSetAttachmentTitleRequest, com.google.protobuf.Empty> getSetAttachmentTitleMethod;
    if ((getSetAttachmentTitleMethod = UserGrpc.getSetAttachmentTitleMethod) == null) {
      synchronized (UserGrpc.class) {
        if ((getSetAttachmentTitleMethod = UserGrpc.getSetAttachmentTitleMethod) == null) {
          UserGrpc.getSetAttachmentTitleMethod = getSetAttachmentTitleMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.UserSetAttachmentTitleRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SetAttachmentTitle"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.UserSetAttachmentTitleRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new UserMethodDescriptorSupplier("SetAttachmentTitle"))
              .build();
        }
      }
    }
    return getSetAttachmentTitleMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest,
      com.google.protobuf.Empty> getDestroyAttachmentMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DestroyAttachment",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest,
      com.google.protobuf.Empty> getDestroyAttachmentMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest, com.google.protobuf.Empty> getDestroyAttachmentMethod;
    if ((getDestroyAttachmentMethod = UserGrpc.getDestroyAttachmentMethod) == null) {
      synchronized (UserGrpc.class) {
        if ((getDestroyAttachmentMethod = UserGrpc.getDestroyAttachmentMethod) == null) {
          UserGrpc.getDestroyAttachmentMethod = getDestroyAttachmentMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DestroyAttachment"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new UserMethodDescriptorSupplier("DestroyAttachment"))
              .build();
        }
      }
    }
    return getDestroyAttachmentMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static UserStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<UserStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<UserStub>() {
        @java.lang.Override
        public UserStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new UserStub(channel, callOptions);
        }
      };
    return UserStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static UserBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<UserBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<UserBlockingV2Stub>() {
        @java.lang.Override
        public UserBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new UserBlockingV2Stub(channel, callOptions);
        }
      };
    return UserBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static UserBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<UserBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<UserBlockingStub>() {
        @java.lang.Override
        public UserBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new UserBlockingStub(channel, callOptions);
        }
      };
    return UserBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static UserFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<UserFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<UserFutureStub>() {
        @java.lang.Override
        public UserFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new UserFutureStub(channel, callOptions);
        }
      };
    return UserFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getIndexMethod(), responseObserver);
    }

    /**
     */
    default void destroy(com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDestroyMethod(), responseObserver);
    }

    /**
     */
    default void lock(com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getLockMethod(), responseObserver);
    }

    /**
     */
    default void unlock(com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getUnlockMethod(), responseObserver);
    }

    /**
     */
    default void indexAttachment(com.github.saturn_xiv.palm.plugins.portal.v1.Page request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexAttachmentResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getIndexAttachmentMethod(), responseObserver);
    }

    /**
     */
    default void createAttachment(com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentUploadResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateAttachmentMethod(), responseObserver);
    }

    /**
     */
    default void showAttachment(com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getShowAttachmentMethod(), responseObserver);
    }

    /**
     */
    default void setAttachmentUploaded(com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSetAttachmentUploadedMethod(), responseObserver);
    }

    /**
     */
    default void setAttachmentTitle(com.github.saturn_xiv.palm.plugins.portal.v1.UserSetAttachmentTitleRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSetAttachmentTitleMethod(), responseObserver);
    }

    /**
     */
    default void destroyAttachment(com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDestroyAttachmentMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service User.
   */
  public static abstract class UserImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return UserGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service User.
   */
  public static final class UserStub
      extends io.grpc.stub.AbstractAsyncStub<UserStub> {
    private UserStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected UserStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new UserStub(channel, callOptions);
    }

    /**
     */
    public void index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void destroy(com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDestroyMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void lock(com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getLockMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void unlock(com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getUnlockMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void indexAttachment(com.github.saturn_xiv.palm.plugins.portal.v1.Page request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexAttachmentResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getIndexAttachmentMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createAttachment(com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentUploadResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateAttachmentMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void showAttachment(com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getShowAttachmentMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void setAttachmentUploaded(com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSetAttachmentUploadedMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void setAttachmentTitle(com.github.saturn_xiv.palm.plugins.portal.v1.UserSetAttachmentTitleRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSetAttachmentTitleMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void destroyAttachment(com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDestroyAttachmentMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service User.
   */
  public static final class UserBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<UserBlockingV2Stub> {
    private UserBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected UserBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new UserBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getIndexMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty destroy(com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getDestroyMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty lock(com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getLockMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty unlock(com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getUnlockMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexAttachmentResponse indexAttachment(com.github.saturn_xiv.palm.plugins.portal.v1.Page request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getIndexAttachmentMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentUploadResponse createAttachment(com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateAttachmentMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentResponse showAttachment(com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getShowAttachmentMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setAttachmentUploaded(com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getSetAttachmentUploadedMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setAttachmentTitle(com.github.saturn_xiv.palm.plugins.portal.v1.UserSetAttachmentTitleRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getSetAttachmentTitleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty destroyAttachment(com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getDestroyAttachmentMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service User.
   */
  public static final class UserBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<UserBlockingStub> {
    private UserBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected UserBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new UserBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getIndexMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty destroy(com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDestroyMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty lock(com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getLockMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty unlock(com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getUnlockMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexAttachmentResponse indexAttachment(com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getIndexAttachmentMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentUploadResponse createAttachment(com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateAttachmentMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentResponse showAttachment(com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getShowAttachmentMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setAttachmentUploaded(com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetAttachmentUploadedMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setAttachmentTitle(com.github.saturn_xiv.palm.plugins.portal.v1.UserSetAttachmentTitleRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetAttachmentTitleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty destroyAttachment(com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDestroyAttachmentMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service User.
   */
  public static final class UserFutureStub
      extends io.grpc.stub.AbstractFutureStub<UserFutureStub> {
    private UserFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected UserFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new UserFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse> index(
        com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> destroy(
        com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDestroyMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> lock(
        com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getLockMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> unlock(
        com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getUnlockMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexAttachmentResponse> indexAttachment(
        com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getIndexAttachmentMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentUploadResponse> createAttachment(
        com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateAttachmentMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentResponse> showAttachment(
        com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getShowAttachmentMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> setAttachmentUploaded(
        com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSetAttachmentUploadedMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> setAttachmentTitle(
        com.github.saturn_xiv.palm.plugins.portal.v1.UserSetAttachmentTitleRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSetAttachmentTitleMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> destroyAttachment(
        com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDestroyAttachmentMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_INDEX = 0;
  private static final int METHODID_DESTROY = 1;
  private static final int METHODID_LOCK = 2;
  private static final int METHODID_UNLOCK = 3;
  private static final int METHODID_INDEX_ATTACHMENT = 4;
  private static final int METHODID_CREATE_ATTACHMENT = 5;
  private static final int METHODID_SHOW_ATTACHMENT = 6;
  private static final int METHODID_SET_ATTACHMENT_UPLOADED = 7;
  private static final int METHODID_SET_ATTACHMENT_TITLE = 8;
  private static final int METHODID_DESTROY_ATTACHMENT = 9;

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
          serviceImpl.index((com.github.saturn_xiv.palm.plugins.portal.v1.Page) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse>) responseObserver);
          break;
        case METHODID_DESTROY:
          serviceImpl.destroy((com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_LOCK:
          serviceImpl.lock((com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_UNLOCK:
          serviceImpl.unlock((com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_INDEX_ATTACHMENT:
          serviceImpl.indexAttachment((com.github.saturn_xiv.palm.plugins.portal.v1.Page) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexAttachmentResponse>) responseObserver);
          break;
        case METHODID_CREATE_ATTACHMENT:
          serviceImpl.createAttachment((com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentUploadResponse>) responseObserver);
          break;
        case METHODID_SHOW_ATTACHMENT:
          serviceImpl.showAttachment((com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentResponse>) responseObserver);
          break;
        case METHODID_SET_ATTACHMENT_UPLOADED:
          serviceImpl.setAttachmentUploaded((com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_SET_ATTACHMENT_TITLE:
          serviceImpl.setAttachmentTitle((com.github.saturn_xiv.palm.plugins.portal.v1.UserSetAttachmentTitleRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_DESTROY_ATTACHMENT:
          serviceImpl.destroyAttachment((com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest) request,
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
              com.github.saturn_xiv.palm.plugins.portal.v1.Page,
              com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse>(
                service, METHODID_INDEX)))
        .addMethod(
          getDestroyMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest,
              com.google.protobuf.Empty>(
                service, METHODID_DESTROY)))
        .addMethod(
          getLockMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest,
              com.google.protobuf.Empty>(
                service, METHODID_LOCK)))
        .addMethod(
          getUnlockMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest,
              com.google.protobuf.Empty>(
                service, METHODID_UNLOCK)))
        .addMethod(
          getIndexAttachmentMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.Page,
              com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexAttachmentResponse>(
                service, METHODID_INDEX_ATTACHMENT)))
        .addMethod(
          getCreateAttachmentMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentRequest,
              com.github.saturn_xiv.palm.plugins.portal.v1.UserCreateAttachmentUploadResponse>(
                service, METHODID_CREATE_ATTACHMENT)))
        .addMethod(
          getShowAttachmentMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentRequest,
              com.github.saturn_xiv.palm.plugins.portal.v1.UserShowAttachmentResponse>(
                service, METHODID_SHOW_ATTACHMENT)))
        .addMethod(
          getSetAttachmentUploadedMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest,
              com.google.protobuf.Empty>(
                service, METHODID_SET_ATTACHMENT_UPLOADED)))
        .addMethod(
          getSetAttachmentTitleMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.UserSetAttachmentTitleRequest,
              com.google.protobuf.Empty>(
                service, METHODID_SET_ATTACHMENT_TITLE)))
        .addMethod(
          getDestroyAttachmentMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.IdRequest,
              com.google.protobuf.Empty>(
                service, METHODID_DESTROY_ATTACHMENT)))
        .build();
  }

  private static abstract class UserBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    UserBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.portal.v1.PortalProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("User");
    }
  }

  private static final class UserFileDescriptorSupplier
      extends UserBaseDescriptorSupplier {
    UserFileDescriptorSupplier() {}
  }

  private static final class UserMethodDescriptorSupplier
      extends UserBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    UserMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (UserGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new UserFileDescriptorSupplier())
              .addMethod(getIndexMethod())
              .addMethod(getDestroyMethod())
              .addMethod(getLockMethod())
              .addMethod(getUnlockMethod())
              .addMethod(getIndexAttachmentMethod())
              .addMethod(getCreateAttachmentMethod())
              .addMethod(getShowAttachmentMethod())
              .addMethod(getSetAttachmentUploadedMethod())
              .addMethod(getSetAttachmentTitleMethod())
              .addMethod(getDestroyAttachmentMethod())
              .build();
        }
      }
    }
    return result;
  }
}
