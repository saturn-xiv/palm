package com.github.saturn_xiv.palm.plugins.musa.wechatpay.tasks;

import com.github.saturn_xiv.palm.plugins.musa.v1.wechat_pay.FundFlowAccountType;
import com.github.saturn_xiv.palm.plugins.musa.v1.wechat_pay.TarType;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.WechatPayClient;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.services.WechatPayStorageService;
import com.wechat.pay.java.core.exception.ServiceException;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.scheduling.annotation.Scheduled;
import org.springframework.stereotype.Component;

import java.time.Duration;

@Component("palm.musa.task.wechat-pay.downloader.fund-flow-bill")
public class WechatPayFundFlowBillDownloader {
    @Scheduled(cron = "${app.wechatpay.fund-flow-bill.downloader-cron}")
    public void execute() throws InterruptedException {
        logger.info("start wechat-pay download fund flow bills");
        for (var billDate : WechatPayClient.latestBillDates()) {
            for (var accountType : FundFlowAccountType.values()) {

                logger.info("download wechat-pay fund flow bills ({},{})", billDate, accountType);
                if (wechatPayStorageService.getFundFlowBill(billDate, accountType) == null) {
                    try {
                        final var content = wechatPayClient.downloadFundFlowBill(billDate, WechatPayClient.accountType(accountType));
                        wechatPayStorageService.addFundFlowBill(billDate, accountType, TarType.GZIP, content);
                    } catch (ServiceException e) {
                        logger.error("{} {} {}", e.getHttpStatusCode(), e.getErrorCode(), e.getErrorMessage());
                    }
                }

                Thread.sleep(Duration.ofMinutes(12));
            }
        }
    }

    @Autowired
    WechatPayStorageService wechatPayStorageService;
    @Autowired
    WechatPayClient wechatPayClient;
    private final static Logger logger = LoggerFactory.getLogger(WechatPayFundFlowBillDownloader.class);
}
