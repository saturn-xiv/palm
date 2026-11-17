package com.github.saturn_xiv.palm.marigold.plugins.portal.graphql;

import java.security.NoSuchAlgorithmException;
import java.time.ZoneId;
import java.util.Locale;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.graphql.data.method.annotation.Argument;
import org.springframework.graphql.data.method.annotation.MutationMapping;
import org.springframework.graphql.execution.ErrorType;
import org.springframework.stereotype.Controller;

import com.github.saturn_xiv.palm.marigold.plugins.portal.controllers.responses.Succeed;
import com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.requests.SignUpByEmailRequest;
import com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.responses.RefreshResponse;
import com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.responses.SignInResponse;
import com.github.saturn_xiv.palm.marigold.plugins.portal.models.user.ByEmail;
import com.github.saturn_xiv.palm.marigold.plugins.portal.queues.MessageSender;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.EmailUserRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.services.UserService;
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
    public Succeed signUpByEmail(HttpServletRequest request, @Argument @Valid SignUpByEmailRequest form)
            throws InvalidProtocolBufferException, NoSuchAlgorithmException {
        final var ip = request.getRemoteHost();
        final var email = HomeController.email(form.email());
        final var password = HomeController.password(form.password());
        logger.warn("sign up by email {}<{}>", form.name(), email);
        userService.signUp(form.name(), email, password, Locale.forLanguageTag(form.lang()),
                ZoneId.of(form.timezone()), ip);
        final var it = emailUserRepository.findByEmail(email).get();
        sendEmail(it, Action.Confirm);
        return new Succeed();
    }

    @MutationMapping
    public SignInResponse signInByEmail(@Argument @Email @NotBlank String email,
            @Argument @NotBlank String password) {
        // TODO
        return new SignInResponse();
    }

    @MutationMapping
    public Succeed confirmForEmailUser(@Argument @Email @NotBlank String email) {
        final var it = emailUserRepository.findByEmail(email).get();
        if (it.getConfirmedAt() != null) {
            throw GraphqlErrorException.newErrorException().errorClassification(ErrorType.FORBIDDEN)
                    .message("User is already confirmed").build();
        }
        sendEmail(it, Action.Confirm);
        return new Succeed();
    }

    @MutationMapping
    public Succeed confirmEmailUserByToken(@Argument @NotBlank String token) {
        // TODO
        return new Succeed();
    }

    @MutationMapping
    public Succeed unlockForEmailUser(@Argument @Email @NotBlank String email) {
        final var it = emailUserRepository.findByEmail(email).get();
        if (it.getLockedAt() == null) {
            throw GraphqlErrorException.newErrorException().errorClassification(ErrorType.FORBIDDEN)
                    .message("User isn't locked").build();
        }
        sendEmail(it, Action.Unlock);
        return new Succeed();
    }

    @MutationMapping
    public Succeed unlockEmailUserByToken(@Argument @NotBlank String token) {
        // TODO
        return new Succeed();
    }

    @MutationMapping
    public Succeed forgotPasswordForEmailUser(@Argument @Email @NotBlank String email) {
        final var it = emailUserRepository.findByEmail(email).get();
        if (!it.isAvailable()) {
            throw GraphqlErrorException.newErrorException().errorClassification(ErrorType.FORBIDDEN)
                    .message("User is locked").build();
        }
        return new Succeed();
    }

    @MutationMapping
    public Succeed resetPasswordForEmailUser(@Argument @NotBlank String token, @Argument @NotBlank String password)
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

    private void sendEmail(ByEmail emailUser, Action action) {
        // TODO
    }

    @Resource
    UserService userService;
    @Resource
    EmailUserRepository emailUserRepository;
    @Resource
    MessageSender messageSender;

    private final static Logger logger = LoggerFactory.getLogger(UserByEmailController.class);
}
