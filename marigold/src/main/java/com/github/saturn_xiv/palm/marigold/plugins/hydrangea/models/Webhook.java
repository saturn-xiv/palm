package com.github.saturn_xiv.palm.marigold.plugins.hydrangea.models;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.io.Serializable;
import java.nio.charset.StandardCharsets;
import java.nio.file.Path;
import java.security.InvalidKeyException;
import java.security.NoSuchAlgorithmException;
import java.util.HashMap;
import java.util.HexFormat;
import java.util.Map;
import java.util.UUID;

import javax.crypto.Mac;
import javax.crypto.spec.SecretKeySpec;

import org.apache.commons.codec.digest.HmacAlgorithms;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.github.saturn_xiv.palm.marigold.plugins.hydrangea.services.JobLogService;
import com.github.saturn_xiv.palm.marigold.plugins.portal.queues.MessageSender;
import com.github.saturn_xiv.palm.plugins.hydrangea.v1.Task;
import com.samskivert.mustache.Mustache;

import jakarta.servlet.http.HttpServletRequest;
import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.Size;

public record Webhook(@NotBlank @Size(max = 15) String version,
        @NotBlank @Size(min = 8, max = 127) String secret,
        @NotBlank String file)
        implements Serializable {

    public void handle(HttpServletRequest request, JobLogService jobLogService, MessageSender messageSender,
            Map<String, String> headers, String queue,
            String name, String summary, String body)
            throws NoSuchAlgorithmException, InvalidKeyException, IOException {
        verify(body);

        final var script = script(headers, body);
        final var sn = UUID.randomUUID().toString();
        final var ip = request.getRemoteHost();

        messageSender.produce(queue, sn, Task.newBuilder().setSn(sn).setName(name).setVersion(version())
                .setSummary(summary).setScript(script).build());
        jobLogService.create(sn, ip, name, ip, summary, script);
    }

    private String script(Map<String, String> headers, String body)
            throws IOException {
        logger.debug("load job {} {}", this.file, this.version);
        var mapper = new ObjectMapper();
        var payload = mapper.readValue(
                body,
                new TypeReference<Map<String, Object>>() {
                });
        Map<String, Object> context = new HashMap<>();
        context.put("headers", headers);
        context.put("payload", payload);
        try (BufferedReader reader = new BufferedReader(
                new FileReader(Path.of(this.file).toFile(), StandardCharsets.UTF_8))) {
            var template = Mustache.compiler().compile(reader);
            return template.execute(context);
        }
    }

    private String verify(String body) throws NoSuchAlgorithmException, InvalidKeyException {
        final var algorithm = HmacAlgorithms.HMAC_SHA_256.getName();
        var mac = Mac.getInstance(algorithm);
        mac.init(new SecretKeySpec(
                this.secret.getBytes(StandardCharsets.UTF_8),
                algorithm));
        var buf = mac.doFinal(body.getBytes(StandardCharsets.UTF_8));
        return HexFormat.of().formatHex(buf);
    }

    private final static Logger logger = LoggerFactory.getLogger(Webhook.class);
}
