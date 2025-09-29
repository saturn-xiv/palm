package com.github.saturn_xiv.palm.plugins.bbs.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 * <pre>
 * ----------------------------------------------------------------------------
 * </pre>
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class TopicGrpc {

  private TopicGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.bbs.v1.Topic";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page,
      com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse> getIndexMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Index",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.Page.class,
      responseType = com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page,
      com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse> getIndexMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page, com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse> getIndexMethod;
    if ((getIndexMethod = TopicGrpc.getIndexMethod) == null) {
      synchronized (TopicGrpc.class) {
        if ((getIndexMethod = TopicGrpc.getIndexMethod) == null) {
          TopicGrpc.getIndexMethod = getIndexMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.Page, com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Index"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.Page.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse.getDefaultInstance()))
              .setSchemaDescriptor(new TopicMethodDescriptorSupplier("Index"))
              .build();
        }
      }
    }
    return getIndexMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByForumRequest,
      com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse> getByForumMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ByForum",
      requestType = com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByForumRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByForumRequest,
      com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse> getByForumMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByForumRequest, com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse> getByForumMethod;
    if ((getByForumMethod = TopicGrpc.getByForumMethod) == null) {
      synchronized (TopicGrpc.class) {
        if ((getByForumMethod = TopicGrpc.getByForumMethod) == null) {
          TopicGrpc.getByForumMethod = getByForumMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByForumRequest, com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ByForum"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByForumRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse.getDefaultInstance()))
              .setSchemaDescriptor(new TopicMethodDescriptorSupplier("ByForum"))
              .build();
        }
      }
    }
    return getByForumMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByUserRequest,
      com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse> getByUserMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ByUser",
      requestType = com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByUserRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByUserRequest,
      com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse> getByUserMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByUserRequest, com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse> getByUserMethod;
    if ((getByUserMethod = TopicGrpc.getByUserMethod) == null) {
      synchronized (TopicGrpc.class) {
        if ((getByUserMethod = TopicGrpc.getByUserMethod) == null) {
          TopicGrpc.getByUserMethod = getByUserMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByUserRequest, com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ByUser"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByUserRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse.getDefaultInstance()))
              .setSchemaDescriptor(new TopicMethodDescriptorSupplier("ByUser"))
              .build();
        }
      }
    }
    return getByUserMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest,
      com.github.saturn_xiv.palm.plugins.bbs.v1.TopicShowResponse> getShowMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Show",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.bbs.v1.TopicShowResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest,
      com.github.saturn_xiv.palm.plugins.bbs.v1.TopicShowResponse> getShowMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest, com.github.saturn_xiv.palm.plugins.bbs.v1.TopicShowResponse> getShowMethod;
    if ((getShowMethod = TopicGrpc.getShowMethod) == null) {
      synchronized (TopicGrpc.class) {
        if ((getShowMethod = TopicGrpc.getShowMethod) == null) {
          TopicGrpc.getShowMethod = getShowMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest, com.github.saturn_xiv.palm.plugins.bbs.v1.TopicShowResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Show"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.bbs.v1.TopicShowResponse.getDefaultInstance()))
              .setSchemaDescriptor(new TopicMethodDescriptorSupplier("Show"))
              .build();
        }
      }
    }
    return getShowMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page,
      com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse> getActiveMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Active",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.Page.class,
      responseType = com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page,
      com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse> getActiveMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page, com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse> getActiveMethod;
    if ((getActiveMethod = TopicGrpc.getActiveMethod) == null) {
      synchronized (TopicGrpc.class) {
        if ((getActiveMethod = TopicGrpc.getActiveMethod) == null) {
          TopicGrpc.getActiveMethod = getActiveMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.Page, com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Active"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.Page.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse.getDefaultInstance()))
              .setSchemaDescriptor(new TopicMethodDescriptorSupplier("Active"))
              .build();
        }
      }
    }
    return getActiveMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page,
      com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse> getUnansweredMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Unanswered",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.Page.class,
      responseType = com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page,
      com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse> getUnansweredMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page, com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse> getUnansweredMethod;
    if ((getUnansweredMethod = TopicGrpc.getUnansweredMethod) == null) {
      synchronized (TopicGrpc.class) {
        if ((getUnansweredMethod = TopicGrpc.getUnansweredMethod) == null) {
          TopicGrpc.getUnansweredMethod = getUnansweredMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.Page, com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Unanswered"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.Page.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse.getDefaultInstance()))
              .setSchemaDescriptor(new TopicMethodDescriptorSupplier("Unanswered"))
              .build();
        }
      }
    }
    return getUnansweredMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicCreateRequest,
      com.google.protobuf.Empty> getCreateMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Create",
      requestType = com.github.saturn_xiv.palm.plugins.bbs.v1.TopicCreateRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicCreateRequest,
      com.google.protobuf.Empty> getCreateMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicCreateRequest, com.google.protobuf.Empty> getCreateMethod;
    if ((getCreateMethod = TopicGrpc.getCreateMethod) == null) {
      synchronized (TopicGrpc.class) {
        if ((getCreateMethod = TopicGrpc.getCreateMethod) == null) {
          TopicGrpc.getCreateMethod = getCreateMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicCreateRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Create"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.bbs.v1.TopicCreateRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new TopicMethodDescriptorSupplier("Create"))
              .build();
        }
      }
    }
    return getCreateMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicUpdateRequest,
      com.google.protobuf.Empty> getUpdateMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Update",
      requestType = com.github.saturn_xiv.palm.plugins.bbs.v1.TopicUpdateRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicUpdateRequest,
      com.google.protobuf.Empty> getUpdateMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicUpdateRequest, com.google.protobuf.Empty> getUpdateMethod;
    if ((getUpdateMethod = TopicGrpc.getUpdateMethod) == null) {
      synchronized (TopicGrpc.class) {
        if ((getUpdateMethod = TopicGrpc.getUpdateMethod) == null) {
          TopicGrpc.getUpdateMethod = getUpdateMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicUpdateRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Update"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.bbs.v1.TopicUpdateRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new TopicMethodDescriptorSupplier("Update"))
              .build();
        }
      }
    }
    return getUpdateMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest,
      com.google.protobuf.Empty> getLockMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Lock",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest,
      com.google.protobuf.Empty> getLockMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest, com.google.protobuf.Empty> getLockMethod;
    if ((getLockMethod = TopicGrpc.getLockMethod) == null) {
      synchronized (TopicGrpc.class) {
        if ((getLockMethod = TopicGrpc.getLockMethod) == null) {
          TopicGrpc.getLockMethod = getLockMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Lock"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new TopicMethodDescriptorSupplier("Lock"))
              .build();
        }
      }
    }
    return getLockMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest,
      com.google.protobuf.Empty> getUnlockMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Unlock",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest,
      com.google.protobuf.Empty> getUnlockMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest, com.google.protobuf.Empty> getUnlockMethod;
    if ((getUnlockMethod = TopicGrpc.getUnlockMethod) == null) {
      synchronized (TopicGrpc.class) {
        if ((getUnlockMethod = TopicGrpc.getUnlockMethod) == null) {
          TopicGrpc.getUnlockMethod = getUnlockMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Unlock"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new TopicMethodDescriptorSupplier("Unlock"))
              .build();
        }
      }
    }
    return getUnlockMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest,
      com.google.protobuf.Empty> getDeleteMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Delete",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest,
      com.google.protobuf.Empty> getDeleteMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest, com.google.protobuf.Empty> getDeleteMethod;
    if ((getDeleteMethod = TopicGrpc.getDeleteMethod) == null) {
      synchronized (TopicGrpc.class) {
        if ((getDeleteMethod = TopicGrpc.getDeleteMethod) == null) {
          TopicGrpc.getDeleteMethod = getDeleteMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Delete"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new TopicMethodDescriptorSupplier("Delete"))
              .build();
        }
      }
    }
    return getDeleteMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static TopicStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<TopicStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<TopicStub>() {
        @java.lang.Override
        public TopicStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new TopicStub(channel, callOptions);
        }
      };
    return TopicStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static TopicBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<TopicBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<TopicBlockingV2Stub>() {
        @java.lang.Override
        public TopicBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new TopicBlockingV2Stub(channel, callOptions);
        }
      };
    return TopicBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static TopicBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<TopicBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<TopicBlockingStub>() {
        @java.lang.Override
        public TopicBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new TopicBlockingStub(channel, callOptions);
        }
      };
    return TopicBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static TopicFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<TopicFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<TopicFutureStub>() {
        @java.lang.Override
        public TopicFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new TopicFutureStub(channel, callOptions);
        }
      };
    return TopicFutureStub.newStub(factory, channel);
  }

  /**
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public interface AsyncService {

    /**
     */
    default void index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getIndexMethod(), responseObserver);
    }

    /**
     */
    default void byForum(com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByForumRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getByForumMethod(), responseObserver);
    }

    /**
     */
    default void byUser(com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByUserRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getByUserMethod(), responseObserver);
    }

    /**
     */
    default void show(com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicShowResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getShowMethod(), responseObserver);
    }

    /**
     */
    default void active(com.github.saturn_xiv.palm.plugins.portal.v1.Page request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getActiveMethod(), responseObserver);
    }

    /**
     */
    default void unanswered(com.github.saturn_xiv.palm.plugins.portal.v1.Page request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getUnansweredMethod(), responseObserver);
    }

    /**
     */
    default void create(com.github.saturn_xiv.palm.plugins.bbs.v1.TopicCreateRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateMethod(), responseObserver);
    }

    /**
     */
    default void update(com.github.saturn_xiv.palm.plugins.bbs.v1.TopicUpdateRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getUpdateMethod(), responseObserver);
    }

    /**
     */
    default void lock(com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getLockMethod(), responseObserver);
    }

    /**
     */
    default void unlock(com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getUnlockMethod(), responseObserver);
    }

    /**
     */
    default void delete(com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeleteMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service Topic.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static abstract class TopicImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return TopicGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Topic.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class TopicStub
      extends io.grpc.stub.AbstractAsyncStub<TopicStub> {
    private TopicStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected TopicStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new TopicStub(channel, callOptions);
    }

    /**
     */
    public void index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void byForum(com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByForumRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getByForumMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void byUser(com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByUserRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getByUserMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void show(com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicShowResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getShowMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void active(com.github.saturn_xiv.palm.plugins.portal.v1.Page request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getActiveMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void unanswered(com.github.saturn_xiv.palm.plugins.portal.v1.Page request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getUnansweredMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void create(com.github.saturn_xiv.palm.plugins.bbs.v1.TopicCreateRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void update(com.github.saturn_xiv.palm.plugins.bbs.v1.TopicUpdateRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getUpdateMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void lock(com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getLockMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void unlock(com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getUnlockMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void delete(com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeleteMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Topic.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class TopicBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<TopicBlockingV2Stub> {
    private TopicBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected TopicBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new TopicBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getIndexMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse byForum(com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByForumRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getByForumMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse byUser(com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByUserRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getByUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.bbs.v1.TopicShowResponse show(com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getShowMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse active(com.github.saturn_xiv.palm.plugins.portal.v1.Page request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getActiveMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse unanswered(com.github.saturn_xiv.palm.plugins.portal.v1.Page request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getUnansweredMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty create(com.github.saturn_xiv.palm.plugins.bbs.v1.TopicCreateRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty update(com.github.saturn_xiv.palm.plugins.bbs.v1.TopicUpdateRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getUpdateMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty lock(com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getLockMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty unlock(com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getUnlockMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty delete(com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getDeleteMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service Topic.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class TopicBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<TopicBlockingStub> {
    private TopicBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected TopicBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new TopicBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getIndexMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse byForum(com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByForumRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getByForumMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse byUser(com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByUserRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getByUserMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.bbs.v1.TopicShowResponse show(com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getShowMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse active(com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getActiveMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse unanswered(com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getUnansweredMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty create(com.github.saturn_xiv.palm.plugins.bbs.v1.TopicCreateRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty update(com.github.saturn_xiv.palm.plugins.bbs.v1.TopicUpdateRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getUpdateMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty lock(com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getLockMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty unlock(com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getUnlockMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty delete(com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeleteMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Topic.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class TopicFutureStub
      extends io.grpc.stub.AbstractFutureStub<TopicFutureStub> {
    private TopicFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected TopicFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new TopicFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse> index(
        com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse> byForum(
        com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByForumRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getByForumMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse> byUser(
        com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByUserRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getByUserMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicShowResponse> show(
        com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getShowMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse> active(
        com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getActiveMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse> unanswered(
        com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getUnansweredMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> create(
        com.github.saturn_xiv.palm.plugins.bbs.v1.TopicCreateRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> update(
        com.github.saturn_xiv.palm.plugins.bbs.v1.TopicUpdateRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getUpdateMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> lock(
        com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getLockMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> unlock(
        com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getUnlockMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> delete(
        com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeleteMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_INDEX = 0;
  private static final int METHODID_BY_FORUM = 1;
  private static final int METHODID_BY_USER = 2;
  private static final int METHODID_SHOW = 3;
  private static final int METHODID_ACTIVE = 4;
  private static final int METHODID_UNANSWERED = 5;
  private static final int METHODID_CREATE = 6;
  private static final int METHODID_UPDATE = 7;
  private static final int METHODID_LOCK = 8;
  private static final int METHODID_UNLOCK = 9;
  private static final int METHODID_DELETE = 10;

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
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse>) responseObserver);
          break;
        case METHODID_BY_FORUM:
          serviceImpl.byForum((com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByForumRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse>) responseObserver);
          break;
        case METHODID_BY_USER:
          serviceImpl.byUser((com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByUserRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse>) responseObserver);
          break;
        case METHODID_SHOW:
          serviceImpl.show((com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicShowResponse>) responseObserver);
          break;
        case METHODID_ACTIVE:
          serviceImpl.active((com.github.saturn_xiv.palm.plugins.portal.v1.Page) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse>) responseObserver);
          break;
        case METHODID_UNANSWERED:
          serviceImpl.unanswered((com.github.saturn_xiv.palm.plugins.portal.v1.Page) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse>) responseObserver);
          break;
        case METHODID_CREATE:
          serviceImpl.create((com.github.saturn_xiv.palm.plugins.bbs.v1.TopicCreateRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_UPDATE:
          serviceImpl.update((com.github.saturn_xiv.palm.plugins.bbs.v1.TopicUpdateRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_LOCK:
          serviceImpl.lock((com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_UNLOCK:
          serviceImpl.unlock((com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_DELETE:
          serviceImpl.delete((com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest) request,
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
              com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse>(
                service, METHODID_INDEX)))
        .addMethod(
          getByForumMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByForumRequest,
              com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse>(
                service, METHODID_BY_FORUM)))
        .addMethod(
          getByUserMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.bbs.v1.TopicByUserRequest,
              com.github.saturn_xiv.palm.plugins.bbs.v1.PostIndexResponse>(
                service, METHODID_BY_USER)))
        .addMethod(
          getShowMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.ByIdRequest,
              com.github.saturn_xiv.palm.plugins.bbs.v1.TopicShowResponse>(
                service, METHODID_SHOW)))
        .addMethod(
          getActiveMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.Page,
              com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse>(
                service, METHODID_ACTIVE)))
        .addMethod(
          getUnansweredMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.Page,
              com.github.saturn_xiv.palm.plugins.bbs.v1.TopicIndexResponse>(
                service, METHODID_UNANSWERED)))
        .addMethod(
          getCreateMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.bbs.v1.TopicCreateRequest,
              com.google.protobuf.Empty>(
                service, METHODID_CREATE)))
        .addMethod(
          getUpdateMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.bbs.v1.TopicUpdateRequest,
              com.google.protobuf.Empty>(
                service, METHODID_UPDATE)))
        .addMethod(
          getLockMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest,
              com.google.protobuf.Empty>(
                service, METHODID_LOCK)))
        .addMethod(
          getUnlockMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest,
              com.google.protobuf.Empty>(
                service, METHODID_UNLOCK)))
        .addMethod(
          getDeleteMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.IdWithReasonRequest,
              com.google.protobuf.Empty>(
                service, METHODID_DELETE)))
        .build();
  }

  private static abstract class TopicBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    TopicBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.bbs.v1.Bbs.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Topic");
    }
  }

  private static final class TopicFileDescriptorSupplier
      extends TopicBaseDescriptorSupplier {
    TopicFileDescriptorSupplier() {}
  }

  private static final class TopicMethodDescriptorSupplier
      extends TopicBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    TopicMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (TopicGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new TopicFileDescriptorSupplier())
              .addMethod(getIndexMethod())
              .addMethod(getByForumMethod())
              .addMethod(getByUserMethod())
              .addMethod(getShowMethod())
              .addMethod(getActiveMethod())
              .addMethod(getUnansweredMethod())
              .addMethod(getCreateMethod())
              .addMethod(getUpdateMethod())
              .addMethod(getLockMethod())
              .addMethod(getUnlockMethod())
              .addMethod(getDeleteMethod())
              .build();
        }
      }
    }
    return result;
  }
}
