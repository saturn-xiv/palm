package com.github.saturn_xiv.palm.plugins.portal.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 * <pre>
 * ----------------------------------------------------------------------------
 * </pre>
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class EmailUserGrpc {

  private EmailUserGrpc() {}

  public static final java.lang.String SERVICE_NAME = "palm.portal.v1.EmailUser";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignInRequest,
      com.github.saturn_xiv.palm.plugins.portal.v1.UserSignInResponse> getSignInMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SignIn",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignInRequest.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.UserSignInResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignInRequest,
      com.github.saturn_xiv.palm.plugins.portal.v1.UserSignInResponse> getSignInMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignInRequest, com.github.saturn_xiv.palm.plugins.portal.v1.UserSignInResponse> getSignInMethod;
    if ((getSignInMethod = EmailUserGrpc.getSignInMethod) == null) {
      synchronized (EmailUserGrpc.class) {
        if ((getSignInMethod = EmailUserGrpc.getSignInMethod) == null) {
          EmailUserGrpc.getSignInMethod = getSignInMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignInRequest, com.github.saturn_xiv.palm.plugins.portal.v1.UserSignInResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SignIn"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignInRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.UserSignInResponse.getDefaultInstance()))
              .setSchemaDescriptor(new EmailUserMethodDescriptorSupplier("SignIn"))
              .build();
        }
      }
    }
    return getSignInMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignUpRequest,
      com.google.protobuf.Empty> getSignUpMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SignUp",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignUpRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignUpRequest,
      com.google.protobuf.Empty> getSignUpMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignUpRequest, com.google.protobuf.Empty> getSignUpMethod;
    if ((getSignUpMethod = EmailUserGrpc.getSignUpMethod) == null) {
      synchronized (EmailUserGrpc.class) {
        if ((getSignUpMethod = EmailUserGrpc.getSignUpMethod) == null) {
          EmailUserGrpc.getSignUpMethod = getSignUpMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignUpRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SignUp"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignUpRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new EmailUserMethodDescriptorSupplier("SignUp"))
              .build();
        }
      }
    }
    return getSignUpMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest,
      com.google.protobuf.Empty> getConfirmByEmailMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ConfirmByEmail",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest,
      com.google.protobuf.Empty> getConfirmByEmailMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest, com.google.protobuf.Empty> getConfirmByEmailMethod;
    if ((getConfirmByEmailMethod = EmailUserGrpc.getConfirmByEmailMethod) == null) {
      synchronized (EmailUserGrpc.class) {
        if ((getConfirmByEmailMethod = EmailUserGrpc.getConfirmByEmailMethod) == null) {
          EmailUserGrpc.getConfirmByEmailMethod = getConfirmByEmailMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ConfirmByEmail"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new EmailUserMethodDescriptorSupplier("ConfirmByEmail"))
              .build();
        }
      }
    }
    return getConfirmByEmailMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest,
      com.google.protobuf.Empty> getConfirmByTokenMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ConfirmByToken",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest,
      com.google.protobuf.Empty> getConfirmByTokenMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest, com.google.protobuf.Empty> getConfirmByTokenMethod;
    if ((getConfirmByTokenMethod = EmailUserGrpc.getConfirmByTokenMethod) == null) {
      synchronized (EmailUserGrpc.class) {
        if ((getConfirmByTokenMethod = EmailUserGrpc.getConfirmByTokenMethod) == null) {
          EmailUserGrpc.getConfirmByTokenMethod = getConfirmByTokenMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ConfirmByToken"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new EmailUserMethodDescriptorSupplier("ConfirmByToken"))
              .build();
        }
      }
    }
    return getConfirmByTokenMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest,
      com.google.protobuf.Empty> getUnlockByEmailMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "UnlockByEmail",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest,
      com.google.protobuf.Empty> getUnlockByEmailMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest, com.google.protobuf.Empty> getUnlockByEmailMethod;
    if ((getUnlockByEmailMethod = EmailUserGrpc.getUnlockByEmailMethod) == null) {
      synchronized (EmailUserGrpc.class) {
        if ((getUnlockByEmailMethod = EmailUserGrpc.getUnlockByEmailMethod) == null) {
          EmailUserGrpc.getUnlockByEmailMethod = getUnlockByEmailMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "UnlockByEmail"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new EmailUserMethodDescriptorSupplier("UnlockByEmail"))
              .build();
        }
      }
    }
    return getUnlockByEmailMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest,
      com.google.protobuf.Empty> getUnlockByTokenMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "UnlockByToken",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest,
      com.google.protobuf.Empty> getUnlockByTokenMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest, com.google.protobuf.Empty> getUnlockByTokenMethod;
    if ((getUnlockByTokenMethod = EmailUserGrpc.getUnlockByTokenMethod) == null) {
      synchronized (EmailUserGrpc.class) {
        if ((getUnlockByTokenMethod = EmailUserGrpc.getUnlockByTokenMethod) == null) {
          EmailUserGrpc.getUnlockByTokenMethod = getUnlockByTokenMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "UnlockByToken"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new EmailUserMethodDescriptorSupplier("UnlockByToken"))
              .build();
        }
      }
    }
    return getUnlockByTokenMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest,
      com.google.protobuf.Empty> getForgotPasswordMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ForgotPassword",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest,
      com.google.protobuf.Empty> getForgotPasswordMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest, com.google.protobuf.Empty> getForgotPasswordMethod;
    if ((getForgotPasswordMethod = EmailUserGrpc.getForgotPasswordMethod) == null) {
      synchronized (EmailUserGrpc.class) {
        if ((getForgotPasswordMethod = EmailUserGrpc.getForgotPasswordMethod) == null) {
          EmailUserGrpc.getForgotPasswordMethod = getForgotPasswordMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ForgotPassword"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new EmailUserMethodDescriptorSupplier("ForgotPassword"))
              .build();
        }
      }
    }
    return getForgotPasswordMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserResetPasswordRequest,
      com.google.protobuf.Empty> getResetPasswordMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ResetPassword",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserResetPasswordRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserResetPasswordRequest,
      com.google.protobuf.Empty> getResetPasswordMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserResetPasswordRequest, com.google.protobuf.Empty> getResetPasswordMethod;
    if ((getResetPasswordMethod = EmailUserGrpc.getResetPasswordMethod) == null) {
      synchronized (EmailUserGrpc.class) {
        if ((getResetPasswordMethod = EmailUserGrpc.getResetPasswordMethod) == null) {
          EmailUserGrpc.getResetPasswordMethod = getResetPasswordMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserResetPasswordRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ResetPassword"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserResetPasswordRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new EmailUserMethodDescriptorSupplier("ResetPassword"))
              .build();
        }
      }
    }
    return getResetPasswordMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest,
      com.google.protobuf.Empty> getChangePasswordMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ChangePassword",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest,
      com.google.protobuf.Empty> getChangePasswordMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest, com.google.protobuf.Empty> getChangePasswordMethod;
    if ((getChangePasswordMethod = EmailUserGrpc.getChangePasswordMethod) == null) {
      synchronized (EmailUserGrpc.class) {
        if ((getChangePasswordMethod = EmailUserGrpc.getChangePasswordMethod) == null) {
          EmailUserGrpc.getChangePasswordMethod = getChangePasswordMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ChangePassword"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new EmailUserMethodDescriptorSupplier("ChangePassword"))
              .build();
        }
      }
    }
    return getChangePasswordMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetRealNameRequest,
      com.google.protobuf.Empty> getSetRealNameMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SetRealName",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetRealNameRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetRealNameRequest,
      com.google.protobuf.Empty> getSetRealNameMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetRealNameRequest, com.google.protobuf.Empty> getSetRealNameMethod;
    if ((getSetRealNameMethod = EmailUserGrpc.getSetRealNameMethod) == null) {
      synchronized (EmailUserGrpc.class) {
        if ((getSetRealNameMethod = EmailUserGrpc.getSetRealNameMethod) == null) {
          EmailUserGrpc.getSetRealNameMethod = getSetRealNameMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetRealNameRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SetRealName"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetRealNameRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new EmailUserMethodDescriptorSupplier("SetRealName"))
              .build();
        }
      }
    }
    return getSetRealNameMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetAvatarRequest,
      com.google.protobuf.Empty> getSetAvatarMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SetAvatar",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetAvatarRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetAvatarRequest,
      com.google.protobuf.Empty> getSetAvatarMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetAvatarRequest, com.google.protobuf.Empty> getSetAvatarMethod;
    if ((getSetAvatarMethod = EmailUserGrpc.getSetAvatarMethod) == null) {
      synchronized (EmailUserGrpc.class) {
        if ((getSetAvatarMethod = EmailUserGrpc.getSetAvatarMethod) == null) {
          EmailUserGrpc.getSetAvatarMethod = getSetAvatarMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetAvatarRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SetAvatar"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetAvatarRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new EmailUserMethodDescriptorSupplier("SetAvatar"))
              .build();
        }
      }
    }
    return getSetAvatarMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserUploadAvatarResponse> getUploadAvatarMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "UploadAvatar",
      requestType = com.google.protobuf.Empty.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserUploadAvatarResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.google.protobuf.Empty,
      com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserUploadAvatarResponse> getUploadAvatarMethod() {
    io.grpc.MethodDescriptor<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserUploadAvatarResponse> getUploadAvatarMethod;
    if ((getUploadAvatarMethod = EmailUserGrpc.getUploadAvatarMethod) == null) {
      synchronized (EmailUserGrpc.class) {
        if ((getUploadAvatarMethod = EmailUserGrpc.getUploadAvatarMethod) == null) {
          EmailUserGrpc.getUploadAvatarMethod = getUploadAvatarMethod =
              io.grpc.MethodDescriptor.<com.google.protobuf.Empty, com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserUploadAvatarResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "UploadAvatar"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserUploadAvatarResponse.getDefaultInstance()))
              .setSchemaDescriptor(new EmailUserMethodDescriptorSupplier("UploadAvatar"))
              .build();
        }
      }
    }
    return getUploadAvatarMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserDeleteByEmailRequest,
      com.google.protobuf.Empty> getDeleteByEmailMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeleteByEmail",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserDeleteByEmailRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserDeleteByEmailRequest,
      com.google.protobuf.Empty> getDeleteByEmailMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserDeleteByEmailRequest, com.google.protobuf.Empty> getDeleteByEmailMethod;
    if ((getDeleteByEmailMethod = EmailUserGrpc.getDeleteByEmailMethod) == null) {
      synchronized (EmailUserGrpc.class) {
        if ((getDeleteByEmailMethod = EmailUserGrpc.getDeleteByEmailMethod) == null) {
          EmailUserGrpc.getDeleteByEmailMethod = getDeleteByEmailMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserDeleteByEmailRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeleteByEmail"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserDeleteByEmailRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new EmailUserMethodDescriptorSupplier("DeleteByEmail"))
              .build();
        }
      }
    }
    return getDeleteByEmailMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest,
      com.google.protobuf.Empty> getDeleteByTokenMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeleteByToken",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest,
      com.google.protobuf.Empty> getDeleteByTokenMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest, com.google.protobuf.Empty> getDeleteByTokenMethod;
    if ((getDeleteByTokenMethod = EmailUserGrpc.getDeleteByTokenMethod) == null) {
      synchronized (EmailUserGrpc.class) {
        if ((getDeleteByTokenMethod = EmailUserGrpc.getDeleteByTokenMethod) == null) {
          EmailUserGrpc.getDeleteByTokenMethod = getDeleteByTokenMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeleteByToken"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new EmailUserMethodDescriptorSupplier("DeleteByToken"))
              .build();
        }
      }
    }
    return getDeleteByTokenMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetPasswordRequest,
      com.google.protobuf.Empty> getSetPasswordMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SetPassword",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetPasswordRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetPasswordRequest,
      com.google.protobuf.Empty> getSetPasswordMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetPasswordRequest, com.google.protobuf.Empty> getSetPasswordMethod;
    if ((getSetPasswordMethod = EmailUserGrpc.getSetPasswordMethod) == null) {
      synchronized (EmailUserGrpc.class) {
        if ((getSetPasswordMethod = EmailUserGrpc.getSetPasswordMethod) == null) {
          EmailUserGrpc.getSetPasswordMethod = getSetPasswordMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetPasswordRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SetPassword"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetPasswordRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new EmailUserMethodDescriptorSupplier("SetPassword"))
              .build();
        }
      }
    }
    return getSetPasswordMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest,
      com.google.protobuf.Empty> getConfirmMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Confirm",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest,
      com.google.protobuf.Empty> getConfirmMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest, com.google.protobuf.Empty> getConfirmMethod;
    if ((getConfirmMethod = EmailUserGrpc.getConfirmMethod) == null) {
      synchronized (EmailUserGrpc.class) {
        if ((getConfirmMethod = EmailUserGrpc.getConfirmMethod) == null) {
          EmailUserGrpc.getConfirmMethod = getConfirmMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Confirm"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new EmailUserMethodDescriptorSupplier("Confirm"))
              .build();
        }
      }
    }
    return getConfirmMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest,
      com.google.protobuf.Empty> getDisableMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Disable",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest,
      com.google.protobuf.Empty> getDisableMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest, com.google.protobuf.Empty> getDisableMethod;
    if ((getDisableMethod = EmailUserGrpc.getDisableMethod) == null) {
      synchronized (EmailUserGrpc.class) {
        if ((getDisableMethod = EmailUserGrpc.getDisableMethod) == null) {
          EmailUserGrpc.getDisableMethod = getDisableMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Disable"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new EmailUserMethodDescriptorSupplier("Disable"))
              .build();
        }
      }
    }
    return getDisableMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest,
      com.google.protobuf.Empty> getEnableMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Enable",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest.class,
      responseType = com.google.protobuf.Empty.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest,
      com.google.protobuf.Empty> getEnableMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest, com.google.protobuf.Empty> getEnableMethod;
    if ((getEnableMethod = EmailUserGrpc.getEnableMethod) == null) {
      synchronized (EmailUserGrpc.class) {
        if ((getEnableMethod = EmailUserGrpc.getEnableMethod) == null) {
          EmailUserGrpc.getEnableMethod = getEnableMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest, com.google.protobuf.Empty>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Enable"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.google.protobuf.Empty.getDefaultInstance()))
              .setSchemaDescriptor(new EmailUserMethodDescriptorSupplier("Enable"))
              .build();
        }
      }
    }
    return getEnableMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page,
      com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserIndexResponse> getIndexMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Index",
      requestType = com.github.saturn_xiv.palm.plugins.portal.v1.Page.class,
      responseType = com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserIndexResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page,
      com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserIndexResponse> getIndexMethod() {
    io.grpc.MethodDescriptor<com.github.saturn_xiv.palm.plugins.portal.v1.Page, com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserIndexResponse> getIndexMethod;
    if ((getIndexMethod = EmailUserGrpc.getIndexMethod) == null) {
      synchronized (EmailUserGrpc.class) {
        if ((getIndexMethod = EmailUserGrpc.getIndexMethod) == null) {
          EmailUserGrpc.getIndexMethod = getIndexMethod =
              io.grpc.MethodDescriptor.<com.github.saturn_xiv.palm.plugins.portal.v1.Page, com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserIndexResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Index"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.Page.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserIndexResponse.getDefaultInstance()))
              .setSchemaDescriptor(new EmailUserMethodDescriptorSupplier("Index"))
              .build();
        }
      }
    }
    return getIndexMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static EmailUserStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<EmailUserStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<EmailUserStub>() {
        @java.lang.Override
        public EmailUserStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new EmailUserStub(channel, callOptions);
        }
      };
    return EmailUserStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static EmailUserBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<EmailUserBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<EmailUserBlockingV2Stub>() {
        @java.lang.Override
        public EmailUserBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new EmailUserBlockingV2Stub(channel, callOptions);
        }
      };
    return EmailUserBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static EmailUserBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<EmailUserBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<EmailUserBlockingStub>() {
        @java.lang.Override
        public EmailUserBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new EmailUserBlockingStub(channel, callOptions);
        }
      };
    return EmailUserBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static EmailUserFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<EmailUserFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<EmailUserFutureStub>() {
        @java.lang.Override
        public EmailUserFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new EmailUserFutureStub(channel, callOptions);
        }
      };
    return EmailUserFutureStub.newStub(factory, channel);
  }

  /**
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public interface AsyncService {

    /**
     */
    default void signIn(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignInRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserSignInResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSignInMethod(), responseObserver);
    }

    /**
     */
    default void signUp(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignUpRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSignUpMethod(), responseObserver);
    }

    /**
     */
    default void confirmByEmail(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getConfirmByEmailMethod(), responseObserver);
    }

    /**
     */
    default void confirmByToken(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getConfirmByTokenMethod(), responseObserver);
    }

    /**
     */
    default void unlockByEmail(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getUnlockByEmailMethod(), responseObserver);
    }

    /**
     */
    default void unlockByToken(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getUnlockByTokenMethod(), responseObserver);
    }

    /**
     */
    default void forgotPassword(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getForgotPasswordMethod(), responseObserver);
    }

    /**
     */
    default void resetPassword(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserResetPasswordRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getResetPasswordMethod(), responseObserver);
    }

    /**
     */
    default void changePassword(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getChangePasswordMethod(), responseObserver);
    }

    /**
     */
    default void setRealName(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetRealNameRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSetRealNameMethod(), responseObserver);
    }

    /**
     */
    default void setAvatar(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetAvatarRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSetAvatarMethod(), responseObserver);
    }

    /**
     */
    default void uploadAvatar(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserUploadAvatarResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getUploadAvatarMethod(), responseObserver);
    }

    /**
     */
    default void deleteByEmail(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserDeleteByEmailRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeleteByEmailMethod(), responseObserver);
    }

    /**
     */
    default void deleteByToken(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeleteByTokenMethod(), responseObserver);
    }

    /**
     */
    default void setPassword(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetPasswordRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSetPasswordMethod(), responseObserver);
    }

    /**
     */
    default void confirm(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getConfirmMethod(), responseObserver);
    }

    /**
     */
    default void disable(com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDisableMethod(), responseObserver);
    }

    /**
     */
    default void enable(com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getEnableMethod(), responseObserver);
    }

    /**
     */
    default void index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserIndexResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getIndexMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service EmailUser.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static abstract class EmailUserImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return EmailUserGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service EmailUser.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class EmailUserStub
      extends io.grpc.stub.AbstractAsyncStub<EmailUserStub> {
    private EmailUserStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected EmailUserStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new EmailUserStub(channel, callOptions);
    }

    /**
     */
    public void signIn(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignInRequest request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserSignInResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSignInMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void signUp(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignUpRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSignUpMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void confirmByEmail(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getConfirmByEmailMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void confirmByToken(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getConfirmByTokenMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void unlockByEmail(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getUnlockByEmailMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void unlockByToken(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getUnlockByTokenMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void forgotPassword(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getForgotPasswordMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void resetPassword(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserResetPasswordRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getResetPasswordMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void changePassword(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getChangePasswordMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void setRealName(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetRealNameRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSetRealNameMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void setAvatar(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetAvatarRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSetAvatarMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void uploadAvatar(com.google.protobuf.Empty request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserUploadAvatarResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getUploadAvatarMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void deleteByEmail(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserDeleteByEmailRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeleteByEmailMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void deleteByToken(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeleteByTokenMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void setPassword(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetPasswordRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSetPasswordMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void confirm(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getConfirmMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void disable(com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDisableMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void enable(com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request,
        io.grpc.stub.StreamObserver<com.google.protobuf.Empty> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getEnableMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request,
        io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserIndexResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service EmailUser.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class EmailUserBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<EmailUserBlockingV2Stub> {
    private EmailUserBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected EmailUserBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new EmailUserBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.UserSignInResponse signIn(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignInRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getSignInMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty signUp(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignUpRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getSignUpMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty confirmByEmail(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getConfirmByEmailMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty confirmByToken(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getConfirmByTokenMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty unlockByEmail(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getUnlockByEmailMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty unlockByToken(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getUnlockByTokenMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty forgotPassword(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getForgotPasswordMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty resetPassword(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserResetPasswordRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getResetPasswordMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty changePassword(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getChangePasswordMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setRealName(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetRealNameRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getSetRealNameMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setAvatar(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetAvatarRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getSetAvatarMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserUploadAvatarResponse uploadAvatar(com.google.protobuf.Empty request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getUploadAvatarMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deleteByEmail(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserDeleteByEmailRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getDeleteByEmailMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deleteByToken(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getDeleteByTokenMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setPassword(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetPasswordRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getSetPasswordMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty confirm(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getConfirmMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty disable(com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getDisableMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty enable(com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getEnableMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserIndexResponse index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getIndexMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service EmailUser.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class EmailUserBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<EmailUserBlockingStub> {
    private EmailUserBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected EmailUserBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new EmailUserBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.UserSignInResponse signIn(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignInRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSignInMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty signUp(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignUpRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSignUpMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty confirmByEmail(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getConfirmByEmailMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty confirmByToken(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getConfirmByTokenMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty unlockByEmail(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getUnlockByEmailMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty unlockByToken(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getUnlockByTokenMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty forgotPassword(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getForgotPasswordMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty resetPassword(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserResetPasswordRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getResetPasswordMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty changePassword(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getChangePasswordMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setRealName(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetRealNameRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetRealNameMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setAvatar(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetAvatarRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetAvatarMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserUploadAvatarResponse uploadAvatar(com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getUploadAvatarMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deleteByEmail(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserDeleteByEmailRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeleteByEmailMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty deleteByToken(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeleteByTokenMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty setPassword(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetPasswordRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSetPasswordMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty confirm(com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getConfirmMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty disable(com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDisableMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.google.protobuf.Empty enable(com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getEnableMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserIndexResponse index(com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getIndexMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service EmailUser.
   * <pre>
   * ----------------------------------------------------------------------------
   * </pre>
   */
  public static final class EmailUserFutureStub
      extends io.grpc.stub.AbstractFutureStub<EmailUserFutureStub> {
    private EmailUserFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected EmailUserFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new EmailUserFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.UserSignInResponse> signIn(
        com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignInRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSignInMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> signUp(
        com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignUpRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSignUpMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> confirmByEmail(
        com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getConfirmByEmailMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> confirmByToken(
        com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getConfirmByTokenMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> unlockByEmail(
        com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getUnlockByEmailMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> unlockByToken(
        com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getUnlockByTokenMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> forgotPassword(
        com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getForgotPasswordMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> resetPassword(
        com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserResetPasswordRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getResetPasswordMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> changePassword(
        com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getChangePasswordMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> setRealName(
        com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetRealNameRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSetRealNameMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> setAvatar(
        com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetAvatarRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSetAvatarMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserUploadAvatarResponse> uploadAvatar(
        com.google.protobuf.Empty request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getUploadAvatarMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> deleteByEmail(
        com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserDeleteByEmailRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeleteByEmailMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> deleteByToken(
        com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeleteByTokenMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> setPassword(
        com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetPasswordRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSetPasswordMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> confirm(
        com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getConfirmMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> disable(
        com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDisableMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.google.protobuf.Empty> enable(
        com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getEnableMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserIndexResponse> index(
        com.github.saturn_xiv.palm.plugins.portal.v1.Page request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getIndexMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_SIGN_IN = 0;
  private static final int METHODID_SIGN_UP = 1;
  private static final int METHODID_CONFIRM_BY_EMAIL = 2;
  private static final int METHODID_CONFIRM_BY_TOKEN = 3;
  private static final int METHODID_UNLOCK_BY_EMAIL = 4;
  private static final int METHODID_UNLOCK_BY_TOKEN = 5;
  private static final int METHODID_FORGOT_PASSWORD = 6;
  private static final int METHODID_RESET_PASSWORD = 7;
  private static final int METHODID_CHANGE_PASSWORD = 8;
  private static final int METHODID_SET_REAL_NAME = 9;
  private static final int METHODID_SET_AVATAR = 10;
  private static final int METHODID_UPLOAD_AVATAR = 11;
  private static final int METHODID_DELETE_BY_EMAIL = 12;
  private static final int METHODID_DELETE_BY_TOKEN = 13;
  private static final int METHODID_SET_PASSWORD = 14;
  private static final int METHODID_CONFIRM = 15;
  private static final int METHODID_DISABLE = 16;
  private static final int METHODID_ENABLE = 17;
  private static final int METHODID_INDEX = 18;

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
        case METHODID_SIGN_IN:
          serviceImpl.signIn((com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignInRequest) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.UserSignInResponse>) responseObserver);
          break;
        case METHODID_SIGN_UP:
          serviceImpl.signUp((com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignUpRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_CONFIRM_BY_EMAIL:
          serviceImpl.confirmByEmail((com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_CONFIRM_BY_TOKEN:
          serviceImpl.confirmByToken((com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_UNLOCK_BY_EMAIL:
          serviceImpl.unlockByEmail((com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_UNLOCK_BY_TOKEN:
          serviceImpl.unlockByToken((com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_FORGOT_PASSWORD:
          serviceImpl.forgotPassword((com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_RESET_PASSWORD:
          serviceImpl.resetPassword((com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserResetPasswordRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_CHANGE_PASSWORD:
          serviceImpl.changePassword((com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_SET_REAL_NAME:
          serviceImpl.setRealName((com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetRealNameRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_SET_AVATAR:
          serviceImpl.setAvatar((com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetAvatarRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_UPLOAD_AVATAR:
          serviceImpl.uploadAvatar((com.google.protobuf.Empty) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserUploadAvatarResponse>) responseObserver);
          break;
        case METHODID_DELETE_BY_EMAIL:
          serviceImpl.deleteByEmail((com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserDeleteByEmailRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_DELETE_BY_TOKEN:
          serviceImpl.deleteByToken((com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_SET_PASSWORD:
          serviceImpl.setPassword((com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetPasswordRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_CONFIRM:
          serviceImpl.confirm((com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_DISABLE:
          serviceImpl.disable((com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_ENABLE:
          serviceImpl.enable((com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest) request,
              (io.grpc.stub.StreamObserver<com.google.protobuf.Empty>) responseObserver);
          break;
        case METHODID_INDEX:
          serviceImpl.index((com.github.saturn_xiv.palm.plugins.portal.v1.Page) request,
              (io.grpc.stub.StreamObserver<com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserIndexResponse>) responseObserver);
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
          getSignInMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignInRequest,
              com.github.saturn_xiv.palm.plugins.portal.v1.UserSignInResponse>(
                service, METHODID_SIGN_IN)))
        .addMethod(
          getSignUpMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSignUpRequest,
              com.google.protobuf.Empty>(
                service, METHODID_SIGN_UP)))
        .addMethod(
          getConfirmByEmailMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest,
              com.google.protobuf.Empty>(
                service, METHODID_CONFIRM_BY_EMAIL)))
        .addMethod(
          getConfirmByTokenMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest,
              com.google.protobuf.Empty>(
                service, METHODID_CONFIRM_BY_TOKEN)))
        .addMethod(
          getUnlockByEmailMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest,
              com.google.protobuf.Empty>(
                service, METHODID_UNLOCK_BY_EMAIL)))
        .addMethod(
          getUnlockByTokenMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest,
              com.google.protobuf.Empty>(
                service, METHODID_UNLOCK_BY_TOKEN)))
        .addMethod(
          getForgotPasswordMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserRequest,
              com.google.protobuf.Empty>(
                service, METHODID_FORGOT_PASSWORD)))
        .addMethod(
          getResetPasswordMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserResetPasswordRequest,
              com.google.protobuf.Empty>(
                service, METHODID_RESET_PASSWORD)))
        .addMethod(
          getChangePasswordMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest,
              com.google.protobuf.Empty>(
                service, METHODID_CHANGE_PASSWORD)))
        .addMethod(
          getSetRealNameMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetRealNameRequest,
              com.google.protobuf.Empty>(
                service, METHODID_SET_REAL_NAME)))
        .addMethod(
          getSetAvatarMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetAvatarRequest,
              com.google.protobuf.Empty>(
                service, METHODID_SET_AVATAR)))
        .addMethod(
          getUploadAvatarMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.google.protobuf.Empty,
              com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserUploadAvatarResponse>(
                service, METHODID_UPLOAD_AVATAR)))
        .addMethod(
          getDeleteByEmailMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserDeleteByEmailRequest,
              com.google.protobuf.Empty>(
                service, METHODID_DELETE_BY_EMAIL)))
        .addMethod(
          getDeleteByTokenMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserByTokenRequest,
              com.google.protobuf.Empty>(
                service, METHODID_DELETE_BY_TOKEN)))
        .addMethod(
          getSetPasswordMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserSetPasswordRequest,
              com.google.protobuf.Empty>(
                service, METHODID_SET_PASSWORD)))
        .addMethod(
          getConfirmMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserChangePasswordRequest,
              com.google.protobuf.Empty>(
                service, METHODID_CONFIRM)))
        .addMethod(
          getDisableMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest,
              com.google.protobuf.Empty>(
                service, METHODID_DISABLE)))
        .addMethod(
          getEnableMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.SetupUserRequest,
              com.google.protobuf.Empty>(
                service, METHODID_ENABLE)))
        .addMethod(
          getIndexMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.github.saturn_xiv.palm.plugins.portal.v1.Page,
              com.github.saturn_xiv.palm.plugins.portal.v1.EmailUserIndexResponse>(
                service, METHODID_INDEX)))
        .build();
  }

  private static abstract class EmailUserBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    EmailUserBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.github.saturn_xiv.palm.plugins.portal.v1.Portal.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("EmailUser");
    }
  }

  private static final class EmailUserFileDescriptorSupplier
      extends EmailUserBaseDescriptorSupplier {
    EmailUserFileDescriptorSupplier() {}
  }

  private static final class EmailUserMethodDescriptorSupplier
      extends EmailUserBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    EmailUserMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (EmailUserGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new EmailUserFileDescriptorSupplier())
              .addMethod(getSignInMethod())
              .addMethod(getSignUpMethod())
              .addMethod(getConfirmByEmailMethod())
              .addMethod(getConfirmByTokenMethod())
              .addMethod(getUnlockByEmailMethod())
              .addMethod(getUnlockByTokenMethod())
              .addMethod(getForgotPasswordMethod())
              .addMethod(getResetPasswordMethod())
              .addMethod(getChangePasswordMethod())
              .addMethod(getSetRealNameMethod())
              .addMethod(getSetAvatarMethod())
              .addMethod(getUploadAvatarMethod())
              .addMethod(getDeleteByEmailMethod())
              .addMethod(getDeleteByTokenMethod())
              .addMethod(getSetPasswordMethod())
              .addMethod(getConfirmMethod())
              .addMethod(getDisableMethod())
              .addMethod(getEnableMethod())
              .addMethod(getIndexMethod())
              .build();
        }
      }
    }
    return result;
  }
}
