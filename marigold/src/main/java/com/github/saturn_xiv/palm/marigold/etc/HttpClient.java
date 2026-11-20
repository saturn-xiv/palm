package com.github.saturn_xiv.palm.marigold.etc;

import org.springframework.context.annotation.Configuration;
import org.springframework.web.service.registry.ImportHttpServices;

import com.github.saturn_xiv.palm.marigold.wechat.WechatApiClient;

@Configuration
@ImportHttpServices(group = "wechat-api-service", types = { WechatApiClient.class })
public class HttpClient {

}
