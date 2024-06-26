// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: musa.proto

// Protobuf Java Version: 3.25.1
package com.github.saturn_xiv.palm.plugins.musa.v1;

public interface WechatPayPrepayRequestOrBuilder extends
    // @@protoc_insertion_point(interface_extends:palm.musa.v1.WechatPayPrepayRequest)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <code>string app_id = 1;</code>
   * @return The appId.
   */
  java.lang.String getAppId();
  /**
   * <code>string app_id = 1;</code>
   * @return The bytes for appId.
   */
  com.google.protobuf.ByteString
      getAppIdBytes();

  /**
   * <code>optional string out_trade_no = 2;</code>
   * @return Whether the outTradeNo field is set.
   */
  boolean hasOutTradeNo();
  /**
   * <code>optional string out_trade_no = 2;</code>
   * @return The outTradeNo.
   */
  java.lang.String getOutTradeNo();
  /**
   * <code>optional string out_trade_no = 2;</code>
   * @return The bytes for outTradeNo.
   */
  com.google.protobuf.ByteString
      getOutTradeNoBytes();

  /**
   * <code>optional string payer_open_id = 11;</code>
   * @return Whether the payerOpenId field is set.
   */
  boolean hasPayerOpenId();
  /**
   * <code>optional string payer_open_id = 11;</code>
   * @return The payerOpenId.
   */
  java.lang.String getPayerOpenId();
  /**
   * <code>optional string payer_open_id = 11;</code>
   * @return The bytes for payerOpenId.
   */
  com.google.protobuf.ByteString
      getPayerOpenIdBytes();

  /**
   * <code>.palm.musa.v1.WechatPayPrepayRequest.Amount amount = 12;</code>
   * @return Whether the amount field is set.
   */
  boolean hasAmount();
  /**
   * <code>.palm.musa.v1.WechatPayPrepayRequest.Amount amount = 12;</code>
   * @return The amount.
   */
  com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayPrepayRequest.Amount getAmount();
  /**
   * <code>.palm.musa.v1.WechatPayPrepayRequest.Amount amount = 12;</code>
   */
  com.github.saturn_xiv.palm.plugins.musa.v1.WechatPayPrepayRequest.AmountOrBuilder getAmountOrBuilder();

  /**
   * <code>string description = 98;</code>
   * @return The description.
   */
  java.lang.String getDescription();
  /**
   * <code>string description = 98;</code>
   * @return The bytes for description.
   */
  com.google.protobuf.ByteString
      getDescriptionBytes();

  /**
   * <code>string notify_host = 99;</code>
   * @return The notifyHost.
   */
  java.lang.String getNotifyHost();
  /**
   * <code>string notify_host = 99;</code>
   * @return The bytes for notifyHost.
   */
  com.google.protobuf.ByteString
      getNotifyHostBytes();
}
