package com.github.saturn_xiv.palm.plugins.musa.wechatpay.services.impl.rpc;

import com.github.saturn_xiv.palm.plugins.musa.v1.wechat_pay.*;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.WechatPayClient;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.helpers.WechatPayJsapiHelper;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.models.OutNoType;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.services.WechatPayStorageService;
import com.google.gson.Gson;
import jakarta.annotation.PostConstruct;
import org.apache.thrift.TException;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Component;

@Component("palm.musa.service.rpc.wechat-pay.jsapi")
public class WechatPayJsapiServiceImpl implements Jsapi.Iface {
    @Override
    public JsapiPrepayIdResponse create_prepay(PrepayRequest request) throws TException {
        final var outTradeNo = WechatPayClient.outNo(OutNoType.TRADE);
        return this.execute_prepay(outTradeNo, request);
    }

    @Override
    public JsapiPrepayIdResponse execute_prepay(String out_trade_no, PrepayRequest request) throws TException {
        final var notifyUrl = WechatPayClient.notifyUrl(request.getNotify_host(), NotifyAction.TRANSCATION);

        logger.info("prepay jsapi out-trade-no {}", out_trade_no);
        var response = wechatPay.prepayWithRequestPayment(request.getApp_id(),
                request.getPayer_open_id(),
                out_trade_no,
                WechatPayClient.currency(request.getAmount().getCurrency()),
                request.getAmount().getTotal(),
                request.getDescription(),
                notifyUrl);
        Gson gson = new Gson();
        storageService.addOrder(request.getApp_id(), request.getPayer_open_id(), out_trade_no, request.getAmount(),
                request.getDescription(), gson.toJson(response));

        var it = new JsapiPrepayIdResponse();
        it.setApp_id(response.getAppId());
        it.setOut_trade_no(out_trade_no);
        it.setTime_stamp(response.getTimeStamp());
        it.setNonce_str(response.getNonceStr());
        it.setPackage_(response.getPackageVal());
        it.setSign_type(response.getSignType());
        it.setPay_sign(response.getPaySign());
        return it;
    }

    @Override
    public TradeResponse query_order_by_out_trade_no(String out_trade_no) throws TException {
        logger.info("query order by out-trade-no {}", out_trade_no);
        final var response = wechatPay.queryOrderByOutTradeNo(out_trade_no);
        var it = new TradeResponse();
        it.setTrade_state_desc(response.getTradeStateDesc());
        it.setTrade_state(response.getTradeState().name());
        return it;
    }

    @Override
    public TradeResponse query_order_by_transaction_id(String transaction_id) throws TException {
        logger.info("query order by transaction id {}", transaction_id);
        final var response = wechatPay.queryOrderById(transaction_id);
        var it = new TradeResponse();
        it.setTrade_state_desc(response.getTradeStateDesc());
        it.setTrade_state(response.getTradeState().name());
        return it;
    }

    @Override
    public void close_order(String out_trade_no, String reason) throws TException {
        logger.warn("close order {}, reason: {}", out_trade_no, reason);
        wechatPay.closeOrder(out_trade_no);
    }


    @PostConstruct
    void init() {
        wechatPay = new WechatPayJsapiHelper(client.getMerchantId(), client.jsapiService());
    }


    @Autowired
    WechatPayClient client;
    @Autowired
    WechatPayStorageService storageService;

    private WechatPayJsapiHelper wechatPay;
    private final static Logger logger = LoggerFactory.getLogger(WechatPayJsapiServiceImpl.class);
}
