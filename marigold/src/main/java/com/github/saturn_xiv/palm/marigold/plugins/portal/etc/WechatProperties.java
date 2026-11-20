package com.github.saturn_xiv.palm.marigold.plugins.portal.etc;

import java.io.Serializable;
import java.util.Map;

import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.validation.annotation.Validated;

import com.github.saturn_xiv.palm.marigold.wechat.WechatMiniProgram;
import com.github.saturn_xiv.palm.marigold.wechat.WechatOauth2;

import jakarta.validation.Valid;

@Validated
@ConfigurationProperties(prefix = "palm.portal.wechat")
public record WechatProperties(Map<String, @Valid WechatOauth2> oauth2,
                Map<String, @Valid WechatMiniProgram> miniProgram) implements Serializable {

}
