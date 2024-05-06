package com.github.saturn_xiv.palm.plugins.musa.wechatpay.services.impl.rpc;

import com.github.saturn_xiv.palm.plugins.musa.v1.wechat_pay.*;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.WechatPayClient;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.helpers.WechatPayBatchTransferHelper;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.models.OutNoType;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.models.transfer.ReceiptSignatureStatus;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.models.transfer.TransferBillReceipt;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.repositories.WechatPayTransferBillReceiptRepository;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.repositories.WechatPayTransferDetailElectronicReceiptRepository;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.services.WechatPayStorageService;
import com.wechat.pay.java.core.exception.ServiceException;
import com.wechat.pay.java.service.transferbatch.model.TransferDetailInput;
import jakarta.annotation.PostConstruct;
import org.apache.thrift.TException;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Component;

import java.nio.ByteBuffer;
import java.util.ArrayList;
import java.util.Date;
import java.util.List;

@Component("palm.musa.service.rpc.wechat-pay.transfer")
public class WechatPayTransferServiceImpl implements Transfer.Iface {
    @Override
    public ExecuteTransferBatchResponse execute_batch(String app_id, String out_batch_no, String name, String remark, List<ExecuteTransferBatchRequestDetail> details, String scene_id) throws TException {
        List<ExecuteTransferBatchResponseDetail> transferDetailList = new ArrayList<>();
        List<TransferDetailInput> transferDetailInputList = new ArrayList<>();
        long totalAmount = 0;
        for (var it : details) {
            final var outDetailsNo = WechatPayClient.outNo(OutNoType.BATCH_TRANSFER_DETAIL);
            totalAmount += it.getAmount();
            {
                var tdi = new TransferDetailInput();
                tdi.setTransferAmount(it.getAmount());
                tdi.setTransferRemark(it.getRemark());
                tdi.setOutDetailNo(outDetailsNo);
                tdi.setOpenid(it.getOpen_id());
                tdi.setUserName(it.getUsername());
                logger.debug("add detail ({},{}) {} {} {}",
                        it.getUsername(), it.getOpen_id(), outDetailsNo, it.getAmount(), it.getRemark());
                transferDetailInputList.add(tdi);
            }
            var detail = new ExecuteTransferBatchResponseDetail();
            detail.setOpen_id(it.getOpen_id());
            detail.setOut_detail_no(outDetailsNo);
            transferDetailList.add(detail);
        }

        logger.info("execute transfer {} for {} with amount {}", out_batch_no, name, totalAmount);

        var it = new ExecuteTransferBatchResponse();
        it.setDetails(transferDetailList);

        var status = new ExecuteTransferBatchResponseStatus();
        try {
            final var response = transferBatchHelper.create(app_id, out_batch_no, name, remark, totalAmount, transferDetailInputList.size(), transferDetailInputList, scene_id);

            logger.info("{} {} {} {}", response.getBatchStatus(), response.getBatchId(), response.getOutBatchNo(), response.getCreateTime());

            it.setOut_batch_no(response.getOutBatchNo());

            var succeeded = new ExecuteTransferBatchResponseSucceeded();
            succeeded.setCreate_time(response.getCreateTime());
            succeeded.setBatch_id(response.getBatchId());
            status.setSucceeded(succeeded);

        } catch (ServiceException e) {
            logger.error("{} {} {}", e.getHttpStatusCode(), e.getErrorCode(), e.getErrorMessage());
            it.setOut_batch_no(out_batch_no);
            var error = new com.github.saturn_xiv.palm.plugins.musa.v1.wechat_pay.ResponseError();
            error.setCode(e.getErrorCode());
            error.setMessage(e.getErrorMessage());
            status.setError(error);
        }

        it.setStatus(status);
        return it;
    }

    @Override
    public ExecuteTransferBatchResponse create_batch(String app_id, String name, String remark, List<ExecuteTransferBatchRequestDetail> details, String scene_id) throws TException {
        final var outBatchNo = WechatPayClient.outNo(OutNoType.BATCH_TRANSFER);
        logger.info("put batch transfer {} into bill receipt queue", outBatchNo);
        var item = new TransferBillReceipt();
        item.setOutBatchNo(outBatchNo);
        item.setSignatureStatus(ReceiptSignatureStatus.PENDING);
        final var now = new Date();
        item.setUpdatedAt(now);
        item.setCreatedAt(now);
        transferBillReceiptRepository.save(item);
        return this.execute_batch(app_id, outBatchNo, name, remark, details, scene_id);
    }

