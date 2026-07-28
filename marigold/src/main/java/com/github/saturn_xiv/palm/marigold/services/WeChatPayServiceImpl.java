package com.github.saturn_xiv.palm.marigold.services;

import org.springframework.grpc.server.service.GrpcService;

import com.github.saturn_xiv.palm.plugins.wechat_pay.v1.BillPullRequest;
import com.github.saturn_xiv.palm.plugins.wechat_pay.v1.BillPullResponse;
import com.github.saturn_xiv.palm.plugins.wechat_pay.v1.WeChatPayGrpc;

import io.grpc.stub.StreamObserver;

@GrpcService
public class WeChatPayServiceImpl extends WeChatPayGrpc.WeChatPayImplBase {

    @Override
    public void billPull(BillPullRequest request, StreamObserver<BillPullResponse> responseObserver) {
        // TODO
        super.billPull(request, responseObserver);
    }

}
