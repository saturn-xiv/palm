package com.github.saturn_xiv.palm.camellia.controllers;

import java.util.ArrayList;
import java.util.List;

import org.springframework.graphql.data.method.annotation.Argument;
import org.springframework.graphql.data.method.annotation.MutationMapping;
import org.springframework.stereotype.Controller;

@Controller("palm.camellia.wechat-mini-program-controller")
public class WechatMiniProgramAuthController {
    @MutationMapping
    public List<String> wechatMiniProgramSignIn(@Argument String name) {
        // TODO
        List<String> items = new ArrayList<>();
        return items;
    }
}
