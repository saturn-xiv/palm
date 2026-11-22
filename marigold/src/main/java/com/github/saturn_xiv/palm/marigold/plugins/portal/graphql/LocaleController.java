package com.github.saturn_xiv.palm.marigold.plugins.portal.graphql;

import java.util.List;
import java.util.Locale;

import org.springframework.graphql.data.method.annotation.Argument;
import org.springframework.graphql.data.method.annotation.QueryMapping;
import org.springframework.stereotype.Controller;

import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.LocaleRepository;

import jakarta.annotation.Resource;
import jakarta.validation.constraints.NotBlank;

@Controller("portal.g.locale")
public class LocaleController {
    @QueryMapping
    public List<com.github.saturn_xiv.palm.marigold.plugins.portal.models.Locale> indexLocaleByLang(
            @Argument @NotBlank String lang) {
        var lng = Locale.forLanguageTag(lang);
        return localeRepository.findByLang(lng.toLanguageTag());
    }

    @Resource
    LocaleRepository localeRepository;
}
