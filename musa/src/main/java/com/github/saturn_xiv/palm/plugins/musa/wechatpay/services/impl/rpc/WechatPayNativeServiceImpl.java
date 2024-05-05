package com.github.saturn_xiv.palm.plugins.musa.wechatpay.services.impl.rpc;

import com.github.saturn_xiv.palm.plugins.musa.v1.wechat_pay.Native;
import com.github.saturn_xiv.palm.plugins.musa.v1.wechat_pay.NativeQrCodeUrlResponse;
import com.github.saturn_xiv.palm.plugins.musa.v1.wechat_pay.NotifyAction;
import com.github.saturn_xiv.palm.plugins.musa.v1.wechat_pay.PrepayRequest;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.WechatPayClient;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.helpers.WechatPayNativeHelper;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.models.OutNoType;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.services.WechatPayStorageService;
import jakarta.annotation.PostConstruct;
import org.apache.thrift.TException;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Component;

@Component("palm.musa.service.rpc.wechat-pay.native")
public class WechatPayNativeServiceImpl implements Native.Iface {
    @Override
    public NativeQrCodeUrlResponse create_prepay(PrepayRequest request) throws TException {
        final var outTradeNo = WechatPayClient.outNo(OutNoType.TRADE);
        return this.execute_prepay(outTradeNo, request);
    }

    @Override
    public NativeQrCodeUrlResponse execute_prepay(String out_trade_no, PrepayRequest request) throws TException {
        final var notifyUrl = WechatPayClient.notifyUrl(request.getNotify_host(), NotifyAction.TRANSCATION);
        logger.info("prepay native out-trade-no {}", out_trade_no);
        var currency = WechatPayClient.currency(request.getAmount().getCurrency());
        var url = wechatPay.prepay(request.getApp_id(),
                request.getDescription(),
                out_trade_no,
                currency, request.getAmount().getTotal(),
                notifyUrl);
        var it = new NativeQrCodeUrlResponse();
        it.setUrl(url);
        it.setOut_trade_no(out_trade_no);
        return it;
    }

    @PostConstruct
    void init() {
        wechatPay = new WechatPayNativeHelper(client.getMerchantId(), client.nativePayService());
    }


    @Autowired
    WechatPayClient client;
    @Autowired
    WechatPayStorageService storageService;

    private WechatPayNativeHelper wechatPay;

    private final static Logger logger = LoggerFactory.getLogger(WechatPayNativeServiceImpl.class);
}
