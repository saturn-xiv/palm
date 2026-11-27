package com.github.saturn_xiv.palm.marigold.plugins.portal.rpc;

import com.github.saturn_xiv.palm.plugins.portal.v1.AdministratorServiceGrpc.AdministratorServiceImplBase;

import java.io.IOException;
import java.security.NoSuchAlgorithmException;
import java.time.ZoneId;
import java.util.Locale;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.grpc.server.service.GrpcService;

import com.github.saturn_xiv.palm.marigold.etc.ClientIpInterceptor;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.EmailUserRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.GoogleOauth2UserRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.SmsUserRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.WechatMiniProgramUserRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.WechatOauth2UserRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.services.I18nService;
import com.github.saturn_xiv.palm.marigold.plugins.portal.services.UserService;
import com.github.saturn_xiv.palm.plugins.portal.v1.CreateUserByEmailRequest;
import com.github.saturn_xiv.palm.plugins.portal.v1.SetPasswordForEmailUserRequest;
import com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse;
import com.google.protobuf.Empty;

import io.grpc.Status;
import io.grpc.stub.StreamObserver;
import jakarta.annotation.Resource;
import jakarta.validation.ConstraintViolationException;
import jakarta.validation.Validator;
import jakarta.validation.constraints.Email;
import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.NotEmpty;
import jakarta.validation.constraints.Size;

@GrpcService
public class AdministratorServiceImpl extends AdministratorServiceImplBase {
    @Override
    public void listUser(Empty request, StreamObserver<UserIndexResponse.Item> responseObserver) {
        for (var it : emailUserRepository.findAll()) {
            responseObserver
                    .onNext(UserIndexResponse.Item.newBuilder().setType(UserIndexResponse.Item.Type.EMAIL)
                            .setId(it.getUser().getId()).setName(it.getName()).setSn(it.getSn()).setLabel(it.label())
                            .build());
        }
        for (var it : smsUserRepository.findAll()) {
            responseObserver
                    .onNext(UserIndexResponse.Item.newBuilder().setType(UserIndexResponse.Item.Type.SMS)
                            .setId(it.getUser().getId()).setName(it.getName()).setSn(it.getSn()).setLabel(it.label())
                            .build());
        }
        for (var it : googleOauth2UserRepository.findAll()) {
            responseObserver
                    .onNext(UserIndexResponse.Item.newBuilder().setType(UserIndexResponse.Item.Type.GOOGLE_OAUTH2)
                            .setId(it.getUser().getId()).setName(it.getName()).setSn(it.getSn()).setLabel(it.label())
                            .build());
        }
        for (var it : wechatOauth2UserRepository.findAll()) {
            responseObserver
                    .onNext(UserIndexResponse.Item.newBuilder().setType(UserIndexResponse.Item.Type.WECHAT_OAUTH2)
                            .setId(it.getUser().getId()).setName(it.getNickname()).setSn(it.getSn())
                            .setLabel(it.label()).build());
        }
        for (var it : wechatMiniProgramUserRepository.findAll()) {
            responseObserver
                    .onNext(UserIndexResponse.Item.newBuilder().setType(UserIndexResponse.Item.Type.WECHAT_MINI_PROGRAM)
                            .setId(it.getUser().getId()).setName(it.getNickname()).setSn(it.getSn())
                            .setLabel(it.label()).build());
        }
        responseObserver.onCompleted();
    }

    public record CreateUserByEmailForm(@NotEmpty @NotBlank @Size(min = 2, max = 31) String name,
            @NotEmpty @Email @Size(min = 5, max = 63) String email,
            @NotEmpty @Size(min = 6, max = 31) String password) {
    }

    @Override
    public void createUserByEmail(CreateUserByEmailRequest request, StreamObserver<Empty> responseObserver) {
        final var form = new CreateUserByEmailForm(request.getName().trim(), request.getEmail().trim().toLowerCase(),
                request.getPassword());
        final var violations = validator.validate(form);
        if (!violations.isEmpty()) {
            responseObserver.onError(Status.INVALID_ARGUMENT.withCause(new ConstraintViolationException(violations))
                    .asRuntimeException());
            return;
        }
        try {
            final var ip = ClientIpInterceptor.CLIENT_IP.get();
            logger.info("create user {}<{}>", form.name, form.email);
            userService.signUp(form.name, form.email, form.password, Locale.getDefault(), ZoneId.systemDefault(), ip);
            userService.confirm(form.email, ip);

            responseObserver.onNext(Empty.newBuilder().build());
            responseObserver.onCompleted();
        } catch (NoSuchAlgorithmException e) {
            responseObserver.onError(Status.INTERNAL.withCause(e).asRuntimeException());
            return;
        }
    }

    @Override
    public void dbSeeds(Empty request, StreamObserver<Empty> responseObserver) {
        try {
            i18nService.sync();
            responseObserver.onNext(Empty.newBuilder().build());
            responseObserver.onCompleted();
        } catch (IOException e) {
            responseObserver.onError(Status.INTERNAL.withCause(e).asRuntimeException());
            return;
        }
    }

    public record SetPasswordForEmailUserForm(@NotEmpty @Email @Size(min = 5, max = 63) String email,
            @NotEmpty @Size(min = 6, max = 31) String password) {
    }

    @Override
    public void setPasswordForEmailUser(SetPasswordForEmailUserRequest request,
            StreamObserver<Empty> responseObserver) {
        final var form = new SetPasswordForEmailUserForm(request.getEmail().trim().toLowerCase(),
                request.getPassword());
        final var violations = validator.validate(form);
        if (!violations.isEmpty()) {
            responseObserver.onError(Status.INVALID_ARGUMENT.withCause(new ConstraintViolationException(violations))
                    .asRuntimeException());
            return;
        }

        final var ip = ClientIpInterceptor.CLIENT_IP.get();

        logger.info("set password for user {}", form.email);
        userService.setPassword(form.email, form.password, ip);

        responseObserver.onNext(Empty.newBuilder().build());
        responseObserver.onCompleted();
    }

    @Resource
    I18nService i18nService;
    @Resource
    UserService userService;
    @Resource
    EmailUserRepository emailUserRepository;
    @Resource
    SmsUserRepository smsUserRepository;
    @Resource
    GoogleOauth2UserRepository googleOauth2UserRepository;
    @Resource
    WechatOauth2UserRepository wechatOauth2UserRepository;
    @Resource
    WechatMiniProgramUserRepository wechatMiniProgramUserRepository;
    @Resource
    Validator validator;

    private final static Logger logger = LoggerFactory.getLogger(AdministratorServiceImpl.class);

}
