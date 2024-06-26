// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: musa.proto

// Protobuf Java Version: 3.25.1
package com.github.saturn_xiv.palm.plugins.musa.v1;

/**
 * Protobuf type {@code palm.musa.v1.WechatPayRefundResponse}
 */
public final class WechatPayRefundResponse extends
    com.google.protobuf.GeneratedMessageV3 implements
    // @@protoc_insertion_point(message_implements:palm.musa.v1.WechatPayRefundResponse)
    WechatPayRefundResponseOrBuilder {
private static final long serialVersionUID = 0L;
  // Use WechatPayRefundResponse.newBuilder() to construct.
  private WechatPayRefundResponse(com.google.protobuf.GeneratedMessageV3.Builder<?> builder) {
    super(builder);
  }
  private WechatPayRefundResponse() {
    outRefundNo_ = "";
    channel_ = "";
    status_ = "";
    userReceivedAccount_ = "";
    createTime_ = "";
  }

  @java.lang.Override
  @SuppressWarnings({"unused"})
  protected java.lang.Object newInstance(
      UnusedPrivateParameter unused) {
    return new WechatPayRefundResponse();
  }

  public static final com.google.protobuf.Descriptors.Descriptor
      getDescriptor() {
    return com.github.saturn_xiv.palm.plugins.musa.v1.Musa.internal_static_palm_musa_v1_WechatPayRefundResponse_descriptor;
  }

  @java.lang.Override
  protected com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internalGetFieldAccessorTable() {
    return com.github.saturn_xiv.palm.plugins.musa.v1.Musa.internal_static_palm_musa_v1_WechatPayRefundResponse_fieldAccessorTable
        .ensureFieldAccessorsInitialized(
            com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse.class, com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse.Builder.class);
  }

  public static final int OUT_REFUND_NO_FIELD_NUMBER = 1;
  @SuppressWarnings("serial")
  private volatile java.lang.Object outRefundNo_ = "";
  /**
   * <code>string out_refund_no = 1;</code>
   * @return The outRefundNo.
   */
  @java.lang.Override
  public java.lang.String getOutRefundNo() {
    java.lang.Object ref = outRefundNo_;
    if (ref instanceof java.lang.String) {
      return (java.lang.String) ref;
    } else {
      com.google.protobuf.ByteString bs = 
          (com.google.protobuf.ByteString) ref;
      java.lang.String s = bs.toStringUtf8();
      outRefundNo_ = s;
      return s;
    }
  }
  /**
   * <code>string out_refund_no = 1;</code>
   * @return The bytes for outRefundNo.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString
      getOutRefundNoBytes() {
    java.lang.Object ref = outRefundNo_;
    if (ref instanceof java.lang.String) {
      com.google.protobuf.ByteString b = 
          com.google.protobuf.ByteString.copyFromUtf8(
              (java.lang.String) ref);
      outRefundNo_ = b;
      return b;
    } else {
      return (com.google.protobuf.ByteString) ref;
    }
  }

  public static final int CHANNEL_FIELD_NUMBER = 2;
  @SuppressWarnings("serial")
  private volatile java.lang.Object channel_ = "";
  /**
   * <code>string channel = 2;</code>
   * @return The channel.
   */
  @java.lang.Override
  public java.lang.String getChannel() {
    java.lang.Object ref = channel_;
    if (ref instanceof java.lang.String) {
      return (java.lang.String) ref;
    } else {
      com.google.protobuf.ByteString bs = 
          (com.google.protobuf.ByteString) ref;
      java.lang.String s = bs.toStringUtf8();
      channel_ = s;
      return s;
    }
  }
  /**
   * <code>string channel = 2;</code>
   * @return The bytes for channel.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString
      getChannelBytes() {
    java.lang.Object ref = channel_;
    if (ref instanceof java.lang.String) {
      com.google.protobuf.ByteString b = 
          com.google.protobuf.ByteString.copyFromUtf8(
              (java.lang.String) ref);
      channel_ = b;
      return b;
    } else {
      return (com.google.protobuf.ByteString) ref;
    }
  }

  public static final int STATUS_FIELD_NUMBER = 3;
  @SuppressWarnings("serial")
  private volatile java.lang.Object status_ = "";
  /**
   * <code>string status = 3;</code>
   * @return The status.
   */
  @java.lang.Override
  public java.lang.String getStatus() {
    java.lang.Object ref = status_;
    if (ref instanceof java.lang.String) {
      return (java.lang.String) ref;
    } else {
      com.google.protobuf.ByteString bs = 
          (com.google.protobuf.ByteString) ref;
      java.lang.String s = bs.toStringUtf8();
      status_ = s;
      return s;
    }
  }
  /**
   * <code>string status = 3;</code>
   * @return The bytes for status.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString
      getStatusBytes() {
    java.lang.Object ref = status_;
    if (ref instanceof java.lang.String) {
      com.google.protobuf.ByteString b = 
          com.google.protobuf.ByteString.copyFromUtf8(
              (java.lang.String) ref);
      status_ = b;
      return b;
    } else {
      return (com.google.protobuf.ByteString) ref;
    }
  }

  public static final int USER_RECEIVED_ACCOUNT_FIELD_NUMBER = 4;
  @SuppressWarnings("serial")
  private volatile java.lang.Object userReceivedAccount_ = "";
  /**
   * <code>string user_received_account = 4;</code>
   * @return The userReceivedAccount.
   */
  @java.lang.Override
  public java.lang.String getUserReceivedAccount() {
    java.lang.Object ref = userReceivedAccount_;
    if (ref instanceof java.lang.String) {
      return (java.lang.String) ref;
    } else {
      com.google.protobuf.ByteString bs = 
          (com.google.protobuf.ByteString) ref;
      java.lang.String s = bs.toStringUtf8();
      userReceivedAccount_ = s;
      return s;
    }
  }
  /**
   * <code>string user_received_account = 4;</code>
   * @return The bytes for userReceivedAccount.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString
      getUserReceivedAccountBytes() {
    java.lang.Object ref = userReceivedAccount_;
    if (ref instanceof java.lang.String) {
      com.google.protobuf.ByteString b = 
          com.google.protobuf.ByteString.copyFromUtf8(
              (java.lang.String) ref);
      userReceivedAccount_ = b;
      return b;
    } else {
      return (com.google.protobuf.ByteString) ref;
    }
  }

  public static final int CREATE_TIME_FIELD_NUMBER = 99;
  @SuppressWarnings("serial")
  private volatile java.lang.Object createTime_ = "";
  /**
   * <code>string create_time = 99;</code>
   * @return The createTime.
   */
  @java.lang.Override
  public java.lang.String getCreateTime() {
    java.lang.Object ref = createTime_;
    if (ref instanceof java.lang.String) {
      return (java.lang.String) ref;
    } else {
      com.google.protobuf.ByteString bs = 
          (com.google.protobuf.ByteString) ref;
      java.lang.String s = bs.toStringUtf8();
      createTime_ = s;
      return s;
    }
  }
  /**
   * <code>string create_time = 99;</code>
   * @return The bytes for createTime.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString
      getCreateTimeBytes() {
    java.lang.Object ref = createTime_;
    if (ref instanceof java.lang.String) {
      com.google.protobuf.ByteString b = 
          com.google.protobuf.ByteString.copyFromUtf8(
              (java.lang.String) ref);
      createTime_ = b;
      return b;
    } else {
      return (com.google.protobuf.ByteString) ref;
    }
  }

  private byte memoizedIsInitialized = -1;
  @java.lang.Override
  public final boolean isInitialized() {
    byte isInitialized = memoizedIsInitialized;
    if (isInitialized == 1) return true;
    if (isInitialized == 0) return false;

    memoizedIsInitialized = 1;
    return true;
  }

  @java.lang.Override
  public void writeTo(com.google.protobuf.CodedOutputStream output)
                      throws java.io.IOException {
    if (!com.google.protobuf.GeneratedMessageV3.isStringEmpty(outRefundNo_)) {
      com.google.protobuf.GeneratedMessageV3.writeString(output, 1, outRefundNo_);
    }
    if (!com.google.protobuf.GeneratedMessageV3.isStringEmpty(channel_)) {
      com.google.protobuf.GeneratedMessageV3.writeString(output, 2, channel_);
    }
    if (!com.google.protobuf.GeneratedMessageV3.isStringEmpty(status_)) {
      com.google.protobuf.GeneratedMessageV3.writeString(output, 3, status_);
    }
    if (!com.google.protobuf.GeneratedMessageV3.isStringEmpty(userReceivedAccount_)) {
      com.google.protobuf.GeneratedMessageV3.writeString(output, 4, userReceivedAccount_);
    }
    if (!com.google.protobuf.GeneratedMessageV3.isStringEmpty(createTime_)) {
      com.google.protobuf.GeneratedMessageV3.writeString(output, 99, createTime_);
    }
    getUnknownFields().writeTo(output);
  }

  @java.lang.Override
  public int getSerializedSize() {
    int size = memoizedSize;
    if (size != -1) return size;

    size = 0;
    if (!com.google.protobuf.GeneratedMessageV3.isStringEmpty(outRefundNo_)) {
      size += com.google.protobuf.GeneratedMessageV3.computeStringSize(1, outRefundNo_);
    }
    if (!com.google.protobuf.GeneratedMessageV3.isStringEmpty(channel_)) {
      size += com.google.protobuf.GeneratedMessageV3.computeStringSize(2, channel_);
    }
    if (!com.google.protobuf.GeneratedMessageV3.isStringEmpty(status_)) {
      size += com.google.protobuf.GeneratedMessageV3.computeStringSize(3, status_);
    }
    if (!com.google.protobuf.GeneratedMessageV3.isStringEmpty(userReceivedAccount_)) {
      size += com.google.protobuf.GeneratedMessageV3.computeStringSize(4, userReceivedAccount_);
    }
    if (!com.google.protobuf.GeneratedMessageV3.isStringEmpty(createTime_)) {
      size += com.google.protobuf.GeneratedMessageV3.computeStringSize(99, createTime_);
    }
    size += getUnknownFields().getSerializedSize();
    memoizedSize = size;
    return size;
  }

  @java.lang.Override
  public boolean equals(final java.lang.Object obj) {
    if (obj == this) {
     return true;
    }
    if (!(obj instanceof com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse)) {
      return super.equals(obj);
    }
    com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse other = (com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse) obj;

    if (!getOutRefundNo()
        .equals(other.getOutRefundNo())) return false;
    if (!getChannel()
        .equals(other.getChannel())) return false;
    if (!getStatus()
        .equals(other.getStatus())) return false;
    if (!getUserReceivedAccount()
        .equals(other.getUserReceivedAccount())) return false;
    if (!getCreateTime()
        .equals(other.getCreateTime())) return false;
    if (!getUnknownFields().equals(other.getUnknownFields())) return false;
    return true;
  }

  @java.lang.Override
  public int hashCode() {
    if (memoizedHashCode != 0) {
      return memoizedHashCode;
    }
    int hash = 41;
    hash = (19 * hash) + getDescriptor().hashCode();
    hash = (37 * hash) + OUT_REFUND_NO_FIELD_NUMBER;
    hash = (53 * hash) + getOutRefundNo().hashCode();
    hash = (37 * hash) + CHANNEL_FIELD_NUMBER;
    hash = (53 * hash) + getChannel().hashCode();
    hash = (37 * hash) + STATUS_FIELD_NUMBER;
    hash = (53 * hash) + getStatus().hashCode();
    hash = (37 * hash) + USER_RECEIVED_ACCOUNT_FIELD_NUMBER;
    hash = (53 * hash) + getUserReceivedAccount().hashCode();
    hash = (37 * hash) + CREATE_TIME_FIELD_NUMBER;
    hash = (53 * hash) + getCreateTime().hashCode();
    hash = (29 * hash) + getUnknownFields().hashCode();
    memoizedHashCode = hash;
    return hash;
  }

  public static com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse parseFrom(
      java.nio.ByteBuffer data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse parseFrom(
      java.nio.ByteBuffer data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input);
  }
  public static com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input, extensionRegistry);
  }

  public static com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseDelimitedWithIOException(PARSER, input);
  }

  public static com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseDelimitedWithIOException(PARSER, input, extensionRegistry);
  }
  public static com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input);
  }
  public static com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse parseFrom(
      com.google.protobuf.CodedInputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input, extensionRegistry);
  }

  @java.lang.Override
  public Builder newBuilderForType() { return newBuilder(); }
  public static Builder newBuilder() {
    return DEFAULT_INSTANCE.toBuilder();
  }
  public static Builder newBuilder(com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse prototype) {
    return DEFAULT_INSTANCE.toBuilder().mergeFrom(prototype);
  }
  @java.lang.Override
  public Builder toBuilder() {
    return this == DEFAULT_INSTANCE
        ? new Builder() : new Builder().mergeFrom(this);
  }

  @java.lang.Override
  protected Builder newBuilderForType(
      com.google.protobuf.GeneratedMessageV3.BuilderParent parent) {
    Builder builder = new Builder(parent);
    return builder;
  }
  /**
   * Protobuf type {@code palm.musa.v1.WechatPayRefundResponse}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessageV3.Builder<Builder> implements
      // @@protoc_insertion_point(builder_implements:palm.musa.v1.WechatPayRefundResponse)
      com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponseOrBuilder {
    public static final com.google.protobuf.Descriptors.Descriptor
        getDescriptor() {
      return com.github.saturn_xiv.palm.plugins.musa.v1.Musa.internal_static_palm_musa_v1_WechatPayRefundResponse_descriptor;
    }

    @java.lang.Override
    protected com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
        internalGetFieldAccessorTable() {
      return com.github.saturn_xiv.palm.plugins.musa.v1.Musa.internal_static_palm_musa_v1_WechatPayRefundResponse_fieldAccessorTable
          .ensureFieldAccessorsInitialized(
              com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse.class, com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse.Builder.class);
    }

    // Construct using com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse.newBuilder()
    private Builder() {

    }

    private Builder(
        com.google.protobuf.GeneratedMessageV3.BuilderParent parent) {
      super(parent);

    }
    @java.lang.Override
    public Builder clear() {
      super.clear();
      bitField0_ = 0;
      outRefundNo_ = "";
      channel_ = "";
      status_ = "";
      userReceivedAccount_ = "";
      createTime_ = "";
      return this;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.Descriptor
        getDescriptorForType() {
      return com.github.saturn_xiv.palm.plugins.musa.v1.Musa.internal_static_palm_musa_v1_WechatPayRefundResponse_descriptor;
    }

    @java.lang.Override
    public com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse getDefaultInstanceForType() {
      return com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse.getDefaultInstance();
    }

    @java.lang.Override
    public com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse build() {
      com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse result = buildPartial();
      if (!result.isInitialized()) {
        throw newUninitializedMessageException(result);
      }
      return result;
    }

    @java.lang.Override
    public com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse buildPartial() {
      com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse result = new com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse(this);
      if (bitField0_ != 0) { buildPartial0(result); }
      onBuilt();
      return result;
    }

    private void buildPartial0(com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse result) {
      int from_bitField0_ = bitField0_;
      if (((from_bitField0_ & 0x00000001) != 0)) {
        result.outRefundNo_ = outRefundNo_;
      }
      if (((from_bitField0_ & 0x00000002) != 0)) {
        result.channel_ = channel_;
      }
      if (((from_bitField0_ & 0x00000004) != 0)) {
        result.status_ = status_;
      }
      if (((from_bitField0_ & 0x00000008) != 0)) {
        result.userReceivedAccount_ = userReceivedAccount_;
      }
      if (((from_bitField0_ & 0x00000010) != 0)) {
        result.createTime_ = createTime_;
      }
    }

    @java.lang.Override
    public Builder clone() {
      return super.clone();
    }
    @java.lang.Override
    public Builder setField(
        com.google.protobuf.Descriptors.FieldDescriptor field,
        java.lang.Object value) {
      return super.setField(field, value);
    }
    @java.lang.Override
    public Builder clearField(
        com.google.protobuf.Descriptors.FieldDescriptor field) {
      return super.clearField(field);
    }
    @java.lang.Override
    public Builder clearOneof(
        com.google.protobuf.Descriptors.OneofDescriptor oneof) {
      return super.clearOneof(oneof);
    }
    @java.lang.Override
    public Builder setRepeatedField(
        com.google.protobuf.Descriptors.FieldDescriptor field,
        int index, java.lang.Object value) {
      return super.setRepeatedField(field, index, value);
    }
    @java.lang.Override
    public Builder addRepeatedField(
        com.google.protobuf.Descriptors.FieldDescriptor field,
        java.lang.Object value) {
      return super.addRepeatedField(field, value);
    }
    @java.lang.Override
    public Builder mergeFrom(com.google.protobuf.Message other) {
      if (other instanceof com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse) {
        return mergeFrom((com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse)other);
      } else {
        super.mergeFrom(other);
        return this;
      }
    }

    public Builder mergeFrom(com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse other) {
      if (other == com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse.getDefaultInstance()) return this;
      if (!other.getOutRefundNo().isEmpty()) {
        outRefundNo_ = other.outRefundNo_;
        bitField0_ |= 0x00000001;
        onChanged();
      }
      if (!other.getChannel().isEmpty()) {
        channel_ = other.channel_;
        bitField0_ |= 0x00000002;
        onChanged();
      }
      if (!other.getStatus().isEmpty()) {
        status_ = other.status_;
        bitField0_ |= 0x00000004;
        onChanged();
      }
      if (!other.getUserReceivedAccount().isEmpty()) {
        userReceivedAccount_ = other.userReceivedAccount_;
        bitField0_ |= 0x00000008;
        onChanged();
      }
      if (!other.getCreateTime().isEmpty()) {
        createTime_ = other.createTime_;
        bitField0_ |= 0x00000010;
        onChanged();
      }
      this.mergeUnknownFields(other.getUnknownFields());
      onChanged();
      return this;
    }

    @java.lang.Override
    public final boolean isInitialized() {
      return true;
    }

    @java.lang.Override
    public Builder mergeFrom(
        com.google.protobuf.CodedInputStream input,
        com.google.protobuf.ExtensionRegistryLite extensionRegistry)
        throws java.io.IOException {
      if (extensionRegistry == null) {
        throw new java.lang.NullPointerException();
      }
      try {
        boolean done = false;
        while (!done) {
          int tag = input.readTag();
          switch (tag) {
            case 0:
              done = true;
              break;
            case 10: {
              outRefundNo_ = input.readStringRequireUtf8();
              bitField0_ |= 0x00000001;
              break;
            } // case 10
            case 18: {
              channel_ = input.readStringRequireUtf8();
              bitField0_ |= 0x00000002;
              break;
            } // case 18
            case 26: {
              status_ = input.readStringRequireUtf8();
              bitField0_ |= 0x00000004;
              break;
            } // case 26
            case 34: {
              userReceivedAccount_ = input.readStringRequireUtf8();
              bitField0_ |= 0x00000008;
              break;
            } // case 34
            case 794: {
              createTime_ = input.readStringRequireUtf8();
              bitField0_ |= 0x00000010;
              break;
            } // case 794
            default: {
              if (!super.parseUnknownField(input, extensionRegistry, tag)) {
                done = true; // was an endgroup tag
              }
              break;
            } // default:
          } // switch (tag)
        } // while (!done)
      } catch (com.google.protobuf.InvalidProtocolBufferException e) {
        throw e.unwrapIOException();
      } finally {
        onChanged();
      } // finally
      return this;
    }
    private int bitField0_;

    private java.lang.Object outRefundNo_ = "";
    /**
     * <code>string out_refund_no = 1;</code>
     * @return The outRefundNo.
     */
    public java.lang.String getOutRefundNo() {
      java.lang.Object ref = outRefundNo_;
      if (!(ref instanceof java.lang.String)) {
        com.google.protobuf.ByteString bs =
            (com.google.protobuf.ByteString) ref;
        java.lang.String s = bs.toStringUtf8();
        outRefundNo_ = s;
        return s;
      } else {
        return (java.lang.String) ref;
      }
    }
    /**
     * <code>string out_refund_no = 1;</code>
     * @return The bytes for outRefundNo.
     */
    public com.google.protobuf.ByteString
        getOutRefundNoBytes() {
      java.lang.Object ref = outRefundNo_;
      if (ref instanceof String) {
        com.google.protobuf.ByteString b = 
            com.google.protobuf.ByteString.copyFromUtf8(
                (java.lang.String) ref);
        outRefundNo_ = b;
        return b;
      } else {
        return (com.google.protobuf.ByteString) ref;
      }
    }
    /**
     * <code>string out_refund_no = 1;</code>
     * @param value The outRefundNo to set.
     * @return This builder for chaining.
     */
    public Builder setOutRefundNo(
        java.lang.String value) {
      if (value == null) { throw new NullPointerException(); }
      outRefundNo_ = value;
      bitField0_ |= 0x00000001;
      onChanged();
      return this;
    }
    /**
     * <code>string out_refund_no = 1;</code>
     * @return This builder for chaining.
     */
    public Builder clearOutRefundNo() {
      outRefundNo_ = getDefaultInstance().getOutRefundNo();
      bitField0_ = (bitField0_ & ~0x00000001);
      onChanged();
      return this;
    }
    /**
     * <code>string out_refund_no = 1;</code>
     * @param value The bytes for outRefundNo to set.
     * @return This builder for chaining.
     */
    public Builder setOutRefundNoBytes(
        com.google.protobuf.ByteString value) {
      if (value == null) { throw new NullPointerException(); }
      checkByteStringIsUtf8(value);
      outRefundNo_ = value;
      bitField0_ |= 0x00000001;
      onChanged();
      return this;
    }

    private java.lang.Object channel_ = "";
    /**
     * <code>string channel = 2;</code>
     * @return The channel.
     */
    public java.lang.String getChannel() {
      java.lang.Object ref = channel_;
      if (!(ref instanceof java.lang.String)) {
        com.google.protobuf.ByteString bs =
            (com.google.protobuf.ByteString) ref;
        java.lang.String s = bs.toStringUtf8();
        channel_ = s;
        return s;
      } else {
        return (java.lang.String) ref;
      }
    }
    /**
     * <code>string channel = 2;</code>
     * @return The bytes for channel.
     */
    public com.google.protobuf.ByteString
        getChannelBytes() {
      java.lang.Object ref = channel_;
      if (ref instanceof String) {
        com.google.protobuf.ByteString b = 
            com.google.protobuf.ByteString.copyFromUtf8(
                (java.lang.String) ref);
        channel_ = b;
        return b;
      } else {
        return (com.google.protobuf.ByteString) ref;
      }
    }
    /**
     * <code>string channel = 2;</code>
     * @param value The channel to set.
     * @return This builder for chaining.
     */
    public Builder setChannel(
        java.lang.String value) {
      if (value == null) { throw new NullPointerException(); }
      channel_ = value;
      bitField0_ |= 0x00000002;
      onChanged();
      return this;
    }
    /**
     * <code>string channel = 2;</code>
     * @return This builder for chaining.
     */
    public Builder clearChannel() {
      channel_ = getDefaultInstance().getChannel();
      bitField0_ = (bitField0_ & ~0x00000002);
      onChanged();
      return this;
    }
    /**
     * <code>string channel = 2;</code>
     * @param value The bytes for channel to set.
     * @return This builder for chaining.
     */
    public Builder setChannelBytes(
        com.google.protobuf.ByteString value) {
      if (value == null) { throw new NullPointerException(); }
      checkByteStringIsUtf8(value);
      channel_ = value;
      bitField0_ |= 0x00000002;
      onChanged();
      return this;
    }

    private java.lang.Object status_ = "";
    /**
     * <code>string status = 3;</code>
     * @return The status.
     */
    public java.lang.String getStatus() {
      java.lang.Object ref = status_;
      if (!(ref instanceof java.lang.String)) {
        com.google.protobuf.ByteString bs =
            (com.google.protobuf.ByteString) ref;
        java.lang.String s = bs.toStringUtf8();
        status_ = s;
        return s;
      } else {
        return (java.lang.String) ref;
      }
    }
    /**
     * <code>string status = 3;</code>
     * @return The bytes for status.
     */
    public com.google.protobuf.ByteString
        getStatusBytes() {
      java.lang.Object ref = status_;
      if (ref instanceof String) {
        com.google.protobuf.ByteString b = 
            com.google.protobuf.ByteString.copyFromUtf8(
                (java.lang.String) ref);
        status_ = b;
        return b;
      } else {
        return (com.google.protobuf.ByteString) ref;
      }
    }
    /**
     * <code>string status = 3;</code>
     * @param value The status to set.
     * @return This builder for chaining.
     */
    public Builder setStatus(
        java.lang.String value) {
      if (value == null) { throw new NullPointerException(); }
      status_ = value;
      bitField0_ |= 0x00000004;
      onChanged();
      return this;
    }
    /**
     * <code>string status = 3;</code>
     * @return This builder for chaining.
     */
    public Builder clearStatus() {
      status_ = getDefaultInstance().getStatus();
      bitField0_ = (bitField0_ & ~0x00000004);
      onChanged();
      return this;
    }
    /**
     * <code>string status = 3;</code>
     * @param value The bytes for status to set.
     * @return This builder for chaining.
     */
    public Builder setStatusBytes(
        com.google.protobuf.ByteString value) {
      if (value == null) { throw new NullPointerException(); }
      checkByteStringIsUtf8(value);
      status_ = value;
      bitField0_ |= 0x00000004;
      onChanged();
      return this;
    }

    private java.lang.Object userReceivedAccount_ = "";
    /**
     * <code>string user_received_account = 4;</code>
     * @return The userReceivedAccount.
     */
    public java.lang.String getUserReceivedAccount() {
      java.lang.Object ref = userReceivedAccount_;
      if (!(ref instanceof java.lang.String)) {
        com.google.protobuf.ByteString bs =
            (com.google.protobuf.ByteString) ref;
        java.lang.String s = bs.toStringUtf8();
        userReceivedAccount_ = s;
        return s;
      } else {
        return (java.lang.String) ref;
      }
    }
    /**
     * <code>string user_received_account = 4;</code>
     * @return The bytes for userReceivedAccount.
     */
    public com.google.protobuf.ByteString
        getUserReceivedAccountBytes() {
      java.lang.Object ref = userReceivedAccount_;
      if (ref instanceof String) {
        com.google.protobuf.ByteString b = 
            com.google.protobuf.ByteString.copyFromUtf8(
                (java.lang.String) ref);
        userReceivedAccount_ = b;
        return b;
      } else {
        return (com.google.protobuf.ByteString) ref;
      }
    }
    /**
     * <code>string user_received_account = 4;</code>
     * @param value The userReceivedAccount to set.
     * @return This builder for chaining.
     */
    public Builder setUserReceivedAccount(
        java.lang.String value) {
      if (value == null) { throw new NullPointerException(); }
      userReceivedAccount_ = value;
      bitField0_ |= 0x00000008;
      onChanged();
      return this;
    }
    /**
     * <code>string user_received_account = 4;</code>
     * @return This builder for chaining.
     */
    public Builder clearUserReceivedAccount() {
      userReceivedAccount_ = getDefaultInstance().getUserReceivedAccount();
      bitField0_ = (bitField0_ & ~0x00000008);
      onChanged();
      return this;
    }
    /**
     * <code>string user_received_account = 4;</code>
     * @param value The bytes for userReceivedAccount to set.
     * @return This builder for chaining.
     */
    public Builder setUserReceivedAccountBytes(
        com.google.protobuf.ByteString value) {
      if (value == null) { throw new NullPointerException(); }
      checkByteStringIsUtf8(value);
      userReceivedAccount_ = value;
      bitField0_ |= 0x00000008;
      onChanged();
      return this;
    }

    private java.lang.Object createTime_ = "";
    /**
     * <code>string create_time = 99;</code>
     * @return The createTime.
     */
    public java.lang.String getCreateTime() {
      java.lang.Object ref = createTime_;
      if (!(ref instanceof java.lang.String)) {
        com.google.protobuf.ByteString bs =
            (com.google.protobuf.ByteString) ref;
        java.lang.String s = bs.toStringUtf8();
        createTime_ = s;
        return s;
      } else {
        return (java.lang.String) ref;
      }
    }
    /**
     * <code>string create_time = 99;</code>
     * @return The bytes for createTime.
     */
    public com.google.protobuf.ByteString
        getCreateTimeBytes() {
      java.lang.Object ref = createTime_;
      if (ref instanceof String) {
        com.google.protobuf.ByteString b = 
            com.google.protobuf.ByteString.copyFromUtf8(
                (java.lang.String) ref);
        createTime_ = b;
        return b;
      } else {
        return (com.google.protobuf.ByteString) ref;
      }
    }
    /**
     * <code>string create_time = 99;</code>
     * @param value The createTime to set.
     * @return This builder for chaining.
     */
    public Builder setCreateTime(
        java.lang.String value) {
      if (value == null) { throw new NullPointerException(); }
      createTime_ = value;
      bitField0_ |= 0x00000010;
      onChanged();
      return this;
    }
    /**
     * <code>string create_time = 99;</code>
     * @return This builder for chaining.
     */
    public Builder clearCreateTime() {
      createTime_ = getDefaultInstance().getCreateTime();
      bitField0_ = (bitField0_ & ~0x00000010);
      onChanged();
      return this;
    }
    /**
     * <code>string create_time = 99;</code>
     * @param value The bytes for createTime to set.
     * @return This builder for chaining.
     */
    public Builder setCreateTimeBytes(
        com.google.protobuf.ByteString value) {
      if (value == null) { throw new NullPointerException(); }
      checkByteStringIsUtf8(value);
      createTime_ = value;
      bitField0_ |= 0x00000010;
      onChanged();
      return this;
    }
    @java.lang.Override
    public final Builder setUnknownFields(
        final com.google.protobuf.UnknownFieldSet unknownFields) {
      return super.setUnknownFields(unknownFields);
    }

    @java.lang.Override
    public final Builder mergeUnknownFields(
        final com.google.protobuf.UnknownFieldSet unknownFields) {
      return super.mergeUnknownFields(unknownFields);
    }


    // @@protoc_insertion_point(builder_scope:palm.musa.v1.WechatPayRefundResponse)
  }

  // @@protoc_insertion_point(class_scope:palm.musa.v1.WechatPayRefundResponse)
  private static final com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse DEFAULT_INSTANCE;
  static {
    DEFAULT_INSTANCE = new com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse();
  }

  public static com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static final com.google.protobuf.Parser<WechatPayRefundResponse>
      PARSER = new com.google.protobuf.AbstractParser<WechatPayRefundResponse>() {
    @java.lang.Override
    public WechatPayRefundResponse parsePartialFrom(
        com.google.protobuf.CodedInputStream input,
        com.google.protobuf.ExtensionRegistryLite extensionRegistry)
        throws com.google.protobuf.InvalidProtocolBufferException {
      Builder builder = newBuilder();
      try {
        builder.mergeFrom(input, extensionRegistry);
      } catch (com.google.protobuf.InvalidProtocolBufferException e) {
        throw e.setUnfinishedMessage(builder.buildPartial());
      } catch (com.google.protobuf.UninitializedMessageException e) {
        throw e.asInvalidProtocolBufferException().setUnfinishedMessage(builder.buildPartial());
      } catch (java.io.IOException e) {
        throw new com.google.protobuf.InvalidProtocolBufferException(e)
            .setUnfinishedMessage(builder.buildPartial());
      }
      return builder.buildPartial();
    }
  };

  public static com.google.protobuf.Parser<WechatPayRefundResponse> parser() {
    return PARSER;
  }

  @java.lang.Override
  public com.google.protobuf.Parser<WechatPayRefundResponse> getParserForType() {
    return PARSER;
  }

  @java.lang.Override
  public com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayRefundResponse getDefaultInstanceForType() {
    return DEFAULT_INSTANCE;
  }

}

