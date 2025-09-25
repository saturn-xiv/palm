package com.github.saturn_xiv.palm.hyacinth.controllers;

import jakarta.annotation.Resource;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.http.HttpHeaders;
import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestHeader;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.server.ResponseStatusException;

import com.github.saturn_xiv.palm.hyacinth.models.HttpRequest;
import com.github.saturn_xiv.palm.hyacinth.services.RpcService;

@RestController("/api")
public class ApiController {
    @PostMapping(value = "/{host}/{package}/{service}/{method}", produces = MediaType.APPLICATION_JSON_VALUE)
    String execute(@PathVariable String host, @PathVariable("package") String package_, @PathVariable String service,
            @PathVariable String method, @RequestParam(value = "q", required = true) String requestType,
            @RequestHeader(HttpHeaders.AUTHORIZATION) String authorization,
            @RequestBody String requestBody) {
        // TODO create log
        final var request = new HttpRequest(package_, service, method, authorization, requestType, requestBody);
        final var reply = rpcService.call(host, request);
        if (reply.left == HttpStatus.OK) {
            return reply.right;
        }
        logger.error("{} {}", reply.left, reply.right);
        throw new ResponseStatusException(reply.left, reply.right);
    }

    @Resource
    RpcService rpcService;

    private static final Logger logger = LoggerFactory.getLogger(ApiController.class);
}
