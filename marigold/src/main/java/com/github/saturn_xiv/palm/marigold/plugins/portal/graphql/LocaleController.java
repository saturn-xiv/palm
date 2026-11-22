package com.github.saturn_xiv.palm.marigold.plugins.portal.graphql;

import java.util.List;
import java.util.Locale;

import org.springframework.graphql.data.method.annotation.Argument;
import org.springframework.graphql.data.method.annotation.MutationMapping;
import org.springframework.graphql.data.method.annotation.QueryMapping;
import org.springframework.stereotype.Controller;

import com.github.saturn_xiv.palm.marigold.plugins.portal.controllers.responses.Succeed;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.LocaleRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.services.I18nService;

import jakarta.annotation.Resource;
import jakarta.validation.constraints.NotBlank;

@Controller("portal.g.locale")
public class LocaleController {
    @QueryMapping
    public List<com.github.saturn_xiv.palm.marigold.plugins.portal.models.Locale> localeByLang(
            @Argument @NotBlank String lang) {
        var lng = Locale.forLanguageTag(lang);
        return localeRepository.findByLang(lng.toLanguageTag());
    }

    @MutationMapping
    public Succeed setLocale(@Argument @NotBlank String lang, @Argument @NotBlank String code,
            @Argument @NotBlank String message) {
        // TODO verify administrator
        i18nService.set(Locale.forLanguageTag(lang), code, message);
        return new Succeed();
    }

    @Resource
    LocaleRepository localeRepository;
    @Resource
    I18nService i18nService;
}
