// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: musa.proto

// Protobuf Java Version: 3.25.1
package com.github.saturn_xiv.palm.plugins.musa.v1;

public interface WechatPayQueryBatchTransferRequestOrBuilder extends
    // @@protoc_insertion_point(interface_extends:palm.musa.v1.WechatPayQueryBatchTransferRequest)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <code>string out_batch_no = 1;</code>
   * @return The outBatchNo.
   */
  java.lang.String getOutBatchNo();
  /**
   * <code>string out_batch_no = 1;</code>
   * @return The bytes for outBatchNo.
   */
  com.google.protobuf.ByteString
      getOutBatchNoBytes();

  /**
   * <code>int32 offset = 2;</code>
   * @return The offset.
   */
  int getOffset();

  /**
   * <code>int32 limit = 3;</code>
   * @return The limit.
   */
  int getLimit();

  /**
   * <code>.palm.musa.v1.WechatPayQueryBatchTransferRequest.DetailStatus detail_status = 9;</code>
   * @return The enum numeric value on the wire for detailStatus.
   */
  int getDetailStatusValue();
  /**
   * <code>.palm.musa.v1.WechatPayQueryBatchTransferRequest.DetailStatus detail_status = 9;</code>
   * @return The detailStatus.
   */
  com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayQueryBatchTransferRequest.DetailStatus getDetailStatus();
}
