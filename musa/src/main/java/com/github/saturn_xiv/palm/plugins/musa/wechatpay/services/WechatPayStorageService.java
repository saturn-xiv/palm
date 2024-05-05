package com.github.saturn_xiv.palm.plugins.musa.wechatpay.services;

import com.github.saturn_xiv.palm.plugins.musa.v1.wechat_pay.*;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.models.FundFlowBill;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.models.TradeBill;

public interface WechatPayStorageService {
    void addFundFlowBill(String billDate, FundFlowAccountType accountType, TarType tarType, byte[] content);

    void addTradeBill(String billDate, TradeBillType billType, TarType tarType, byte[] content);

    FundFlowBill getFundFlowBill(String billDate, FundFlowAccountType accountType);

    TradeBill getTradeBill(String billDate, TradeBillType billType);

    void addOrder(String appId, String payerOpenId, String outTradeNo, Amount amount, String description, String response);

    void addRefund(String outTradeNo, String outRefundNo, CreateRefundAmount amount, String reason);


    void addNotification(com.wechat.pay.java.core.notification.Notification notification, String resource);

    void acceptBatchTransferReceipts(String outBatchNo, String... outDetailNos);

    void finishBatchTransferReceipt(String outBatchNo, byte[] content);

    void finishDetailElectronicReceipt(String acceptType, String outBatchNo, String outDetailNo, byte[] content);
}