    @Override
    public QueryTransferBatchResponse query_batch(String out_batch_no, QueryBatchTransferDetailStatus detail_status, int offset, int limit) throws TException {
        logger.info("query batch transfer {} ({}, {})", out_batch_no, offset, limit);
        final var response = transferBatchHelper.query(
                out_batch_no, offset, limit,
                WechatPayClient.batchTransferDetailStatus(detail_status)
        );


        List<QueryTransferBatchResponseDetail> transferDetailList = new ArrayList<>();
        for (var it : response.getTransferDetailList()) {
            var detail = new QueryTransferBatchResponseDetail();
            detail.setDetail_id(it.getDetailId());
            detail.setOut_detail_no(it.getOutDetailNo());
            detail.setStatus(it.getDetailStatus());
            transferDetailList.add(detail);
        }

        var it = new QueryTransferBatchResponse();
        it.setMerchant_id(response.getTransferBatch().getMchid());
        it.setOut_batch_no(response.getTransferBatch().getOutBatchNo());
        it.setBatch_id(response.getTransferBatch().getBatchId());
        it.setApp_id(response.getTransferBatch().getAppid());
        it.setBatch_status(response.getTransferBatch().getBatchStatus());
        it.setBatch_type(response.getTransferBatch().getBatchType());
        it.setBatch_name(response.getTransferBatch().getBatchName());
        it.setBatch_remark(response.getTransferBatch().getBatchRemark());
        it.setTotal_amount(response.getTransferBatch().getTotalAmount());
        it.setTotal_num(response.getTransferBatch().getTotalNum());
        it.setDetails(transferDetailList);

        if (response.getTransferBatch().getCloseReason() != null) {
            it.setClose_reason(response.getTransferBatch().getCloseReason().name());
        }
        if (response.getTransferBatch().getCreateTime() != null) {
            it.setCreate_time(response.getTransferBatch().getCreateTime());
        }
        if (response.getTransferBatch().getUpdateTime() != null) {
            it.setUpdate_time(response.getTransferBatch().getUpdateTime());
        }
        if (response.getTransferBatch().getSuccessAmount() != null) {
            it.setSuccess_amount(response.getTransferBatch().getSuccessAmount());
        }
        if (response.getTransferBatch().getSuccessNum() != null) {
            it.setSuccess_num(response.getTransferBatch().getSuccessNum());
        }
        if (response.getTransferBatch().getFailAmount() != null) {
            it.setFail_amount(response.getTransferBatch().getFailAmount());
        }
        if (response.getTransferBatch().getFailNum() != null) {
            it.setFail_num(response.getTransferBatch().getFailNum());
        }

        return it;
    }

    @Override
    public QueryTransferDetailResponse query_detail(String out_batch_no, String out_detail_no) throws TException {
        logger.info("query transfer detail {} ({})", out_batch_no, out_detail_no);
        final var response = transferBatchHelper.query(out_batch_no, out_detail_no);

        var it = new QueryTransferDetailResponse();
        it.setMerchant_id(response.getMchid());
        it.setOut_batch_no(response.getOutBatchNo());
        it.setBatch_id(response.getBatchId());
        it.setApp_id(response.getAppid());
        it.setOut_detail_no(response.getOutDetailNo());
        it.setDetail_id(response.getDetailId());
        it.setDetail_status(response.getDetailStatus());
        it.setTransfer_amount(response.getTransferAmount());
        it.setTransfer_remark(response.getTransferRemark());
        it.setOpen_id(response.getOpenid());


//        TODO initial-time & update-time may be null
        if (response.getInitiateTime() != null) {
            it.setInitiate_time(response.getInitiateTime());
        }
        if (response.getUpdateTime() != null) {
            it.setUpdate_time(response.getUpdateTime());
        }

        if (response.getFailReason() != null) {
            it.setFail_reason(response.getFailReason().name());
        }
        if (response.getUserName() != null) {
            it.setUser_name(response.getUserName());
        }

        return it;
    }

    @Override
    public ByteBuffer get_bill_receipt(String out_batch_no) throws TException {
        final var it = transferBillReceiptRepository.findByOutBatchNo(out_batch_no);

        if (it == null) {
            logger.warn("couldn't find transfer {}", out_batch_no);
            var item = new TransferBillReceipt();
            item.setOutBatchNo(out_batch_no);
            item.setSignatureStatus(ReceiptSignatureStatus.PENDING);
            final var now = new Date();
            item.setUpdatedAt(now);
            item.setCreatedAt(now);
            transferBillReceiptRepository.save(item);
            throw new TException("not found, put it into task queue, please check it later");
        }

        if (it.getSignatureStatus() == ReceiptSignatureStatus.FINISHED) {
            return ByteBuffer.wrap(it.getContent());
        }

        logger.error("batch bill({}) status {}", out_batch_no, it.getSignatureStatus());
        throw new TException("please wait another 30 minutes");
    }

    @Override
    public ByteBuffer get_electronic_receipt(TransferElectronicReceiptAcceptType accept_type, String out_batch_no, String out_detail_no) throws TException {
        final var it = transferDetailElectronicReceiptsRepository.findByOutBatchNoAndOutDetailNoAndAcceptType(
                out_batch_no,
                out_detail_no,
                WechatPayClient.transferDetailElectronicReceiptAcceptType(accept_type)
        );

        return ByteBuffer.wrap(it.getContent());
    }

    @PostConstruct
    void startUp() {
        transferBatchHelper = new WechatPayBatchTransferHelper(client.batchTransferService());
    }


    @Autowired
    WechatPayClient client;
    @Autowired
    WechatPayStorageService storageService;
    @Autowired
    WechatPayTransferBillReceiptRepository transferBillReceiptRepository;
    @Autowired
    WechatPayTransferDetailElectronicReceiptRepository transferDetailElectronicReceiptsRepository;

    private WechatPayBatchTransferHelper transferBatchHelper;
    private final static Logger logger = LoggerFactory.getLogger(WechatPayTransferServiceImpl.class);
}
