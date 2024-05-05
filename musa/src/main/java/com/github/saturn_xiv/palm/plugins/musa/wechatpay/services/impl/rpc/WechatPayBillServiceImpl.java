package com.github.saturn_xiv.palm.plugins.musa.wechatpay.services.impl.rpc;

import com.github.saturn_xiv.palm.plugins.musa.v1.wechat_pay.*;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.WechatPayClient;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.services.WechatPayStorageService;
import org.apache.thrift.TException;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Component;

import java.nio.ByteBuffer;

@Component("palm.musa.service.rpc.wechat-pay.bill")
public class WechatPayBillServiceImpl implements Bill.Iface {
    @Override
    public ByteBuffer trade(TradeBillType bill_type, BillDate bill_date) throws TException {
        final var billDate = WechatPayClient.billDate(bill_date);
        var it = storageService.getTradeBill(billDate, bill_type);

        if (it == null) {
            if (!WechatPayClient.canDownload(bill_date)) {
                logger.error("invalid trade bill date {}", billDate);
                throw new TException();
            }
            var content = client.downloadTradeBill(billDate, WechatPayClient.billType(bill_type));
            storageService.addTradeBill(billDate, bill_type, TarType.GZIP, content);
            return ByteBuffer.wrap(content);
        }
        return ByteBuffer.wrap(it.getContent());
    }

    @Override
    public ByteBuffer fund_flow(FundFlowAccountType account_type, BillDate bill_date) throws TException {
        final var billDate = WechatPayClient.billDate(bill_date);
        var it = storageService.getFundFlowBill(billDate, account_type);
        if (it == null) {
            logger.info("download fund-flow bill {}@{}", bill_date, account_type);
            if (!WechatPayClient.canDownload(bill_date)) {
                logger.error("invalid fund flow bill date {}", billDate);
                throw new TException();
            }
            var content = client.downloadFundFlowBill(billDate, WechatPayClient.accountType(account_type));
            storageService.addFundFlowBill(billDate, account_type, TarType.GZIP, content);
            return ByteBuffer.wrap(content);
        }
        return ByteBuffer.wrap(it.getContent());
    }

    @Autowired
    WechatPayClient client;
    @Autowired
    WechatPayStorageService storageService;

    private final static Logger logger = LoggerFactory.getLogger(WechatPayBillServiceImpl.class);
}
