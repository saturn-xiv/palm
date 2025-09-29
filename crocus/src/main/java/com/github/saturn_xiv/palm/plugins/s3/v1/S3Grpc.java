package com.github.saturn_xiv.palm.plugins.s3.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class S3Grpc {

  private S3Grpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.s3.v1.S3";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketsResponse> getListBucketsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListBuckets",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketsResponse> getListBucketsMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketsResponse> getListBucketsMethod;
    if ((getListBucketsMethod = S3Grpc.getListBucketsMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getListBucketsMethod = S3Grpc.getListBucketsMethod) == null) {
          S3Grpc.getListBucketsMethod = getListBucketsMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListBuckets"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("ListBuckets"))
              .build();
        }
      }
    }
    return getListBucketsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse> getBucketExistsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "BucketExists",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse> getBucketExistsMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest, com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse> getBucketExistsMethod;
    if ((getBucketExistsMethod = S3Grpc.getBucketExistsMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getBucketExistsMethod = S3Grpc.getBucketExistsMethod) == null) {
          S3Grpc.getBucketExistsMethod = getBucketExistsMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest, com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "BucketExists"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("BucketExists"))
              .build();
        }
      }
    }
    return getBucketExistsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketEncryptionResponse> getGetBucketEncryptionMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetBucketEncryption",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketEncryptionResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketEncryptionResponse> getGetBucketEncryptionMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest, com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketEncryptionResponse> getGetBucketEncryptionMethod;
    if ((getGetBucketEncryptionMethod = S3Grpc.getGetBucketEncryptionMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getGetBucketEncryptionMethod = S3Grpc.getGetBucketEncryptionMethod) == null) {
          S3Grpc.getGetBucketEncryptionMethod = getGetBucketEncryptionMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest, com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketEncryptionResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetBucketEncryption"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketEncryptionResponse.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("GetBucketEncryption"))
              .build();
        }
      }
    }
    return getGetBucketEncryptionMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketPolicyResponse> getGetBucketPolicyMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetBucketPolicy",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketPolicyResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketPolicyResponse> getGetBucketPolicyMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest, com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketPolicyResponse> getGetBucketPolicyMethod;
    if ((getGetBucketPolicyMethod = S3Grpc.getGetBucketPolicyMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getGetBucketPolicyMethod = S3Grpc.getGetBucketPolicyMethod) == null) {
          S3Grpc.getGetBucketPolicyMethod = getGetBucketPolicyMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest, com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketPolicyResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetBucketPolicy"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketPolicyResponse.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("GetBucketPolicy"))
              .build();
        }
      }
    }
    return getGetBucketPolicyMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketTagsResponse> getGetBucketTagsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetBucketTags",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketTagsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketTagsResponse> getGetBucketTagsMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest, com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketTagsResponse> getGetBucketTagsMethod;
    if ((getGetBucketTagsMethod = S3Grpc.getGetBucketTagsMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getGetBucketTagsMethod = S3Grpc.getGetBucketTagsMethod) == null) {
          S3Grpc.getGetBucketTagsMethod = getGetBucketTagsMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest, com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketTagsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetBucketTags"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketTagsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("GetBucketTags"))
              .build();
        }
      }
    }
    return getGetBucketTagsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketLifecycleResponse> getGetBucketLifecycleMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetBucketLifecycle",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketLifecycleResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketLifecycleResponse> getGetBucketLifecycleMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest, com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketLifecycleResponse> getGetBucketLifecycleMethod;
    if ((getGetBucketLifecycleMethod = S3Grpc.getGetBucketLifecycleMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getGetBucketLifecycleMethod = S3Grpc.getGetBucketLifecycleMethod) == null) {
          S3Grpc.getGetBucketLifecycleMethod = getGetBucketLifecycleMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest, com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketLifecycleResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetBucketLifecycle"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketLifecycleResponse.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("GetBucketLifecycle"))
              .build();
        }
      }
    }
    return getGetBucketLifecycleMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest,
      com.google.protobuf.Empty> getMakeBucketMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "MakeBucket",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest,
      com.google.protobuf.Empty> getMakeBucketMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest, com.google.protobuf.Empty> getMakeBucketMethod;
    if ((getMakeBucketMethod = S3Grpc.getMakeBucketMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getMakeBucketMethod = S3Grpc.getMakeBucketMethod) == null) {
          S3Grpc.getMakeBucketMethod = getMakeBucketMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "MakeBucket"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("MakeBucket"))
              .build();
        }
      }
    }
    return getMakeBucketMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
      com.google.protobuf.Empty> getRemoveBucketMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RemoveBucket",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
      com.google.protobuf.Empty> getRemoveBucketMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest, com.google.protobuf.Empty> getRemoveBucketMethod;
    if ((getRemoveBucketMethod = S3Grpc.getRemoveBucketMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getRemoveBucketMethod = S3Grpc.getRemoveBucketMethod) == null) {
          S3Grpc.getRemoveBucketMethod = getRemoveBucketMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RemoveBucket"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("RemoveBucket"))
              .build();
        }
      }
    }
    return getRemoveBucketMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketTagsRequest,
      com.google.protobuf.Empty> getSetBucketTagsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SetBucketTags",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketTagsRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketTagsRequest,
      com.google.protobuf.Empty> getSetBucketTagsMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketTagsRequest, com.google.protobuf.Empty> getSetBucketTagsMethod;
    if ((getSetBucketTagsMethod = S3Grpc.getSetBucketTagsMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getSetBucketTagsMethod = S3Grpc.getSetBucketTagsMethod) == null) {
          S3Grpc.getSetBucketTagsMethod = getSetBucketTagsMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketTagsRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SetBucketTags"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketTagsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("SetBucketTags"))
              .build();
        }
      }
    }
    return getSetBucketTagsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketPolicyRequest,
      com.google.protobuf.Empty> getSetBucketPolicyMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SetBucketPolicy",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketPolicyRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketPolicyRequest,
      com.google.protobuf.Empty> getSetBucketPolicyMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketPolicyRequest, com.google.protobuf.Empty> getSetBucketPolicyMethod;
    if ((getSetBucketPolicyMethod = S3Grpc.getSetBucketPolicyMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getSetBucketPolicyMethod = S3Grpc.getSetBucketPolicyMethod) == null) {
          S3Grpc.getSetBucketPolicyMethod = getSetBucketPolicyMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketPolicyRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SetBucketPolicy"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketPolicyRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("SetBucketPolicy"))
              .build();
        }
      }
    }
    return getSetBucketPolicyMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketLifecycleRequest,
      com.google.protobuf.Empty> getSetBucketLifecycleMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SetBucketLifecycle",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketLifecycleRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketLifecycleRequest,
      com.google.protobuf.Empty> getSetBucketLifecycleMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketLifecycleRequest, com.google.protobuf.Empty> getSetBucketLifecycleMethod;
    if ((getSetBucketLifecycleMethod = S3Grpc.getSetBucketLifecycleMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getSetBucketLifecycleMethod = S3Grpc.getSetBucketLifecycleMethod) == null) {
          S3Grpc.getSetBucketLifecycleMethod = getSetBucketLifecycleMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketLifecycleRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SetBucketLifecycle"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketLifecycleRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("SetBucketLifecycle"))
              .build();
        }
      }
    }
    return getSetBucketLifecycleMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketEncryptionRequest,
      com.google.protobuf.Empty> getSetBucketEncryptionMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SetBucketEncryption",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketEncryptionRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketEncryptionRequest,
      com.google.protobuf.Empty> getSetBucketEncryptionMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketEncryptionRequest, com.google.protobuf.Empty> getSetBucketEncryptionMethod;
    if ((getSetBucketEncryptionMethod = S3Grpc.getSetBucketEncryptionMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getSetBucketEncryptionMethod = S3Grpc.getSetBucketEncryptionMethod) == null) {
          S3Grpc.getSetBucketEncryptionMethod = getSetBucketEncryptionMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketEncryptionRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SetBucketEncryption"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketEncryptionRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("SetBucketEncryption"))
              .build();
        }
      }
    }
    return getSetBucketEncryptionMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
      com.google.protobuf.Empty> getDeleteBucketEncryptionMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeleteBucketEncryption",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
      com.google.protobuf.Empty> getDeleteBucketEncryptionMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest, com.google.protobuf.Empty> getDeleteBucketEncryptionMethod;
    if ((getDeleteBucketEncryptionMethod = S3Grpc.getDeleteBucketEncryptionMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getDeleteBucketEncryptionMethod = S3Grpc.getDeleteBucketEncryptionMethod) == null) {
          S3Grpc.getDeleteBucketEncryptionMethod = getDeleteBucketEncryptionMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeleteBucketEncryption"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("DeleteBucketEncryption"))
              .build();
        }
      }
    }
    return getDeleteBucketEncryptionMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
      com.google.protobuf.Empty> getDeleteBucketPolicyMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeleteBucketPolicy",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
      com.google.protobuf.Empty> getDeleteBucketPolicyMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest, com.google.protobuf.Empty> getDeleteBucketPolicyMethod;
    if ((getDeleteBucketPolicyMethod = S3Grpc.getDeleteBucketPolicyMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getDeleteBucketPolicyMethod = S3Grpc.getDeleteBucketPolicyMethod) == null) {
          S3Grpc.getDeleteBucketPolicyMethod = getDeleteBucketPolicyMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeleteBucketPolicy"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("DeleteBucketPolicy"))
              .build();
        }
      }
    }
    return getDeleteBucketPolicyMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
      com.google.protobuf.Empty> getDeleteBucketTagsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeleteBucketTags",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
      com.google.protobuf.Empty> getDeleteBucketTagsMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest, com.google.protobuf.Empty> getDeleteBucketTagsMethod;
    if ((getDeleteBucketTagsMethod = S3Grpc.getDeleteBucketTagsMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getDeleteBucketTagsMethod = S3Grpc.getDeleteBucketTagsMethod) == null) {
          S3Grpc.getDeleteBucketTagsMethod = getDeleteBucketTagsMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeleteBucketTags"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("DeleteBucketTags"))
              .build();
        }
      }
    }
    return getDeleteBucketTagsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
      com.google.protobuf.Empty> getDeleteBucketLifecycleMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeleteBucketLifecycle",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
      com.google.protobuf.Empty> getDeleteBucketLifecycleMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest, com.google.protobuf.Empty> getDeleteBucketLifecycleMethod;
    if ((getDeleteBucketLifecycleMethod = S3Grpc.getDeleteBucketLifecycleMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getDeleteBucketLifecycleMethod = S3Grpc.getDeleteBucketLifecycleMethod) == null) {
          S3Grpc.getDeleteBucketLifecycleMethod = getDeleteBucketLifecycleMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeleteBucketLifecycle"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("DeleteBucketLifecycle"))
              .build();
        }
      }
    }
    return getDeleteBucketLifecycleMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataResponse> getGetPresignedPostFormDataMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetPresignedPostFormData",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataResponse> getGetPresignedPostFormDataMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataRequest, com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataResponse> getGetPresignedPostFormDataMethod;
    if ((getGetPresignedPostFormDataMethod = S3Grpc.getGetPresignedPostFormDataMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getGetPresignedPostFormDataMethod = S3Grpc.getGetPresignedPostFormDataMethod) == null) {
          S3Grpc.getGetPresignedPostFormDataMethod = getGetPresignedPostFormDataMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataRequest, com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetPresignedPostFormData"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataResponse.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("GetPresignedPostFormData"))
              .build();
        }
      }
    }
    return getGetPresignedPostFormDataMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlResponse> getGetPresignedObjectUrlMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetPresignedObjectUrl",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlResponse> getGetPresignedObjectUrlMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlRequest, com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlResponse> getGetPresignedObjectUrlMethod;
    if ((getGetPresignedObjectUrlMethod = S3Grpc.getGetPresignedObjectUrlMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getGetPresignedObjectUrlMethod = S3Grpc.getGetPresignedObjectUrlMethod) == null) {
          S3Grpc.getGetPresignedObjectUrlMethod = getGetPresignedObjectUrlMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlRequest, com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetPresignedObjectUrl"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlResponse.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("GetPresignedObjectUrl"))
              .build();
        }
      }
    }
    return getGetPresignedObjectUrlMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.ListObjectsResponse> getListObjectsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListObjects",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.s3.v1.ListObjectsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.ListObjectsResponse> getListObjectsMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest, com.github.saturn_xiv.palm.plugins.s3.v1.ListObjectsResponse> getListObjectsMethod;
    if ((getListObjectsMethod = S3Grpc.getListObjectsMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getListObjectsMethod = S3Grpc.getListObjectsMethod) == null) {
          S3Grpc.getListObjectsMethod = getListObjectsMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest, com.github.saturn_xiv.palm.plugins.s3.v1.ListObjectsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListObjects"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.ListObjectsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("ListObjects"))
              .build();
        }
      }
    }
    return getListObjectsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectTagsResponse> getGetObjectTagsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetObjectTags",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectTagsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectTagsResponse> getGetObjectTagsMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest, com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectTagsResponse> getGetObjectTagsMethod;
    if ((getGetObjectTagsMethod = S3Grpc.getGetObjectTagsMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getGetObjectTagsMethod = S3Grpc.getGetObjectTagsMethod) == null) {
          S3Grpc.getGetObjectTagsMethod = getGetObjectTagsMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest, com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectTagsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetObjectTags"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectTagsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("GetObjectTags"))
              .build();
        }
      }
    }
    return getGetObjectTagsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectRetentionResponse> getGetObjectRetentionMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetObjectRetention",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectRetentionResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectRetentionResponse> getGetObjectRetentionMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest, com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectRetentionResponse> getGetObjectRetentionMethod;
    if ((getGetObjectRetentionMethod = S3Grpc.getGetObjectRetentionMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getGetObjectRetentionMethod = S3Grpc.getGetObjectRetentionMethod) == null) {
          S3Grpc.getGetObjectRetentionMethod = getGetObjectRetentionMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest, com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectRetentionResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetObjectRetention"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectRetentionResponse.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("GetObjectRetention"))
              .build();
        }
      }
    }
    return getGetObjectRetentionMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.StatObjectResponse> getStatObjectMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "StatObject",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.s3.v1.StatObjectResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest,
      com.github.saturn_xiv.palm.plugins.s3.v1.StatObjectResponse> getStatObjectMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest, com.github.saturn_xiv.palm.plugins.s3.v1.StatObjectResponse> getStatObjectMethod;
    if ((getStatObjectMethod = S3Grpc.getStatObjectMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getStatObjectMethod = S3Grpc.getStatObjectMethod) == null) {
          S3Grpc.getStatObjectMethod = getStatObjectMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest, com.github.saturn_xiv.palm.plugins.s3.v1.StatObjectResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "StatObject"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.StatObjectResponse.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("StatObject"))
              .build();
        }
      }
    }
    return getStatObjectMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.SetObjectTagsRequest,
      com.google.protobuf.Empty> getSetObjectTagsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SetObjectTags",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.SetObjectTagsRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.SetObjectTagsRequest,
      com.google.protobuf.Empty> getSetObjectTagsMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.SetObjectTagsRequest, com.google.protobuf.Empty> getSetObjectTagsMethod;
    if ((getSetObjectTagsMethod = S3Grpc.getSetObjectTagsMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getSetObjectTagsMethod = S3Grpc.getSetObjectTagsMethod) == null) {
          S3Grpc.getSetObjectTagsMethod = getSetObjectTagsMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.SetObjectTagsRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SetObjectTags"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.SetObjectTagsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("SetObjectTags"))
              .build();
        }
      }
    }
    return getSetObjectTagsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.DeleteObjectsRequest,
      com.google.protobuf.Empty> getDeleteObjectsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeleteObjects",
      requestType = com.github.saturn_xiv.palm.plugins.s3.v1.DeleteObjectsRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.DeleteObjectsRequest,
      com.google.protobuf.Empty> getDeleteObjectsMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.s3.v1.DeleteObjectsRequest, com.google.protobuf.Empty> getDeleteObjectsMethod;
    if ((getDeleteObjectsMethod = S3Grpc.getDeleteObjectsMethod) == null) {
      synchronized (S3Grpc.class) {
        if ((getDeleteObjectsMethod = S3Grpc.getDeleteObjectsMethod) == null) {
          S3Grpc.getDeleteObjectsMethod = getDeleteObjectsMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.s3.v1.DeleteObjectsRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeleteObjects"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.s3.v1.DeleteObjectsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new S3MethodDescriptorSupplier("DeleteObjects"))
              .build();
        }
      }
    }
    return getDeleteObjectsMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static S3Stub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<S3Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<S3Stub>() {
        @java.lang.Override
        public S3Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new S3Stub(channel, callOptions);
        }
      };
    return S3Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static S3BlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<S3BlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<S3BlockingV2Stub>() {
        @java.lang.Override
        public S3BlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new S3BlockingV2Stub(channel, callOptions);
        }
      };
    return S3BlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static S3BlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<S3BlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<S3BlockingStub>() {
        @java.lang.Override
        public S3BlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new S3BlockingStub(channel, callOptions);
        }
      };
    return S3BlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static S3FutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<S3FutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<S3FutureStub>() {
        @java.lang.Override
        public S3FutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new S3FutureStub(channel, callOptions);
        }
      };
    return S3FutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void listBuckets(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListBucketsMethod(), responseObserver);
    }

    /**
     */
    default void bucketExists(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getBucketExistsMethod(), responseObserver);
    }

    /**
     */
    default void getBucketEncryption(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketEncryptionResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetBucketEncryptionMethod(), responseObserver);
    }

    /**
     */
    default void getBucketPolicy(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketPolicyResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetBucketPolicyMethod(), responseObserver);
    }

    /**
     */
    default void getBucketTags(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketTagsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetBucketTagsMethod(), responseObserver);
    }

    /**
     */
    default void getBucketLifecycle(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketLifecycleResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetBucketLifecycleMethod(), responseObserver);
    }

    /**
     */
    default void makeBucket(com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getMakeBucketMethod(), responseObserver);
    }

    /**
     */
    default void removeBucket(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRemoveBucketMethod(), responseObserver);
    }

    /**
     */
    default void setBucketTags(com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketTagsRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSetBucketTagsMethod(), responseObserver);
    }

    /**
     */
    default void setBucketPolicy(com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketPolicyRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSetBucketPolicyMethod(), responseObserver);
    }

    /**
     */
    default void setBucketLifecycle(com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketLifecycleRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSetBucketLifecycleMethod(), responseObserver);
    }

    /**
     */
    default void setBucketEncryption(com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketEncryptionRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSetBucketEncryptionMethod(), responseObserver);
    }

    /**
     */
    default void deleteBucketEncryption(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeleteBucketEncryptionMethod(), responseObserver);
    }

    /**
     */
    default void deleteBucketPolicy(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeleteBucketPolicyMethod(), responseObserver);
    }

    /**
     */
    default void deleteBucketTags(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeleteBucketTagsMethod(), responseObserver);
    }

    /**
     */
    default void deleteBucketLifecycle(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeleteBucketLifecycleMethod(), responseObserver);
    }

    /**
     */
    default void getPresignedPostFormData(com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetPresignedPostFormDataMethod(), responseObserver);
    }

    /**
     */
    default void getPresignedObjectUrl(com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetPresignedObjectUrlMethod(), responseObserver);
    }

    /**
     */
    default void listObjects(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.ListObjectsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListObjectsMethod(), responseObserver);
    }

    /**
     */
    default void getObjectTags(com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectTagsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetObjectTagsMethod(), responseObserver);
    }

    /**
     */
    default void getObjectRetention(com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectRetentionResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetObjectRetentionMethod(), responseObserver);
    }

    /**
     */
    default void statObject(com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.StatObjectResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getStatObjectMethod(), responseObserver);
    }

    /**
     */
    default void setObjectTags(com.github.saturn_xiv.palm.plugins.s3.v1.SetObjectTagsRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSetObjectTagsMethod(), responseObserver);
    }

    /**
     */
    default void deleteObjects(com.github.saturn_xiv.palm.plugins.s3.v1.DeleteObjectsRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeleteObjectsMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service S3.
   */
  public static abstract class S3ImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return S3Grpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service S3.
   */
  public static final class S3Stub
      extends io.grpc.stub.AbstractAsyncStub<S3Stub> {
    private S3Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected S3Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new S3Stub(channel, callOptions);
    }

    /**
     */
    public void listBuckets(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListBucketsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void bucketExists(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getBucketExistsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getBucketEncryption(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketEncryptionResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetBucketEncryptionMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getBucketPolicy(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketPolicyResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetBucketPolicyMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getBucketTags(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketTagsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetBucketTagsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getBucketLifecycle(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketLifecycleResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetBucketLifecycleMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void makeBucket(com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getMakeBucketMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void removeBucket(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRemoveBucketMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void setBucketTags(com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketTagsRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSetBucketTagsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void setBucketPolicy(com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketPolicyRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSetBucketPolicyMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void setBucketLifecycle(com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketLifecycleRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSetBucketLifecycleMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void setBucketEncryption(com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketEncryptionRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSetBucketEncryptionMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void deleteBucketEncryption(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeleteBucketEncryptionMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void deleteBucketPolicy(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeleteBucketPolicyMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void deleteBucketTags(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeleteBucketTagsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void deleteBucketLifecycle(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeleteBucketLifecycleMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getPresignedPostFormData(com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetPresignedPostFormDataMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getPresignedObjectUrl(com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetPresignedObjectUrlMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void listObjects(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.ListObjectsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListObjectsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getObjectTags(com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectTagsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetObjectTagsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getObjectRetention(com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectRetentionResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetObjectRetentionMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void statObject(com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.StatObjectResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getStatObjectMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void setObjectTags(com.github.saturn_xiv.palm.plugins.s3.v1.SetObjectTagsRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSetObjectTagsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void deleteObjects(com.github.saturn_xiv.palm.plugins.s3.v1.DeleteObjectsRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeleteObjectsMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service S3.
   */
  public static final class S3BlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<S3BlockingV2Stub> {
    private S3BlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected S3BlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new S3BlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketsResponse listBuckets(com.google.protobuf.Empty request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListBucketsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse bucketExists(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getBucketExistsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketEncryptionResponse getBucketEncryption(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getGetBucketEncryptionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketPolicyResponse getBucketPolicy(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getGetBucketPolicyMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketTagsResponse getBucketTags(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getGetBucketTagsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketLifecycleResponse getBucketLifecycle(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getGetBucketLifecycleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty makeBucket(com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getMakeBucketMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty removeBucket(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRemoveBucketMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setBucketTags(com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketTagsRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getSetBucketTagsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setBucketPolicy(com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketPolicyRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getSetBucketPolicyMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setBucketLifecycle(com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketLifecycleRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getSetBucketLifecycleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setBucketEncryption(com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketEncryptionRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getSetBucketEncryptionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deleteBucketEncryption(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getDeleteBucketEncryptionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deleteBucketPolicy(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getDeleteBucketPolicyMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deleteBucketTags(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getDeleteBucketTagsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deleteBucketLifecycle(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getDeleteBucketLifecycleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataResponse getPresignedPostFormData(com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getGetPresignedPostFormDataMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlResponse getPresignedObjectUrl(com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getGetPresignedObjectUrlMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.ListObjectsResponse listObjects(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListObjectsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectTagsResponse getObjectTags(com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getGetObjectTagsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectRetentionResponse getObjectRetention(com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getGetObjectRetentionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.StatObjectResponse statObject(com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getStatObjectMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setObjectTags(com.github.saturn_xiv.palm.plugins.s3.v1.SetObjectTagsRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getSetObjectTagsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deleteObjects(com.github.saturn_xiv.palm.plugins.s3.v1.DeleteObjectsRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getDeleteObjectsMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service S3.
   */
  public static final class S3BlockingStub
      extends io.grpc.stub.AbstractBlockingStub<S3BlockingStub> {
    private S3BlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected S3BlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new S3BlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketsResponse listBuckets(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListBucketsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse bucketExists(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getBucketExistsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketEncryptionResponse getBucketEncryption(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetBucketEncryptionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketPolicyResponse getBucketPolicy(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetBucketPolicyMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketTagsResponse getBucketTags(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetBucketTagsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketLifecycleResponse getBucketLifecycle(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetBucketLifecycleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty makeBucket(com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getMakeBucketMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty removeBucket(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRemoveBucketMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setBucketTags(com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketTagsRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetBucketTagsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setBucketPolicy(com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketPolicyRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetBucketPolicyMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setBucketLifecycle(com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketLifecycleRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetBucketLifecycleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setBucketEncryption(com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketEncryptionRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetBucketEncryptionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deleteBucketEncryption(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeleteBucketEncryptionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deleteBucketPolicy(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeleteBucketPolicyMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deleteBucketTags(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeleteBucketTagsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deleteBucketLifecycle(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeleteBucketLifecycleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataResponse getPresignedPostFormData(com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetPresignedPostFormDataMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlResponse getPresignedObjectUrl(com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetPresignedObjectUrlMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.ListObjectsResponse listObjects(com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListObjectsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectTagsResponse getObjectTags(com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetObjectTagsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectRetentionResponse getObjectRetention(com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetObjectRetentionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.s3.v1.StatObjectResponse statObject(com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getStatObjectMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setObjectTags(com.github.saturn_xiv.palm.plugins.s3.v1.SetObjectTagsRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetObjectTagsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deleteObjects(com.github.saturn_xiv.palm.plugins.s3.v1.DeleteObjectsRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeleteObjectsMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service S3.
   */
  public static final class S3FutureStub
      extends io.grpc.stub.AbstractFutureStub<S3FutureStub> {
    private S3FutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected S3FutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new S3FutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketsResponse> listBuckets(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListBucketsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse> bucketExists(
        com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getBucketExistsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketEncryptionResponse> getBucketEncryption(
        com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetBucketEncryptionMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketPolicyResponse> getBucketPolicy(
        com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetBucketPolicyMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketTagsResponse> getBucketTags(
        com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetBucketTagsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketLifecycleResponse> getBucketLifecycle(
        com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetBucketLifecycleMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> makeBucket(
        com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getMakeBucketMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> removeBucket(
        com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRemoveBucketMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> setBucketTags(
        com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketTagsRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSetBucketTagsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> setBucketPolicy(
        com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketPolicyRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSetBucketPolicyMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> setBucketLifecycle(
        com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketLifecycleRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSetBucketLifecycleMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> setBucketEncryption(
        com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketEncryptionRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSetBucketEncryptionMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> deleteBucketEncryption(
        com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeleteBucketEncryptionMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> deleteBucketPolicy(
        com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeleteBucketPolicyMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> deleteBucketTags(
        com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeleteBucketTagsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> deleteBucketLifecycle(
        com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeleteBucketLifecycleMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataResponse> getPresignedPostFormData(
        com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetPresignedPostFormDataMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlResponse> getPresignedObjectUrl(
        com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetPresignedObjectUrlMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.s3.v1.ListObjectsResponse> listObjects(
        com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListObjectsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectTagsResponse> getObjectTags(
        com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetObjectTagsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectRetentionResponse> getObjectRetention(
        com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetObjectRetentionMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.s3.v1.StatObjectResponse> statObject(
        com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getStatObjectMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> setObjectTags(
        com.github.saturn_xiv.palm.plugins.s3.v1.SetObjectTagsRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSetObjectTagsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> deleteObjects(
        com.github.saturn_xiv.palm.plugins.s3.v1.DeleteObjectsRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeleteObjectsMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_LIST_BUCKETS = 0;
  private static final int METHODID_BUCKET_EXISTS = 1;
  private static final int METHODID_GET_BUCKET_ENCRYPTION = 2;
  private static final int METHODID_GET_BUCKET_POLICY = 3;
  private static final int METHODID_GET_BUCKET_TAGS = 4;
  private static final int METHODID_GET_BUCKET_LIFECYCLE = 5;
  private static final int METHODID_MAKE_BUCKET = 6;
  private static final int METHODID_REMOVE_BUCKET = 7;
  private static final int METHODID_SET_BUCKET_TAGS = 8;
  private static final int METHODID_SET_BUCKET_POLICY = 9;
  private static final int METHODID_SET_BUCKET_LIFECYCLE = 10;
  private static final int METHODID_SET_BUCKET_ENCRYPTION = 11;
  private static final int METHODID_DELETE_BUCKET_ENCRYPTION = 12;
  private static final int METHODID_DELETE_BUCKET_POLICY = 13;
  private static final int METHODID_DELETE_BUCKET_TAGS = 14;
  private static final int METHODID_DELETE_BUCKET_LIFECYCLE = 15;
  private static final int METHODID_GET_PRESIGNED_POST_FORM_DATA = 16;
  private static final int METHODID_GET_PRESIGNED_OBJECT_URL = 17;
  private static final int METHODID_LIST_OBJECTS = 18;
  private static final int METHODID_GET_OBJECT_TAGS = 19;
  private static final int METHODID_GET_OBJECT_RETENTION = 20;
  private static final int METHODID_STAT_OBJECT = 21;
  private static final int METHODID_SET_OBJECT_TAGS = 22;
  private static final int METHODID_DELETE_OBJECTS = 23;

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
        case METHODID_LIST_BUCKETS:
          serviceImpl.listBuckets((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketsResponse>) responseObserver);
          break;
        case METHODID_BUCKET_EXISTS:
          serviceImpl.bucketExists((com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse>) responseObserver);
          break;
        case METHODID_GET_BUCKET_ENCRYPTION:
          serviceImpl.getBucketEncryption((com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketEncryptionResponse>) responseObserver);
          break;
        case METHODID_GET_BUCKET_POLICY:
          serviceImpl.getBucketPolicy((com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketPolicyResponse>) responseObserver);
          break;
        case METHODID_GET_BUCKET_TAGS:
          serviceImpl.getBucketTags((com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketTagsResponse>) responseObserver);
          break;
        case METHODID_GET_BUCKET_LIFECYCLE:
          serviceImpl.getBucketLifecycle((com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketLifecycleResponse>) responseObserver);
          break;
        case METHODID_MAKE_BUCKET:
          serviceImpl.makeBucket((com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_REMOVE_BUCKET:
          serviceImpl.removeBucket((com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_SET_BUCKET_TAGS:
          serviceImpl.setBucketTags((com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketTagsRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_SET_BUCKET_POLICY:
          serviceImpl.setBucketPolicy((com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketPolicyRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_SET_BUCKET_LIFECYCLE:
          serviceImpl.setBucketLifecycle((com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketLifecycleRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_SET_BUCKET_ENCRYPTION:
          serviceImpl.setBucketEncryption((com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketEncryptionRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_DELETE_BUCKET_ENCRYPTION:
          serviceImpl.deleteBucketEncryption((com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_DELETE_BUCKET_POLICY:
          serviceImpl.deleteBucketPolicy((com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_DELETE_BUCKET_TAGS:
          serviceImpl.deleteBucketTags((com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_DELETE_BUCKET_LIFECYCLE:
          serviceImpl.deleteBucketLifecycle((com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_GET_PRESIGNED_POST_FORM_DATA:
          serviceImpl.getPresignedPostFormData((com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataResponse>) responseObserver);
          break;
        case METHODID_GET_PRESIGNED_OBJECT_URL:
          serviceImpl.getPresignedObjectUrl((com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlResponse>) responseObserver);
          break;
        case METHODID_LIST_OBJECTS:
          serviceImpl.listObjects((com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.ListObjectsResponse>) responseObserver);
          break;
        case METHODID_GET_OBJECT_TAGS:
          serviceImpl.getObjectTags((com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectTagsResponse>) responseObserver);
          break;
        case METHODID_GET_OBJECT_RETENTION:
          serviceImpl.getObjectRetention((com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectRetentionResponse>) responseObserver);
          break;
        case METHODID_STAT_OBJECT:
          serviceImpl.statObject((com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.s3.v1.StatObjectResponse>) responseObserver);
          break;
        case METHODID_SET_OBJECT_TAGS:
          serviceImpl.setObjectTags((com.github.saturn_xiv.palm.plugins.s3.v1.SetObjectTagsRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_DELETE_OBJECTS:
          serviceImpl.deleteObjects((com.github.saturn_xiv.palm.plugins.s3.v1.DeleteObjectsRequest) request,
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
          getListBucketsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.s3.v1.ListBucketsResponse>(
                service, METHODID_LIST_BUCKETS)))
        .addMethod(
          getBucketExistsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
              com.github.saturn_xiv.palm.plugins.s3.v1.BucketExistsResponse>(
                service, METHODID_BUCKET_EXISTS)))
        .addMethod(
          getGetBucketEncryptionMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
              com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketEncryptionResponse>(
                service, METHODID_GET_BUCKET_ENCRYPTION)))
        .addMethod(
          getGetBucketPolicyMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
              com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketPolicyResponse>(
                service, METHODID_GET_BUCKET_POLICY)))
        .addMethod(
          getGetBucketTagsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
              com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketTagsResponse>(
                service, METHODID_GET_BUCKET_TAGS)))
        .addMethod(
          getGetBucketLifecycleMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
              com.github.saturn_xiv.palm.plugins.s3.v1.GetBucketLifecycleResponse>(
                service, METHODID_GET_BUCKET_LIFECYCLE)))
        .addMethod(
          getMakeBucketMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.MakeBucketRequest,
              com.google.protobuf.Empty>(
                service, METHODID_MAKE_BUCKET)))
        .addMethod(
          getRemoveBucketMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
              com.google.protobuf.Empty>(
                service, METHODID_REMOVE_BUCKET)))
        .addMethod(
          getSetBucketTagsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketTagsRequest,
              com.google.protobuf.Empty>(
                service, METHODID_SET_BUCKET_TAGS)))
        .addMethod(
          getSetBucketPolicyMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketPolicyRequest,
              com.google.protobuf.Empty>(
                service, METHODID_SET_BUCKET_POLICY)))
        .addMethod(
          getSetBucketLifecycleMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketLifecycleRequest,
              com.google.protobuf.Empty>(
                service, METHODID_SET_BUCKET_LIFECYCLE)))
        .addMethod(
          getSetBucketEncryptionMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.SetBucketEncryptionRequest,
              com.google.protobuf.Empty>(
                service, METHODID_SET_BUCKET_ENCRYPTION)))
        .addMethod(
          getDeleteBucketEncryptionMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
              com.google.protobuf.Empty>(
                service, METHODID_DELETE_BUCKET_ENCRYPTION)))
        .addMethod(
          getDeleteBucketPolicyMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
              com.google.protobuf.Empty>(
                service, METHODID_DELETE_BUCKET_POLICY)))
        .addMethod(
          getDeleteBucketTagsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
              com.google.protobuf.Empty>(
                service, METHODID_DELETE_BUCKET_TAGS)))
        .addMethod(
          getDeleteBucketLifecycleMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
              com.google.protobuf.Empty>(
                service, METHODID_DELETE_BUCKET_LIFECYCLE)))
        .addMethod(
          getGetPresignedPostFormDataMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataRequest,
              com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedPostFormDataResponse>(
                service, METHODID_GET_PRESIGNED_POST_FORM_DATA)))
        .addMethod(
          getGetPresignedObjectUrlMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlRequest,
              com.github.saturn_xiv.palm.plugins.s3.v1.GetPresignedObjectUrlResponse>(
                service, METHODID_GET_PRESIGNED_OBJECT_URL)))
        .addMethod(
          getListObjectsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.BucketRequest,
              com.github.saturn_xiv.palm.plugins.s3.v1.ListObjectsResponse>(
                service, METHODID_LIST_OBJECTS)))
        .addMethod(
          getGetObjectTagsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest,
              com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectTagsResponse>(
                service, METHODID_GET_OBJECT_TAGS)))
        .addMethod(
          getGetObjectRetentionMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest,
              com.github.saturn_xiv.palm.plugins.s3.v1.GetObjectRetentionResponse>(
                service, METHODID_GET_OBJECT_RETENTION)))
        .addMethod(
          getStatObjectMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.ObjectRequest,
              com.github.saturn_xiv.palm.plugins.s3.v1.StatObjectResponse>(
                service, METHODID_STAT_OBJECT)))
        .addMethod(
          getSetObjectTagsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.SetObjectTagsRequest,
              com.google.protobuf.Empty>(
                service, METHODID_SET_OBJECT_TAGS)))
        .addMethod(
          getDeleteObjectsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.s3.v1.DeleteObjectsRequest,
              com.google.protobuf.Empty>(
                service, METHODID_DELETE_OBJECTS)))
        .build();
  }

  private static abstract class S3BaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    S3BaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.s3.v1.S3OuterClass.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("S3");
    }
  }

  private static final class S3FileDescriptorSupplier
      extends S3BaseDescriptorSupplier {
    S3FileDescriptorSupplier() {}
  }

  private static final class S3MethodDescriptorSupplier
      extends S3BaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    S3MethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (S3Grpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new S3FileDescriptorSupplier())
              .addMethod(getListBucketsMethod())
              .addMethod(getBucketExistsMethod())
              .addMethod(getGetBucketEncryptionMethod())
              .addMethod(getGetBucketPolicyMethod())
              .addMethod(getGetBucketTagsMethod())
              .addMethod(getGetBucketLifecycleMethod())
              .addMethod(getMakeBucketMethod())
              .addMethod(getRemoveBucketMethod())
              .addMethod(getSetBucketTagsMethod())
              .addMethod(getSetBucketPolicyMethod())
              .addMethod(getSetBucketLifecycleMethod())
              .addMethod(getSetBucketEncryptionMethod())
              .addMethod(getDeleteBucketEncryptionMethod())
              .addMethod(getDeleteBucketPolicyMethod())
              .addMethod(getDeleteBucketTagsMethod())
              .addMethod(getDeleteBucketLifecycleMethod())
              .addMethod(getGetPresignedPostFormDataMethod())
              .addMethod(getGetPresignedObjectUrlMethod())
              .addMethod(getListObjectsMethod())
              .addMethod(getGetObjectTagsMethod())
              .addMethod(getGetObjectRetentionMethod())
              .addMethod(getStatObjectMethod())
              .addMethod(getSetObjectTagsMethod())
              .addMethod(getDeleteObjectsMethod())
              .build();
        }
      }
    }
    return result;
  }
}
