package com.github.saturn_xiv.palm.camellia.controllers;

import java.util.ArrayList;
import java.util.List;

import org.springframework.graphql.data.method.annotation.MutationMapping;
import org.springframework.graphql.data.method.annotation.SchemaMapping;
import org.springframework.stereotype.Controller;

import com.github.saturn_xiv.palm.camellia.requests.WechatOauth2SignInForm;

@Controller("palm.camellia.wechat-oauth2-controller")
public class WechatOauth2AuthController {
    @MutationMapping
    @SchemaMapping
    public List<String> wechatOauth2SignIn(WechatOauth2SignInForm form) {
        // TODO
        List<String> items = new ArrayList<>();
        return items;
    }
}
