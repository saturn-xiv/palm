package com.github.saturn_xiv.palm.plugins.portal.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 * <pre>
 * ----------------------------------------------------------------------------
 * </pre>
 */
@javax.annotation.Generated(
    value = "by gRPC proto compiler (version 1.68.1)",
    comments = "Source: portal.proto")
@io.grpc.stub.annotations.GrpcGenerated
public final class SiteGrpc {

  private SiteGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.portal.v1.Site";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SetSiteInfoByLangRequest,
      com.google.protobuf.Empty> getSetInfoByLangMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SetInfoByLang",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.SetSiteInfoByLangRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SetSiteInfoByLangRequest,
      com.google.protobuf.Empty> getSetInfoByLangMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SetSiteInfoByLangRequest, com.google.protobuf.Empty> getSetInfoByLangMethod;
    if ((getSetInfoByLangMethod = SiteGrpc.getSetInfoByLangMethod) == null) {
      synchronized (SiteGrpc.class) {
        if ((getSetInfoByLangMethod = SiteGrpc.getSetInfoByLangMethod) == null) {
          SiteGrpc.getSetInfoByLangMethod = getSetInfoByLangMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.SetSiteInfoByLangRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SetInfoByLang"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.SetSiteInfoByLangRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new SiteMethodDescriptorSupplier("SetInfoByLang"))
              .build();
        }
      }
    }
    return getSetInfoByLangMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.GetSiteInfoByLangRequest,
      com.github.saturn_xiv.palm.plugins.portal.v1.GetSiteInfoByLangResponse> getGetInfoByLangMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetInfoByLang",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.GetSiteInfoByLangRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.GetSiteInfoByLangResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.GetSiteInfoByLangRequest,
      com.github.saturn_xiv.palm.plugins.portal.v1.GetSiteInfoByLangResponse> getGetInfoByLangMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.GetSiteInfoByLangRequest, com.github.saturn_xiv.palm.plugins.portal.v1.GetSiteInfoByLangResponse> getGetInfoByLangMethod;
    if ((getGetInfoByLangMethod = SiteGrpc.getGetInfoByLangMethod) == null) {
      synchronized (SiteGrpc.class) {
        if ((getGetInfoByLangMethod = SiteGrpc.getGetInfoByLangMethod) == null) {
          SiteGrpc.getGetInfoByLangMethod = getGetInfoByLangMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.GetSiteInfoByLangRequest, com.github.saturn_xiv.palm.plugins.portal.v1.GetSiteInfoByLangResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetInfoByLang"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.GetSiteInfoByLangRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.GetSiteInfoByLangResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SiteMethodDescriptorSupplier("GetInfoByLang"))
              .build();
        }
      }
    }
    return getGetInfoByLangMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SiteAuthorProfile,
      com.google.protobuf.Empty> getSetAuthorMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SetAuthor",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.SiteAuthorProfile.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SiteAuthorProfile,
      com.google.protobuf.Empty> getSetAuthorMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SiteAuthorProfile, com.google.protobuf.Empty> getSetAuthorMethod;
    if ((getSetAuthorMethod = SiteGrpc.getSetAuthorMethod) == null) {
      synchronized (SiteGrpc.class) {
        if ((getSetAuthorMethod = SiteGrpc.getSetAuthorMethod) == null) {
          SiteGrpc.getSetAuthorMethod = getSetAuthorMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.SiteAuthorProfile, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SetAuthor"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.SiteAuthorProfile.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new SiteMethodDescriptorSupplier("SetAuthor"))
              .build();
        }
      }
    }
    return getSetAuthorMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.SiteAuthorProfile> getGetAuthorMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetAuthor",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.SiteAuthorProfile.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.SiteAuthorProfile> getGetAuthorMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.SiteAuthorProfile> getGetAuthorMethod;
    if ((getGetAuthorMethod = SiteGrpc.getGetAuthorMethod) == null) {
      synchronized (SiteGrpc.class) {
        if ((getGetAuthorMethod = SiteGrpc.getGetAuthorMethod) == null) {
          SiteGrpc.getGetAuthorMethod = getGetAuthorMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.SiteAuthorProfile>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetAuthor"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.SiteAuthorProfile.getDefaultInstance()))
              .setSchemaDescriptor(new SiteMethodDescriptorSupplier("GetAuthor"))
              .build();
        }
      }
    }
    return getGetAuthorMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SiteFaviconProfile,
      com.google.protobuf.Empty> getSetFaviconMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SetFavicon",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.SiteFaviconProfile.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SiteFaviconProfile,
      com.google.protobuf.Empty> getSetFaviconMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SiteFaviconProfile, com.google.protobuf.Empty> getSetFaviconMethod;
    if ((getSetFaviconMethod = SiteGrpc.getSetFaviconMethod) == null) {
      synchronized (SiteGrpc.class) {
        if ((getSetFaviconMethod = SiteGrpc.getSetFaviconMethod) == null) {
          SiteGrpc.getSetFaviconMethod = getSetFaviconMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.SiteFaviconProfile, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SetFavicon"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.SiteFaviconProfile.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new SiteMethodDescriptorSupplier("SetFavicon"))
              .build();
        }
      }
    }
    return getSetFaviconMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.SiteFaviconProfile> getGetFaviconMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetFavicon",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.SiteFaviconProfile.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.SiteFaviconProfile> getGetFaviconMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.SiteFaviconProfile> getGetFaviconMethod;
    if ((getGetFaviconMethod = SiteGrpc.getGetFaviconMethod) == null) {
      synchronized (SiteGrpc.class) {
        if ((getGetFaviconMethod = SiteGrpc.getGetFaviconMethod) == null) {
          SiteGrpc.getGetFaviconMethod = getGetFaviconMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.SiteFaviconProfile>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetFavicon"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.SiteFaviconProfile.getDefaultInstance()))
              .setSchemaDescriptor(new SiteMethodDescriptorSupplier("GetFavicon"))
              .build();
        }
      }
    }
    return getGetFaviconMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.SiteUploadFaviconResponse> getUploadFaviconMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "UploadFavicon",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.SiteUploadFaviconResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.SiteUploadFaviconResponse> getUploadFaviconMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.SiteUploadFaviconResponse> getUploadFaviconMethod;
    if ((getUploadFaviconMethod = SiteGrpc.getUploadFaviconMethod) == null) {
      synchronized (SiteGrpc.class) {
        if ((getUploadFaviconMethod = SiteGrpc.getUploadFaviconMethod) == null) {
          SiteGrpc.getUploadFaviconMethod = getUploadFaviconMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.SiteUploadFaviconResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "UploadFavicon"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.SiteUploadFaviconResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SiteMethodDescriptorSupplier("UploadFavicon"))
              .build();
        }
      }
    }
    return getUploadFaviconMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.GoogleSiteOwnershipVerification,
      com.google.protobuf.Empty> getSetGoogleSiteOwnershipVerificationMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SetGoogleSiteOwnershipVerification",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.GoogleSiteOwnershipVerification.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.GoogleSiteOwnershipVerification,
      com.google.protobuf.Empty> getSetGoogleSiteOwnershipVerificationMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.GoogleSiteOwnershipVerification, com.google.protobuf.Empty> getSetGoogleSiteOwnershipVerificationMethod;
    if ((getSetGoogleSiteOwnershipVerificationMethod = SiteGrpc.getSetGoogleSiteOwnershipVerificationMethod) == null) {
      synchronized (SiteGrpc.class) {
        if ((getSetGoogleSiteOwnershipVerificationMethod = SiteGrpc.getSetGoogleSiteOwnershipVerificationMethod) == null) {
          SiteGrpc.getSetGoogleSiteOwnershipVerificationMethod = getSetGoogleSiteOwnershipVerificationMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.GoogleSiteOwnershipVerification, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SetGoogleSiteOwnershipVerification"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.GoogleSiteOwnershipVerification.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new SiteMethodDescriptorSupplier("SetGoogleSiteOwnershipVerification"))
              .build();
        }
      }
    }
    return getSetGoogleSiteOwnershipVerificationMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.GoogleSiteOwnershipVerification> getGetGoogleSiteOwnershipVerificationMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetGoogleSiteOwnershipVerification",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.GoogleSiteOwnershipVerification.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.GoogleSiteOwnershipVerification> getGetGoogleSiteOwnershipVerificationMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.GoogleSiteOwnershipVerification> getGetGoogleSiteOwnershipVerificationMethod;
    if ((getGetGoogleSiteOwnershipVerificationMethod = SiteGrpc.getGetGoogleSiteOwnershipVerificationMethod) == null) {
      synchronized (SiteGrpc.class) {
        if ((getGetGoogleSiteOwnershipVerificationMethod = SiteGrpc.getGetGoogleSiteOwnershipVerificationMethod) == null) {
          SiteGrpc.getGetGoogleSiteOwnershipVerificationMethod = getGetGoogleSiteOwnershipVerificationMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.GoogleSiteOwnershipVerification>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetGoogleSiteOwnershipVerification"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.GoogleSiteOwnershipVerification.getDefaultInstance()))
              .setSchemaDescriptor(new SiteMethodDescriptorSupplier("GetGoogleSiteOwnershipVerification"))
              .build();
        }
      }
    }
    return getGetGoogleSiteOwnershipVerificationMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.ReCaptchaProfile,
      com.google.protobuf.Empty> getSetReCaptchaMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SetReCaptcha",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.ReCaptchaProfile.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.ReCaptchaProfile,
      com.google.protobuf.Empty> getSetReCaptchaMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.ReCaptchaProfile, com.google.protobuf.Empty> getSetReCaptchaMethod;
    if ((getSetReCaptchaMethod = SiteGrpc.getSetReCaptchaMethod) == null) {
      synchronized (SiteGrpc.class) {
        if ((getSetReCaptchaMethod = SiteGrpc.getSetReCaptchaMethod) == null) {
          SiteGrpc.getSetReCaptchaMethod = getSetReCaptchaMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.ReCaptchaProfile, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SetReCaptcha"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.ReCaptchaProfile.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new SiteMethodDescriptorSupplier("SetReCaptcha"))
              .build();
        }
      }
    }
    return getSetReCaptchaMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.ReCaptchaProfile> getGetReCaptchaMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetReCaptcha",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.ReCaptchaProfile.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.ReCaptchaProfile> getGetReCaptchaMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.ReCaptchaProfile> getGetReCaptchaMethod;
    if ((getGetReCaptchaMethod = SiteGrpc.getGetReCaptchaMethod) == null) {
      synchronized (SiteGrpc.class) {
        if ((getGetReCaptchaMethod = SiteGrpc.getGetReCaptchaMethod) == null) {
          SiteGrpc.getGetReCaptchaMethod = getGetReCaptchaMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.ReCaptchaProfile>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetReCaptcha"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.ReCaptchaProfile.getDefaultInstance()))
              .setSchemaDescriptor(new SiteMethodDescriptorSupplier("GetReCaptcha"))
              .build();
        }
      }
    }
    return getGetReCaptchaMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.BaiduSiteOwnershipVerification,
      com.google.protobuf.Empty> getSetBaiduSiteOwnershipVerificationMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SetBaiduSiteOwnershipVerification",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.BaiduSiteOwnershipVerification.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.BaiduSiteOwnershipVerification,
      com.google.protobuf.Empty> getSetBaiduSiteOwnershipVerificationMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.BaiduSiteOwnershipVerification, com.google.protobuf.Empty> getSetBaiduSiteOwnershipVerificationMethod;
    if ((getSetBaiduSiteOwnershipVerificationMethod = SiteGrpc.getSetBaiduSiteOwnershipVerificationMethod) == null) {
      synchronized (SiteGrpc.class) {
        if ((getSetBaiduSiteOwnershipVerificationMethod = SiteGrpc.getSetBaiduSiteOwnershipVerificationMethod) == null) {
          SiteGrpc.getSetBaiduSiteOwnershipVerificationMethod = getSetBaiduSiteOwnershipVerificationMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.BaiduSiteOwnershipVerification, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SetBaiduSiteOwnershipVerification"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.BaiduSiteOwnershipVerification.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new SiteMethodDescriptorSupplier("SetBaiduSiteOwnershipVerification"))
              .build();
        }
      }
    }
    return getSetBaiduSiteOwnershipVerificationMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.BaiduSiteOwnershipVerification> getGetBaiduSiteOwnershipVerificationMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetBaiduSiteOwnershipVerification",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.BaiduSiteOwnershipVerification.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.BaiduSiteOwnershipVerification> getGetBaiduSiteOwnershipVerificationMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.BaiduSiteOwnershipVerification> getGetBaiduSiteOwnershipVerificationMethod;
    if ((getGetBaiduSiteOwnershipVerificationMethod = SiteGrpc.getGetBaiduSiteOwnershipVerificationMethod) == null) {
      synchronized (SiteGrpc.class) {
        if ((getGetBaiduSiteOwnershipVerificationMethod = SiteGrpc.getGetBaiduSiteOwnershipVerificationMethod) == null) {
          SiteGrpc.getGetBaiduSiteOwnershipVerificationMethod = getGetBaiduSiteOwnershipVerificationMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.BaiduSiteOwnershipVerification>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetBaiduSiteOwnershipVerification"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.BaiduSiteOwnershipVerification.getDefaultInstance()))
              .setSchemaDescriptor(new SiteMethodDescriptorSupplier("GetBaiduSiteOwnershipVerification"))
              .build();
        }
      }
    }
    return getGetBaiduSiteOwnershipVerificationMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.google.protobuf.Empty> getPingBaiduMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "PingBaidu",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.google.protobuf.Empty> getPingBaiduMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.google.protobuf.Empty> getPingBaiduMethod;
    if ((getPingBaiduMethod = SiteGrpc.getPingBaiduMethod) == null) {
      synchronized (SiteGrpc.class) {
        if ((getPingBaiduMethod = SiteGrpc.getPingBaiduMethod) == null) {
          SiteGrpc.getPingBaiduMethod = getPingBaiduMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "PingBaidu"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new SiteMethodDescriptorSupplier("PingBaidu"))
              .build();
        }
      }
    }
    return getPingBaiduMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IndexNowProfile,
      com.google.protobuf.Empty> getSetIndexNowMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SetIndexNow",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.IndexNowProfile.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IndexNowProfile,
      com.google.protobuf.Empty> getSetIndexNowMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.IndexNowProfile, com.google.protobuf.Empty> getSetIndexNowMethod;
    if ((getSetIndexNowMethod = SiteGrpc.getSetIndexNowMethod) == null) {
      synchronized (SiteGrpc.class) {
        if ((getSetIndexNowMethod = SiteGrpc.getSetIndexNowMethod) == null) {
          SiteGrpc.getSetIndexNowMethod = getSetIndexNowMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.IndexNowProfile, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SetIndexNow"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.IndexNowProfile.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new SiteMethodDescriptorSupplier("SetIndexNow"))
              .build();
        }
      }
    }
    return getSetIndexNowMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.IndexNowProfile> getGetIndexNowMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetIndexNow",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.IndexNowProfile.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.IndexNowProfile> getGetIndexNowMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.IndexNowProfile> getGetIndexNowMethod;
    if ((getGetIndexNowMethod = SiteGrpc.getGetIndexNowMethod) == null) {
      synchronized (SiteGrpc.class) {
        if ((getGetIndexNowMethod = SiteGrpc.getGetIndexNowMethod) == null) {
          SiteGrpc.getGetIndexNowMethod = getGetIndexNowMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.IndexNowProfile>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetIndexNow"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.IndexNowProfile.getDefaultInstance()))
              .setSchemaDescriptor(new SiteMethodDescriptorSupplier("GetIndexNow"))
              .build();
        }
      }
    }
    return getGetIndexNowMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.google.protobuf.Empty> getPingIndexNowMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "PingIndexNow",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.google.protobuf.Empty> getPingIndexNowMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.google.protobuf.Empty> getPingIndexNowMethod;
    if ((getPingIndexNowMethod = SiteGrpc.getPingIndexNowMethod) == null) {
      synchronized (SiteGrpc.class) {
        if ((getPingIndexNowMethod = SiteGrpc.getPingIndexNowMethod) == null) {
          SiteGrpc.getPingIndexNowMethod = getPingIndexNowMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "PingIndexNow"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new SiteMethodDescriptorSupplier("PingIndexNow"))
              .build();
        }
      }
    }
    return getPingIndexNowMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.google.protobuf.Empty> getClearCacheMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ClearCache",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.google.protobuf.Empty> getClearCacheMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.google.protobuf.Empty> getClearCacheMethod;
    if ((getClearCacheMethod = SiteGrpc.getClearCacheMethod) == null) {
      synchronized (SiteGrpc.class) {
        if ((getClearCacheMethod = SiteGrpc.getClearCacheMethod) == null) {
          SiteGrpc.getClearCacheMethod = getClearCacheMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ClearCache"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new SiteMethodDescriptorSupplier("ClearCache"))
              .build();
        }
      }
    }
    return getClearCacheMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SiteSetMaintenanceModeRequest,
      com.google.protobuf.Empty> getSetMaintenanceModeMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SetMaintenanceMode",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.SiteSetMaintenanceModeRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SiteSetMaintenanceModeRequest,
      com.google.protobuf.Empty> getSetMaintenanceModeMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SiteSetMaintenanceModeRequest, com.google.protobuf.Empty> getSetMaintenanceModeMethod;
    if ((getSetMaintenanceModeMethod = SiteGrpc.getSetMaintenanceModeMethod) == null) {
      synchronized (SiteGrpc.class) {
        if ((getSetMaintenanceModeMethod = SiteGrpc.getSetMaintenanceModeMethod) == null) {
          SiteGrpc.getSetMaintenanceModeMethod = getSetMaintenanceModeMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.SiteSetMaintenanceModeRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SetMaintenanceMode"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.SiteSetMaintenanceModeRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new SiteMethodDescriptorSupplier("SetMaintenanceMode"))
              .build();
        }
      }
    }
    return getSetMaintenanceModeMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.SiteTimezonesResponse> getTimezonesMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Timezones",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.SiteTimezonesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.SiteTimezonesResponse> getTimezonesMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.SiteTimezonesResponse> getTimezonesMethod;
    if ((getTimezonesMethod = SiteGrpc.getTimezonesMethod) == null) {
      synchronized (SiteGrpc.class) {
        if ((getTimezonesMethod = SiteGrpc.getTimezonesMethod) == null) {
          SiteGrpc.getTimezonesMethod = getTimezonesMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.SiteTimezonesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Timezones"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.SiteTimezonesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SiteMethodDescriptorSupplier("Timezones"))
              .build();
        }
      }
    }
    return getTimezonesMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.SiteCurrenciesResponse> getCurrenciesMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Currencies",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.SiteCurrenciesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.SiteCurrenciesResponse> getCurrenciesMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.SiteCurrenciesResponse> getCurrenciesMethod;
    if ((getCurrenciesMethod = SiteGrpc.getCurrenciesMethod) == null) {
      synchronized (SiteGrpc.class) {
        if ((getCurrenciesMethod = SiteGrpc.getCurrenciesMethod) == null) {
          SiteGrpc.getCurrenciesMethod = getCurrenciesMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.SiteCurrenciesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Currencies"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.SiteCurrenciesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SiteMethodDescriptorSupplier("Currencies"))
              .build();
        }
      }
    }
    return getCurrenciesMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.SiteLanguagesResponse> getLanguagesMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Languages",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.SiteLanguagesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.SiteLanguagesResponse> getLanguagesMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.SiteLanguagesResponse> getLanguagesMethod;
    if ((getLanguagesMethod = SiteGrpc.getLanguagesMethod) == null) {
      synchronized (SiteGrpc.class) {
        if ((getLanguagesMethod = SiteGrpc.getLanguagesMethod) == null) {
          SiteGrpc.getLanguagesMethod = getLanguagesMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.SiteLanguagesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Languages"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.SiteLanguagesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SiteMethodDescriptorSupplier("Languages"))
              .build();
        }
      }
    }
    return getLanguagesMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static SiteStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SiteStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SiteStub>() {
        @java.lang.Override
        public SiteStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SiteStub(channel, callOptions);
        }
      };
    return SiteStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static SiteBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SiteBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SiteBlockingStub>() {
        @java.lang.Override
        public SiteBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SiteBlockingStub(channel, callOptions);
        }
      };
    return SiteBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static SiteFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SiteFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SiteFutureStub>() {
        @java.lang.Override
        public SiteFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SiteFutureStub(channel, callOptions);
        }
      };
    return SiteFutureStub.newStub(factory, channel);
  }

  /**
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public interface AsyncService {

    /**
     */
    default void setInfoByLang(com.github.saturn_xiv.palm.plugins.portal.v1.SetSiteInfoByLangRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSetInfoByLangMethod(), responseObserver);
    }

    /**
     */
    default void getInfoByLang(com.github.saturn_xiv.palm.plugins.portal.v1.GetSiteInfoByLangRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.GetSiteInfoByLangResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetInfoByLangMethod(), responseObserver);
    }

    /**
     */
    default void setAuthor(com.github.saturn_xiv.palm.plugins.portal.v1.SiteAuthorProfile request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSetAuthorMethod(), responseObserver);
    }

    /**
     */
    default void getAuthor(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.SiteAuthorProfile> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetAuthorMethod(), responseObserver);
    }

    /**
     */
    default void setFavicon(com.github.saturn_xiv.palm.plugins.portal.v1.SiteFaviconProfile request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSetFaviconMethod(), responseObserver);
    }

    /**
     */
    default void getFavicon(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.SiteFaviconProfile> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetFaviconMethod(), responseObserver);
    }

    /**
     */
    default void uploadFavicon(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.SiteUploadFaviconResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getUploadFaviconMethod(), responseObserver);
    }

    /**
     */
    default void setGoogleSiteOwnershipVerification(com.github.saturn_xiv.palm.plugins.portal.v1.GoogleSiteOwnershipVerification request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSetGoogleSiteOwnershipVerificationMethod(), responseObserver);
    }

    /**
     */
    default void getGoogleSiteOwnershipVerification(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.GoogleSiteOwnershipVerification> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetGoogleSiteOwnershipVerificationMethod(), responseObserver);
    }

    /**
     */
    default void setReCaptcha(com.github.saturn_xiv.palm.plugins.portal.v1.ReCaptchaProfile request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSetReCaptchaMethod(), responseObserver);
    }

    /**
     */
    default void getReCaptcha(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.ReCaptchaProfile> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetReCaptchaMethod(), responseObserver);
    }

    /**
     */
    default void setBaiduSiteOwnershipVerification(com.github.saturn_xiv.palm.plugins.portal.v1.BaiduSiteOwnershipVerification request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSetBaiduSiteOwnershipVerificationMethod(), responseObserver);
    }

    /**
     */
    default void getBaiduSiteOwnershipVerification(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.BaiduSiteOwnershipVerification> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetBaiduSiteOwnershipVerificationMethod(), responseObserver);
    }

    /**
     */
    default void pingBaidu(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getPingBaiduMethod(), responseObserver);
    }

    /**
     */
    default void setIndexNow(com.github.saturn_xiv.palm.plugins.portal.v1.IndexNowProfile request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSetIndexNowMethod(), responseObserver);
    }

    /**
     */
    default void getIndexNow(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.IndexNowProfile> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetIndexNowMethod(), responseObserver);
    }

    /**
     */
    default void pingIndexNow(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getPingIndexNowMethod(), responseObserver);
    }

    /**
     */
    default void clearCache(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getClearCacheMethod(), responseObserver);
    }

    /**
     */
    default void setMaintenanceMode(com.github.saturn_xiv.palm.plugins.portal.v1.SiteSetMaintenanceModeRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSetMaintenanceModeMethod(), responseObserver);
    }

    /**
     */
    default void timezones(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.SiteTimezonesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getTimezonesMethod(), responseObserver);
    }

    /**
     */
    default void currencies(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.SiteCurrenciesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCurrenciesMethod(), responseObserver);
    }

    /**
     */
    default void languages(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.SiteLanguagesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getLanguagesMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service Site.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static abstract class SiteImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return SiteGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Site.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class SiteStub
      extends io.grpc.stub.AbstractAsyncStub<SiteStub> {
    private SiteStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SiteStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SiteStub(channel, callOptions);
    }

    /**
     */
    public void setInfoByLang(com.github.saturn_xiv.palm.plugins.portal.v1.SetSiteInfoByLangRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSetInfoByLangMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getInfoByLang(com.github.saturn_xiv.palm.plugins.portal.v1.GetSiteInfoByLangRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.GetSiteInfoByLangResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetInfoByLangMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void setAuthor(com.github.saturn_xiv.palm.plugins.portal.v1.SiteAuthorProfile request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSetAuthorMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getAuthor(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.SiteAuthorProfile> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetAuthorMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void setFavicon(com.github.saturn_xiv.palm.plugins.portal.v1.SiteFaviconProfile request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSetFaviconMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getFavicon(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.SiteFaviconProfile> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetFaviconMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void uploadFavicon(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.SiteUploadFaviconResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getUploadFaviconMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void setGoogleSiteOwnershipVerification(com.github.saturn_xiv.palm.plugins.portal.v1.GoogleSiteOwnershipVerification request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSetGoogleSiteOwnershipVerificationMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getGoogleSiteOwnershipVerification(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.GoogleSiteOwnershipVerification> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetGoogleSiteOwnershipVerificationMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void setReCaptcha(com.github.saturn_xiv.palm.plugins.portal.v1.ReCaptchaProfile request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSetReCaptchaMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getReCaptcha(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.ReCaptchaProfile> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetReCaptchaMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void setBaiduSiteOwnershipVerification(com.github.saturn_xiv.palm.plugins.portal.v1.BaiduSiteOwnershipVerification request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSetBaiduSiteOwnershipVerificationMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getBaiduSiteOwnershipVerification(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.BaiduSiteOwnershipVerification> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetBaiduSiteOwnershipVerificationMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void pingBaidu(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getPingBaiduMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void setIndexNow(com.github.saturn_xiv.palm.plugins.portal.v1.IndexNowProfile request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSetIndexNowMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getIndexNow(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.IndexNowProfile> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetIndexNowMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void pingIndexNow(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getPingIndexNowMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void clearCache(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getClearCacheMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void setMaintenanceMode(com.github.saturn_xiv.palm.plugins.portal.v1.SiteSetMaintenanceModeRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSetMaintenanceModeMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void timezones(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.SiteTimezonesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getTimezonesMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void currencies(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.SiteCurrenciesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCurrenciesMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void languages(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.SiteLanguagesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getLanguagesMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Site.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class SiteBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<SiteBlockingStub> {
    private SiteBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SiteBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SiteBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.google.protobuf.Empty setInfoByLang(com.github.saturn_xiv.palm.plugins.portal.v1.SetSiteInfoByLangRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetInfoByLangMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.GetSiteInfoByLangResponse getInfoByLang(com.github.saturn_xiv.palm.plugins.portal.v1.GetSiteInfoByLangRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetInfoByLangMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setAuthor(com.github.saturn_xiv.palm.plugins.portal.v1.SiteAuthorProfile request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetAuthorMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.SiteAuthorProfile getAuthor(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetAuthorMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setFavicon(com.github.saturn_xiv.palm.plugins.portal.v1.SiteFaviconProfile request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetFaviconMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.SiteFaviconProfile getFavicon(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetFaviconMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.SiteUploadFaviconResponse uploadFavicon(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getUploadFaviconMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setGoogleSiteOwnershipVerification(com.github.saturn_xiv.palm.plugins.portal.v1.GoogleSiteOwnershipVerification request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetGoogleSiteOwnershipVerificationMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.GoogleSiteOwnershipVerification getGoogleSiteOwnershipVerification(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetGoogleSiteOwnershipVerificationMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setReCaptcha(com.github.saturn_xiv.palm.plugins.portal.v1.ReCaptchaProfile request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetReCaptchaMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.ReCaptchaProfile getReCaptcha(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetReCaptchaMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setBaiduSiteOwnershipVerification(com.github.saturn_xiv.palm.plugins.portal.v1.BaiduSiteOwnershipVerification request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetBaiduSiteOwnershipVerificationMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.BaiduSiteOwnershipVerification getBaiduSiteOwnershipVerification(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetBaiduSiteOwnershipVerificationMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty pingBaidu(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getPingBaiduMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setIndexNow(com.github.saturn_xiv.palm.plugins.portal.v1.IndexNowProfile request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetIndexNowMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.IndexNowProfile getIndexNow(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetIndexNowMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty pingIndexNow(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getPingIndexNowMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty clearCache(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getClearCacheMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setMaintenanceMode(com.github.saturn_xiv.palm.plugins.portal.v1.SiteSetMaintenanceModeRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetMaintenanceModeMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.SiteTimezonesResponse timezones(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getTimezonesMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.SiteCurrenciesResponse currencies(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCurrenciesMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.SiteLanguagesResponse languages(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getLanguagesMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Site.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class SiteFutureStub
      extends io.grpc.stub.AbstractFutureStub<SiteFutureStub> {
    private SiteFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SiteFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SiteFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> setInfoByLang(
        com.github.saturn_xiv.palm.plugins.portal.v1.SetSiteInfoByLangRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSetInfoByLangMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.GetSiteInfoByLangResponse> getInfoByLang(
        com.github.saturn_xiv.palm.plugins.portal.v1.GetSiteInfoByLangRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetInfoByLangMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> setAuthor(
        com.github.saturn_xiv.palm.plugins.portal.v1.SiteAuthorProfile request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSetAuthorMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.SiteAuthorProfile> getAuthor(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetAuthorMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> setFavicon(
        com.github.saturn_xiv.palm.plugins.portal.v1.SiteFaviconProfile request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSetFaviconMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.SiteFaviconProfile> getFavicon(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetFaviconMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.SiteUploadFaviconResponse> uploadFavicon(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getUploadFaviconMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> setGoogleSiteOwnershipVerification(
        com.github.saturn_xiv.palm.plugins.portal.v1.GoogleSiteOwnershipVerification request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSetGoogleSiteOwnershipVerificationMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.GoogleSiteOwnershipVerification> getGoogleSiteOwnershipVerification(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetGoogleSiteOwnershipVerificationMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> setReCaptcha(
        com.github.saturn_xiv.palm.plugins.portal.v1.ReCaptchaProfile request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSetReCaptchaMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.ReCaptchaProfile> getReCaptcha(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetReCaptchaMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> setBaiduSiteOwnershipVerification(
        com.github.saturn_xiv.palm.plugins.portal.v1.BaiduSiteOwnershipVerification request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSetBaiduSiteOwnershipVerificationMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.BaiduSiteOwnershipVerification> getBaiduSiteOwnershipVerification(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetBaiduSiteOwnershipVerificationMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> pingBaidu(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getPingBaiduMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> setIndexNow(
        com.github.saturn_xiv.palm.plugins.portal.v1.IndexNowProfile request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSetIndexNowMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.IndexNowProfile> getIndexNow(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetIndexNowMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> pingIndexNow(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getPingIndexNowMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> clearCache(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getClearCacheMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> setMaintenanceMode(
        com.github.saturn_xiv.palm.plugins.portal.v1.SiteSetMaintenanceModeRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSetMaintenanceModeMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.SiteTimezonesResponse> timezones(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getTimezonesMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.SiteCurrenciesResponse> currencies(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCurrenciesMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.SiteLanguagesResponse> languages(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getLanguagesMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_SET_INFO_BY_LANG = 0;
  private static final int METHODID_GET_INFO_BY_LANG = 1;
  private static final int METHODID_SET_AUTHOR = 2;
  private static final int METHODID_GET_AUTHOR = 3;
  private static final int METHODID_SET_FAVICON = 4;
  private static final int METHODID_GET_FAVICON = 5;
  private static final int METHODID_UPLOAD_FAVICON = 6;
  private static final int METHODID_SET_GOOGLE_SITE_OWNERSHIP_VERIFICATION = 7;
  private static final int METHODID_GET_GOOGLE_SITE_OWNERSHIP_VERIFICATION = 8;
  private static final int METHODID_SET_RE_CAPTCHA = 9;
  private static final int METHODID_GET_RE_CAPTCHA = 10;
  private static final int METHODID_SET_BAIDU_SITE_OWNERSHIP_VERIFICATION = 11;
  private static final int METHODID_GET_BAIDU_SITE_OWNERSHIP_VERIFICATION = 12;
  private static final int METHODID_PING_BAIDU = 13;
  private static final int METHODID_SET_INDEX_NOW = 14;
  private static final int METHODID_GET_INDEX_NOW = 15;
  private static final int METHODID_PING_INDEX_NOW = 16;
  private static final int METHODID_CLEAR_CACHE = 17;
  private static final int METHODID_SET_MAINTENANCE_MODE = 18;
  private static final int METHODID_TIMEZONES = 19;
  private static final int METHODID_CURRENCIES = 20;
  private static final int METHODID_LANGUAGES = 21;

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
        case METHODID_SET_INFO_BY_LANG:
          serviceImpl.setInfoByLang((com.github.saturn_xiv.palm.plugins.portal.v1.SetSiteInfoByLangRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_GET_INFO_BY_LANG:
          serviceImpl.getInfoByLang((com.github.saturn_xiv.palm.plugins.portal.v1.GetSiteInfoByLangRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.GetSiteInfoByLangResponse>) responseObserver);
          break;
        case METHODID_SET_AUTHOR:
          serviceImpl.setAuthor((com.github.saturn_xiv.palm.plugins.portal.v1.SiteAuthorProfile) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_GET_AUTHOR:
          serviceImpl.getAuthor((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.SiteAuthorProfile>) responseObserver);
          break;
        case METHODID_SET_FAVICON:
          serviceImpl.setFavicon((com.github.saturn_xiv.palm.plugins.portal.v1.SiteFaviconProfile) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_GET_FAVICON:
          serviceImpl.getFavicon((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.SiteFaviconProfile>) responseObserver);
          break;
        case METHODID_UPLOAD_FAVICON:
          serviceImpl.uploadFavicon((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.SiteUploadFaviconResponse>) responseObserver);
          break;
        case METHODID_SET_GOOGLE_SITE_OWNERSHIP_VERIFICATION:
          serviceImpl.setGoogleSiteOwnershipVerification((com.github.saturn_xiv.palm.plugins.portal.v1.GoogleSiteOwnershipVerification) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_GET_GOOGLE_SITE_OWNERSHIP_VERIFICATION:
          serviceImpl.getGoogleSiteOwnershipVerification((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.GoogleSiteOwnershipVerification>) responseObserver);
          break;
        case METHODID_SET_RE_CAPTCHA:
          serviceImpl.setReCaptcha((com.github.saturn_xiv.palm.plugins.portal.v1.ReCaptchaProfile) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_GET_RE_CAPTCHA:
          serviceImpl.getReCaptcha((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.ReCaptchaProfile>) responseObserver);
          break;
        case METHODID_SET_BAIDU_SITE_OWNERSHIP_VERIFICATION:
          serviceImpl.setBaiduSiteOwnershipVerification((com.github.saturn_xiv.palm.plugins.portal.v1.BaiduSiteOwnershipVerification) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_GET_BAIDU_SITE_OWNERSHIP_VERIFICATION:
          serviceImpl.getBaiduSiteOwnershipVerification((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.BaiduSiteOwnershipVerification>) responseObserver);
          break;
        case METHODID_PING_BAIDU:
          serviceImpl.pingBaidu((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_SET_INDEX_NOW:
          serviceImpl.setIndexNow((com.github.saturn_xiv.palm.plugins.portal.v1.IndexNowProfile) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_GET_INDEX_NOW:
          serviceImpl.getIndexNow((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.IndexNowProfile>) responseObserver);
          break;
        case METHODID_PING_INDEX_NOW:
          serviceImpl.pingIndexNow((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_CLEAR_CACHE:
          serviceImpl.clearCache((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_SET_MAINTENANCE_MODE:
          serviceImpl.setMaintenanceMode((com.github.saturn_xiv.palm.plugins.portal.v1.SiteSetMaintenanceModeRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_TIMEZONES:
          serviceImpl.timezones((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.SiteTimezonesResponse>) responseObserver);
          break;
        case METHODID_CURRENCIES:
          serviceImpl.currencies((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.SiteCurrenciesResponse>) responseObserver);
          break;
        case METHODID_LANGUAGES:
          serviceImpl.languages((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.SiteLanguagesResponse>) responseObserver);
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
          getSetInfoByLangMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.SetSiteInfoByLangRequest,
              com.google.protobuf.Empty>(
                service, METHODID_SET_INFO_BY_LANG)))
        .addMethod(
          getGetInfoByLangMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.GetSiteInfoByLangRequest,
              com.github.saturn_xiv.palm.plugins.portal.v1.GetSiteInfoByLangResponse>(
                service, METHODID_GET_INFO_BY_LANG)))
        .addMethod(
          getSetAuthorMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.SiteAuthorProfile,
              com.google.protobuf.Empty>(
                service, METHODID_SET_AUTHOR)))
        .addMethod(
          getGetAuthorMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.portal.v1.SiteAuthorProfile>(
                service, METHODID_GET_AUTHOR)))
        .addMethod(
          getSetFaviconMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.SiteFaviconProfile,
              com.google.protobuf.Empty>(
                service, METHODID_SET_FAVICON)))
        .addMethod(
          getGetFaviconMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.portal.v1.SiteFaviconProfile>(
                service, METHODID_GET_FAVICON)))
        .addMethod(
          getUploadFaviconMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.portal.v1.SiteUploadFaviconResponse>(
                service, METHODID_UPLOAD_FAVICON)))
        .addMethod(
          getSetGoogleSiteOwnershipVerificationMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.GoogleSiteOwnershipVerification,
              com.google.protobuf.Empty>(
                service, METHODID_SET_GOOGLE_SITE_OWNERSHIP_VERIFICATION)))
        .addMethod(
          getGetGoogleSiteOwnershipVerificationMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.portal.v1.GoogleSiteOwnershipVerification>(
                service, METHODID_GET_GOOGLE_SITE_OWNERSHIP_VERIFICATION)))
        .addMethod(
          getSetReCaptchaMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.ReCaptchaProfile,
              com.google.protobuf.Empty>(
                service, METHODID_SET_RE_CAPTCHA)))
        .addMethod(
          getGetReCaptchaMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.portal.v1.ReCaptchaProfile>(
                service, METHODID_GET_RE_CAPTCHA)))
        .addMethod(
          getSetBaiduSiteOwnershipVerificationMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.BaiduSiteOwnershipVerification,
              com.google.protobuf.Empty>(
                service, METHODID_SET_BAIDU_SITE_OWNERSHIP_VERIFICATION)))
        .addMethod(
          getGetBaiduSiteOwnershipVerificationMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.portal.v1.BaiduSiteOwnershipVerification>(
                service, METHODID_GET_BAIDU_SITE_OWNERSHIP_VERIFICATION)))
        .addMethod(
          getPingBaiduMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.google.protobuf.Empty>(
                service, METHODID_PING_BAIDU)))
        .addMethod(
          getSetIndexNowMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.IndexNowProfile,
              com.google.protobuf.Empty>(
                service, METHODID_SET_INDEX_NOW)))
        .addMethod(
          getGetIndexNowMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.portal.v1.IndexNowProfile>(
                service, METHODID_GET_INDEX_NOW)))
        .addMethod(
          getPingIndexNowMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.google.protobuf.Empty>(
                service, METHODID_PING_INDEX_NOW)))
        .addMethod(
          getClearCacheMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.google.protobuf.Empty>(
                service, METHODID_CLEAR_CACHE)))
        .addMethod(
          getSetMaintenanceModeMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.SiteSetMaintenanceModeRequest,
              com.google.protobuf.Empty>(
                service, METHODID_SET_MAINTENANCE_MODE)))
        .addMethod(
          getTimezonesMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.portal.v1.SiteTimezonesResponse>(
                service, METHODID_TIMEZONES)))
        .addMethod(
          getCurrenciesMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.portal.v1.SiteCurrenciesResponse>(
                service, METHODID_CURRENCIES)))
        .addMethod(
          getLanguagesMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.portal.v1.SiteLanguagesResponse>(
                service, METHODID_LANGUAGES)))
        .build();
  }

  private static abstract class SiteBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    SiteBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.portal.v1.Portal.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Site");
    }
  }

  private static final class SiteFileDescriptorSupplier
      extends SiteBaseDescriptorSupplier {
    SiteFileDescriptorSupplier() {}
  }

  private static final class SiteMethodDescriptorSupplier
      extends SiteBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    SiteMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (SiteGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new SiteFileDescriptorSupplier())
              .addMethod(getSetInfoByLangMethod())
              .addMethod(getGetInfoByLangMethod())
              .addMethod(getSetAuthorMethod())
              .addMethod(getGetAuthorMethod())
              .addMethod(getSetFaviconMethod())
              .addMethod(getGetFaviconMethod())
              .addMethod(getUploadFaviconMethod())
              .addMethod(getSetGoogleSiteOwnershipVerificationMethod())
              .addMethod(getGetGoogleSiteOwnershipVerificationMethod())
              .addMethod(getSetReCaptchaMethod())
              .addMethod(getGetReCaptchaMethod())
              .addMethod(getSetBaiduSiteOwnershipVerificationMethod())
              .addMethod(getGetBaiduSiteOwnershipVerificationMethod())
              .addMethod(getPingBaiduMethod())
              .addMethod(getSetIndexNowMethod())
              .addMethod(getGetIndexNowMethod())
              .addMethod(getPingIndexNowMethod())
              .addMethod(getClearCacheMethod())
              .addMethod(getSetMaintenanceModeMethod())
              .addMethod(getTimezonesMethod())
              .addMethod(getCurrenciesMethod())
              .addMethod(getLanguagesMethod())
              .build();
        }
      }
    }
    return result;
  }
}
