package com.github.saturn_xiv.palm.marigold.plugins.portal.controllers.api;

import java.nio.charset.StandardCharsets;
import java.security.InvalidKeyException;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.HexFormat;
import java.util.List;

import javax.crypto.BadPaddingException;
import javax.crypto.IllegalBlockSizeException;
import javax.crypto.NoSuchPaddingException;

import org.apache.commons.codec.digest.MessageDigestAlgorithms;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.server.ResponseStatusException;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.github.saturn_xiv.palm.marigold.plugins.portal.controllers.responses.Succeed;
import com.github.saturn_xiv.palm.marigold.wechat.impl.AuthorizationChangeMessagePushHandler;
import com.github.saturn_xiv.palm.marigold.wechat.requests.MessagePushRequest;
import com.github.saturn_xiv.palm.marigold.wechat.responses.MessagePushResponse;

import jakarta.annotation.PostConstruct;
import jakarta.annotation.Resource;
import jakarta.servlet.http.HttpServletRequest;

@RestController("portal.c.api.wechat.message-push")
@RequestMapping("/api/wechat/message-push")
public class WechatMessagePushController {
    // https://developers.weixin.qq.com/doc/oplatform/Website_App/WeChat_Login/message_push.html
    @GetMapping(produces = MediaType.TEXT_PLAIN_VALUE)
    public String verify(HttpServletRequest request,
            @RequestParam String signature, @RequestParam String timestamp, @RequestParam String nonce,
            @RequestParam(name = "echostr") String echoStr) throws NoSuchAlgorithmException {
        if (!verify(signature, timestamp, nonce)) {
            throw new ResponseStatusException(HttpStatus.BAD_REQUEST);
        }
        return echoStr;
    }

    @PostMapping("/{appId}")
    public MessagePushResponse post(HttpServletRequest request, @PathVariable String appId,
            @RequestParam String signature, @RequestParam String timestamp, @RequestParam String nonce,
            @RequestParam(name = "echostr") String echoStr,
            @RequestBody MessagePushRequest body)
            throws NoSuchAlgorithmException, NoSuchPaddingException, InvalidKeyException, IllegalBlockSizeException,
            BadPaddingException {
        final var ip = request.getRemoteHost();
        if (!body.verify(this.token, signature, timestamp, nonce)) {
            throw new ResponseStatusException(HttpStatus.BAD_REQUEST);
        }
        var message = body.message_body(appId, this.encodingAesKey);
        return this.handleMessage(appId, message, ip);
    }

    private MessagePushResponse handleMessage(String appId, String body, String ip) {
        logger.debug("receive message: {}", body);
        {
            final var req = authorizationChangeMessagePushHandler.parse(objectMapper, body);
            if (req.isPresent()) {
                final var res = authorizationChangeMessagePushHandler.execute(appId, req.get(), ip);
                return new MessagePushResponse(appId, this.encodingAesKey, res);
            }
        }
        logger.warn("ignore it");
        return new MessagePushResponse(appId, this.encodingAesKey, new Succeed());
    }

    private boolean verify(String signature, String timestamp, String nonce) throws NoSuchAlgorithmException {
        var items = List.of(this.token, timestamp, nonce);
        items.sort(null);
        var str = String.join("", items);
        var md = MessageDigest.getInstance(MessageDigestAlgorithms.SHA_1);
        final var digest = md.digest(str.getBytes(StandardCharsets.UTF_8));
        return signature.equals(HexFormat.of().formatHex(digest));
    }

    @PostConstruct
    void init() {
        this.objectMapper = new ObjectMapper();
    }

    @Value("${palm.portal.wechat.message-push.token}")
    String token;
    @Value("${palm.portal.wechat.message-push.encoding-aes-key}")
    String encodingAesKey;
    @Resource
    AuthorizationChangeMessagePushHandler authorizationChangeMessagePushHandler;

    ObjectMapper objectMapper;
    private final static Logger logger = LoggerFactory.getLogger(WechatMessagePushController.class);
}
