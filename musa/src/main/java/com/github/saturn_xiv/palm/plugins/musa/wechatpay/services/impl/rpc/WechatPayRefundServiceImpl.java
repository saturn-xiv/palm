package com.github.saturn_xiv.palm.plugins.musa.wechatpay.services.impl.rpc;

import com.github.saturn_xiv.palm.plugins.musa.v1.wechat_pay.CreateRefundAmount;
import com.github.saturn_xiv.palm.plugins.musa.v1.wechat_pay.NotifyAction;
import com.github.saturn_xiv.palm.plugins.musa.v1.wechat_pay.QueryRefundResponse;
import com.github.saturn_xiv.palm.plugins.musa.v1.wechat_pay.Refund;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.WechatPayClient;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.helpers.WechatPayRefundHelper;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.models.OutNoType;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.services.WechatPayStorageService;
import jakarta.annotation.PostConstruct;
import org.apache.thrift.TException;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Component;

@Component("palm.musa.service.rpc.wechat-pay.refund")
public class WechatPayRefundServiceImpl implements Refund.Iface {

    @Override
    public QueryRefundResponse create(String out_trade_no, CreateRefundAmount amount, String reason, String notify_host) throws TException {
        final var outRefundNo = WechatPayClient.outNo(OutNoType.REFUND);
        final var notifyUrl = WechatPayClient.notifyUrl(notify_host, NotifyAction.REFUND);
        final var currency = WechatPayClient.currency(amount.getCurrency());

        logger.info("create refund {}", outRefundNo);
        final var response = refundHelper.create(out_trade_no, outRefundNo,
                amount.getRefund(), amount.getTotal(), currency,
                reason, notifyUrl);
        storageService.addRefund(out_trade_no, outRefundNo, amount, reason);
        var it = new QueryRefundResponse();
        it.setOut_refund_no(response.getOutRefundNo());
        it.setChannel(response.getChannel().name());
        it.setUser_received_account(response.getUserReceivedAccount());
        it.setStatus(response.getStatus().name());
        it.setCreate_time(response.getCreateTime());
        return it;
    }

    @Override
    public QueryRefundResponse query(String out_refund_no) throws TException {
        logger.info("query refund out-refund-no {}", out_refund_no);
        final var response = refundHelper.queryByOutRefundNo(out_refund_no);

        var it = new QueryRefundResponse();
        it.setOut_refund_no(response.getOutRefundNo());
        it.setChannel(response.getChannel().name());
        it.setUser_received_account(response.getUserReceivedAccount());
        it.setStatus(response.getStatus().name());
        it.setCreate_time(response.getCreateTime());
        return it;

    }

    @PostConstruct
    void startUp() {
        refundHelper = new WechatPayRefundHelper(client.refundService());
    }

    @Autowired
    WechatPayClient client;
    @Autowired
    WechatPayStorageService storageService;

    private WechatPayRefundHelper refundHelper;

    private final static Logger logger = LoggerFactory.getLogger(WechatPayRefundServiceImpl.class);
}
