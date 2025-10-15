package com.github.saturn_xiv.palm.hyacinth.models;

import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;
import java.util.Optional;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.http.HttpHeaders;
import com.google.protobuf.InvalidProtocolBufferException;
import com.google.protobuf.Message;
import com.google.protobuf.MessageOrBuilder;
import com.google.protobuf.util.JsonFormat;
import io.grpc.Channel;
import io.grpc.ClientInterceptor;
import io.grpc.ManagedChannel;
import io.grpc.Metadata;
import io.grpc.stub.AbstractStub;
import io.grpc.stub.MetadataUtils;

public record HttpRequest(String package_, String service, String method,
                String authorization, String requestType, String requestBody) {

        public void health_check(ManagedChannel channel) {
                // var stub = HealthGrpc.newBlockingStub(channel);
                // healthRequest = HealthCheckRequest.getDefaultInstance();
        }

        @SuppressWarnings({ "rawtypes", "unchecked" })
        public Optional<MessageOrBuilder> execute(ManagedChannel channel)
                        throws ClassNotFoundException, NoSuchMethodException, IllegalAccessException,
                        InvocationTargetException, InvalidProtocolBufferException {
                Class requestClass = Class.forName(requestType);
                Method requestBuilderMethod = requestClass.getMethod("newBuilder");
                Class requestBuilderClass = requestBuilderMethod.getReturnType();
                Object requestBuilder = requestBuilderMethod.invoke(null);
                {
                        JsonFormat.Parser parser = JsonFormat.parser();
                        Method mergeMethod = JsonFormat.Parser.class.getMethod("merge", String.class,
                                        Message.Builder.class);
                        mergeMethod.invoke(parser, this.requestBody, requestBuilder);
                }
                Method requestBuilderBuildMethod = requestBuilderClass.getMethod("build");
                Object request = requestBuilderBuildMethod.invoke(requestBuilder);

                Metadata header = new Metadata();
                if (this.authorization() != null) {
                        Metadata.Key<String> key = Metadata.Key.of(HttpHeaders.AUTHORIZATION.toLowerCase(),
                                        Metadata.ASCII_STRING_MARSHALLER);
                        header.put(key, this.authorization());
                }

                Class grpcClass = Class.forName(String.format("%s.%sGrpc", this.package_, this.service));
                Method grpcNewBlockingStubMethod = grpcClass.getMethod("newBlockingStub", Channel.class);
                Class blockingStubClass = grpcNewBlockingStubMethod.getReturnType();
                Object blockingStubOrigin = grpcNewBlockingStubMethod.invoke(null, channel);
                // https://grpc.github.io/grpc-java/javadoc/io/grpc/stub/AbstractStub.html
                Method stubWithInterceptorsMethod = AbstractStub.class.getMethod("withInterceptors",
                                ClientInterceptor[].class);
                Object blockingStub = stubWithInterceptorsMethod.invoke(blockingStubOrigin,
                                (Object) new ClientInterceptor[] { MetadataUtils.newAttachHeadersInterceptor(header) });
                Method rpcMethod = blockingStubClass.getMethod(this.method, requestClass);
                {
                        Class responseClass = rpcMethod.getReturnType();
                        logger.debug("response class: {}", responseClass.getCanonicalName());
                }

                Object response = rpcMethod.invoke(blockingStub, request);

                return Optional.of((Message) response);

        }

        private static final Logger logger = LoggerFactory.getLogger(HttpRequest.class);
}
