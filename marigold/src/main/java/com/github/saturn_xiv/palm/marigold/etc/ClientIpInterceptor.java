package com.github.saturn_xiv.palm.marigold.etc;

import java.net.InetSocketAddress;
import java.util.Optional;

import org.springframework.grpc.server.GlobalServerInterceptor;
import org.springframework.stereotype.Component;

import io.grpc.Context;
import io.grpc.Contexts;
import io.grpc.Grpc;
import io.grpc.Metadata;
import io.grpc.ServerCall;
import io.grpc.ServerCall.Listener;
import io.grpc.ServerCallHandler;
import io.grpc.ServerInterceptor;

@Component("grpc.interceptors.client-ip")
@GlobalServerInterceptor
public class ClientIpInterceptor implements ServerInterceptor {
    public static final Context.Key<String> CLIENT_IP = Context.key("client-ip");

    @Override
    public <ReqT, RespT> Listener<ReqT> interceptCall(ServerCall<ReqT, RespT> call, Metadata headers,
            ServerCallHandler<ReqT, RespT> next) {
        final String clientIp = detect(call, headers).orElse("n/a");
        Context context = Context.current().withValue(CLIENT_IP, clientIp);
        return Contexts.interceptCall(context, call, headers, next);
    }

    private Optional<String> detect(ServerCall<?, ?> call, Metadata headers) {

        final var forwardedForKey = Metadata.Key.of("x-forwarded-for", Metadata.ASCII_STRING_MARSHALLER);
        final var forwardedFor = headers.get(forwardedForKey);
        if (forwardedFor != null && !forwardedFor.isBlank()) {
            return Optional.of(forwardedFor.split(",")[0].trim());
        }

        final var remote = call.getAttributes().get(Grpc.TRANSPORT_ATTR_REMOTE_ADDR);
        if (remote instanceof InetSocketAddress address) {
            return Optional.of(address.getAddress().getHostAddress());
        }
        return Optional.empty();
    }
}
