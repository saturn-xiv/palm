package com.github.saturn_xiv.palm.marigold.plugins.portal.graphql;

import java.util.Locale;
import java.util.Optional;

import org.springframework.graphql.data.method.annotation.Argument;
import org.springframework.graphql.data.method.annotation.QueryMapping;
import org.springframework.stereotype.Controller;

import com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.responses.CurrentUser;
import com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.responses.RefreshResponse;
import com.github.saturn_xiv.palm.marigold.plugins.portal.models.Session;
import com.github.saturn_xiv.palm.marigold.plugins.portal.services.LayoutService;
import com.github.saturn_xiv.palm.marigold.plugins.portal.utils.CasbinEnforcer;
import com.google.protobuf.InvalidProtocolBufferException;

import jakarta.annotation.Resource;
import jakarta.validation.constraints.NotBlank;

@Controller("portal.g.user")
public class UserController {
    @QueryMapping
    public RefreshResponse refresh(@Argument @NotBlank String lang) throws InvalidProtocolBufferException {
        // TODO
        Optional<Session> ss = Optional.empty();
        var page = layoutService.fetch(Locale.forLanguageTag(lang));
        if (ss.isEmpty()) {
            return new RefreshResponse(page, null);
        }
        var user = new CurrentUser(ss.get(), enforcer);
        return new RefreshResponse(page, user);
    }

    @Resource
    LayoutService layoutService;
    @Resource
    CasbinEnforcer enforcer;
}
