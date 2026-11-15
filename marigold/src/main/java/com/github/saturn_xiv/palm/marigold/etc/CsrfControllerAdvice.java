package com.github.saturn_xiv.palm.marigold.etc;

import java.util.Map;

import org.springframework.security.web.csrf.CsrfToken;
import org.springframework.web.bind.annotation.ControllerAdvice;
import org.springframework.web.bind.annotation.ModelAttribute;

@ControllerAdvice
public class CsrfControllerAdvice {
    @ModelAttribute("_csrf")
    public Map<String, String> csrfToken(CsrfToken token) {
        if (token == null) {
            return Map.of();
        }
        return Map.of(
                "parameterName", token.getParameterName(),
                "token", token.getToken());
    }
}
