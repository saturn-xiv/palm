package com.github.saturn_xiv.palm.marigold.plugins.portal.rpc;

import com.github.saturn_xiv.palm.plugins.wechat_pay.v1.BillPullRequest;
import com.github.saturn_xiv.palm.plugins.wechat_pay.v1.BillPullResponse;
import com.github.saturn_xiv.palm.plugins.wechat_pay.v1.WechatPayServiceGrpc.WechatPayServiceImplBase;

import io.grpc.stub.StreamObserver;
import org.springframework.grpc.server.service.GrpcService;

@GrpcService
public class WechatPayServiceImpl
        extends WechatPayServiceImplBase {

    @Override
    public void billPull(
            BillPullRequest request,
            StreamObserver<BillPullResponse> responseObserver) {
        // TODO
        super.billPull(request, responseObserver);
    }

}
