package com.github.saturn_xiv.palm.marigold.plugins.portal.graphql;

import java.security.NoSuchAlgorithmException;
import java.time.ZoneId;
import java.util.Locale;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.graphql.data.method.annotation.Argument;
import org.springframework.graphql.data.method.annotation.MutationMapping;
import org.springframework.graphql.execution.ErrorType;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.stereotype.Controller;

import com.github.saturn_xiv.palm.marigold.plugins.portal.controllers.responses.Succeed;
import com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.requests.SignUpByEmailRequest;
import com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.responses.RefreshResponse;
import com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.responses.SignInResponse;
import com.github.saturn_xiv.palm.marigold.plugins.portal.models.user.ByEmail;
import com.github.saturn_xiv.palm.marigold.plugins.portal.queues.MessageSender;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.EmailUserRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.services.UserService;
import com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse.Item.Type;
import com.google.protobuf.InvalidProtocolBufferException;

import graphql.GraphqlErrorException;
import jakarta.annotation.Resource;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.validation.Valid;
import jakarta.validation.constraints.Email;
import jakarta.validation.constraints.NotBlank;

@Controller("portal.g.user-by-email")
public class UserByEmailController {
    @MutationMapping
    public Succeed signUpByEmail(HttpServletRequest request_, @Argument @Valid SignUpByEmailRequest request)
            throws InvalidProtocolBufferException, NoSuchAlgorithmException {
        final var ip = request_.getRemoteHost();
        final var email = HomeController.email(request.email());
        final var password = HomeController.password(request.password());
        logger.warn("sign up by email {}<{}>", request.name(), email);
        userService.signUp(request.name(), email, password, Locale.forLanguageTag(request.lang()),
                ZoneId.of(request.timezone()), ip);
        final var it = emailUserRepository.findByEmail(email).get();
        sendEmail(it, Action.Confirm, request.home());
        return new Succeed();
    }

    @MutationMapping
    public SignInResponse signInByEmail(HttpServletRequest request_, @Argument @Email @NotBlank String email,
            @Argument @NotBlank String password) {
        var it = emailUserRepository.findByEmail(email.trim().toLowerCase()).get();
        if (!it.isAvailable()) {
            throw GraphqlErrorException.newErrorException().errorClassification(ErrorType.FORBIDDEN).build();
        }
        if (!passwordEncoder.matches(password, it.getPassword())) {
            throw GraphqlErrorException.newErrorException().errorClassification(ErrorType.UNAUTHORIZED)
                    .message("Wrong password").build();
        }
        final var ip = request_.getRemoteHost();
        userService.signIn(it.getUser(), it.getEmail(), ip);
        return new SignInResponse(it.getUser(), it.getSn(), Type.EMAIL);
    }

    @MutationMapping
    public Succeed confirmForEmailUser(@Argument @Email @NotBlank String email, @Argument @NotBlank String home) {
        final var it = emailUserRepository.findByEmail(email).get();
        if (it.getConfirmedAt() != null) {
            throw GraphqlErrorException.newErrorException().errorClassification(ErrorType.FORBIDDEN)
                    .message("User is already confirmed").build();
        }
        sendEmail(it, Action.Confirm, home);
        return new Succeed();
    }

    @MutationMapping
    public Succeed confirmEmailUserByToken(HttpServletRequest request_, @Argument @NotBlank String token) {
        // TODO
        return new Succeed();
    }

    @MutationMapping
    public Succeed unlockForEmailUser(@Argument @Email @NotBlank String email, @Argument @NotBlank String home) {
        final var it = emailUserRepository.findByEmail(email).get();
        if (it.getLockedAt() == null) {
            throw GraphqlErrorException.newErrorException().errorClassification(ErrorType.FORBIDDEN)
                    .message("User isn't locked").build();
        }
        sendEmail(it, Action.Unlock, home);
        return new Succeed();
    }

    @MutationMapping
    public Succeed unlockEmailUserByToken(HttpServletRequest request_, @Argument @NotBlank String token) {
        // TODO
        return new Succeed();
    }

    @MutationMapping
    public Succeed forgotPasswordForEmailUser(@Argument @Email @NotBlank String email,
            @Argument @NotBlank String home) {
        final var it = emailUserRepository.findByEmail(email).get();
        if (!it.isAvailable()) {
            throw GraphqlErrorException.newErrorException().errorClassification(ErrorType.FORBIDDEN)
                    .message("User is locked").build();
        }
        sendEmail(it, Action.ResetPassword, home);
        return new Succeed();
    }

    @MutationMapping
    public Succeed resetPasswordForEmailUser(HttpServletRequest request_, @Argument @NotBlank String token,
            @Argument @NotBlank String password)
            throws InvalidProtocolBufferException {
        // final var _new_password = HomeController.password(password);

        // TODO
        return new Succeed();
    }

    @MutationMapping
    public RefreshResponse refresh() {
        // TODO
        return new RefreshResponse();
    }

    enum Action {
        Confirm, Unlock, ResetPassword
    }

    private void sendEmail(ByEmail emailUser, Action action, String home) {
        // TODO
    }

    @Resource
    UserService userService;
    @Resource
    EmailUserRepository emailUserRepository;
    @Resource
    MessageSender messageSender;
    @Resource
    PasswordEncoder passwordEncoder;

    private final static Logger logger = LoggerFactory.getLogger(UserByEmailController.class);
}
