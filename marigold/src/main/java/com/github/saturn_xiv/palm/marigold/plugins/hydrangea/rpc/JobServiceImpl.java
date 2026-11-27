package com.github.saturn_xiv.palm.marigold.plugins.hydrangea.rpc;

import java.io.IOException;

import org.springframework.grpc.server.service.GrpcService;

import com.github.saturn_xiv.palm.marigold.etc.ClientIpInterceptor;
import com.github.saturn_xiv.palm.marigold.plugins.hydrangea.utils.JobHelper;
import com.github.saturn_xiv.palm.plugins.hydrangea.v1.JobPushRequest;
import com.github.saturn_xiv.palm.plugins.hydrangea.v1.JobServiceGrpc.JobServiceImplBase;
import com.google.protobuf.Empty;

import io.grpc.Status;
import io.grpc.stub.StreamObserver;
import jakarta.annotation.Resource;

@GrpcService
public class JobServiceImpl extends JobServiceImplBase {

    @Override
    public void push(JobPushRequest request, StreamObserver<Empty> responseObserver) {
        final var ip = ClientIpInterceptor.CLIENT_IP.get();
        try {
            jobHelper.execute(request.getName(), ip, request.getArgsList().toArray(new String[0]));
        } catch (IOException e) {
            responseObserver.onError(Status.INTERNAL.withCause(e).asRuntimeException());
            return;
        }

        responseObserver.onNext(Empty.newBuilder().build());
        responseObserver.onCompleted();
    }

    @Resource
    JobHelper jobHelper;

}
