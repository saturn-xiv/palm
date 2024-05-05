package com.github.saturn_xiv.palm.plugins.musa.wechatpay.services.impl.rpc2;

import com.github.saturn_xiv.palm.plugins.musa.wechatpay.WechatPayClient;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.helpers.WechatPayBatchTransferHelper;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.models.OutNoType;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.models.transfer.ReceiptSignatureStatus;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.models.transfer.TransferBillReceipt;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.repositories.WechatPayTransferBillReceiptRepository;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.repositories.WechatPayTransferDetailElectronicReceiptRepository;
import com.github.saturn_xiv.palm.plugins.musa.wechatpay.services.WechatPayStorageService;
import com.google.protobuf.ByteString;
import com.wechat.pay.java.core.exception.ServiceException;
import com.wechat.pay.java.service.transferbatch.model.TransferDetailInput;
import io.grpc.Status;
import io.grpc.stub.StreamObserver;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Component;

import javax.annotation.PostConstruct;
import java.util.ArrayList;
import java.util.Date;
import java.util.List;

@Component("palm.musa.service.rpc.wechat-pay.transfer")
public class WechatPayTransferServiceImpl extends WechatPayTransferGrpc.WechatPayTransferImplBase {
    @Override
    public void getBillReceipt(WechatPayTransferGetBillReceiptRequest request,
                               StreamObserver<WechatPayTransferGetReceiptResponse> responseObserver) {
        final var it = transferBillReceiptRepository.findByOutBatchNo(request.getOutBatchNo());

        if (it == null) {
            logger.warn("couldn't find transfer {}", request.getOutBatchNo());
            var item = new TransferBillReceipt();
            item.setOutBatchNo(request.getOutBatchNo());
            item.setSignatureStatus(ReceiptSignatureStatus.PENDING);
            final var now = new Date();
            item.setUpdatedAt(now);
            item.setCreatedAt(now);
            transferBillReceiptRepository.save(item);
            responseObserver.onError(
                    Status.NOT_FOUND
                            .withDescription("not found, put it into task queue, please check it later")
                            .asRuntimeException());
            return;
        }

        if (it.getSignatureStatus() == ReceiptSignatureStatus.FINISHED) {
            responseObserver.onNext(WechatPayTransferGetReceiptResponse.newBuilder()
                    .setPayload(ByteString.copyFrom(it.getContent()))
                    .build());
            responseObserver.onCompleted();
            return;
        }

        responseObserver.onError(
                Status.UNAVAILABLE.withDescription("please wait another 30 minutes").asRuntimeException());
    }

    @Override
    public void getElectronicReceipt(WechatPayTransferGetElectronicReceiptRequest request,
                                     StreamObserver<WechatPayTransferGetReceiptResponse> responseObserver) {
        final var it = transferDetailElectronicReceiptsRepository.findByOutBatchNoAndOutDetailNoAndAcceptType(
                request.getOutBatchNo(),
                request.getOutDetailNo(),
                WechatPayClient.transferDetailElectronicReceiptAcceptType(request.getAcceptType())
        );
        responseObserver.onNext(WechatPayTransferGetReceiptResponse.newBuilder()
                .setPayload(ByteString.copyFrom(it.getContent()))
                .build());
        responseObserver.onCompleted();
    }

    @Override
    public void executeBatch(WechatPayExecuteBatchTransferRequest request,
                             StreamObserver<WechatPayExecuteBatchTransferResponse> responseObserver) {

        responseObserver.onCompleted();
    }

    @Override
    public void queryBatch(WechatPayQueryBatchTransferRequest request,
                           StreamObserver<WechatPayQueryBatchTransferResponse> responseObserver) {


    }

    @Override
    public void queryDetail(WechatPayQueryTransferDetailRequest request,
                            StreamObserver<WechatPayQueryTransferDetailResponse> responseObserver) {


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
