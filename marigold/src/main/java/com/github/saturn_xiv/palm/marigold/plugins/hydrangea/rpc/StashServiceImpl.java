package com.github.saturn_xiv.palm.marigold.plugins.hydrangea.rpc;

import org.springframework.grpc.server.service.GrpcService;

import com.github.saturn_xiv.palm.plugins.hydrangea.v1.StashJobRequest;
import com.github.saturn_xiv.palm.plugins.hydrangea.v1.StashServiceGrpc.StashServiceImplBase;
import com.google.protobuf.Empty;

import io.grpc.stub.StreamObserver;

@GrpcService
public class StashServiceImpl extends StashServiceImplBase {

    @Override
    public void job(StashJobRequest request, StreamObserver<Empty> responseObserver) {
        // TODO Auto-generated method stub
        super.job(request, responseObserver);
    }

}
