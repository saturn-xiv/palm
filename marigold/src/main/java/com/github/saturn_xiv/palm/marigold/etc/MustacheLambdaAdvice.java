package com.github.saturn_xiv.palm.marigold.etc;

import java.util.Arrays;
import java.util.Locale;

import org.springframework.context.MessageSource;
import org.springframework.context.i18n.LocaleContextHolder;
import org.springframework.web.bind.annotation.ControllerAdvice;
import org.springframework.web.bind.annotation.ModelAttribute;

import com.samskivert.mustache.Mustache;

@ControllerAdvice
public class MustacheLambdaAdvice {
    public MustacheLambdaAdvice(MessageSource messageSource) {
        this.messageSource = messageSource;
    }

    @ModelAttribute("tr")
    public Mustache.Lambda tr() {
        return (frag, out) -> {
            Locale locale = LocaleContextHolder.getLocale();
            String raw = frag.execute().trim();
            String[] parts = raw.split("\\|");
            String key = parts[0].trim();
            Object[] args = parts.length > 1 ? Arrays.copyOfRange(parts, 1, parts.length) : null;

            String message = messageSource.getMessage(key, args, key, locale);
            out.write(message);
        };
    }

    private final MessageSource messageSource;
}
