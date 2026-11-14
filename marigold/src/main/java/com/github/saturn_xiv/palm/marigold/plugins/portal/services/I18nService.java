package com.github.saturn_xiv.palm.marigold.plugins.portal.services;

import java.util.List;
import java.util.Locale;

import org.springframework.stereotype.Service;

import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.LocaleRepository;
import com.samskivert.mustache.Mustache;

import jakarta.annotation.PostConstruct;
import jakarta.annotation.Resource;

@Service("portal.s.i18n")
public class I18nService {
    public String t(Locale locale, String code) {
        return t(locale, code, null);
    }

    public String t(Locale locale, String code, Object args) {
        final var lang = locale.toLanguageTag();
        final var it = localeRepository.findByLangAndCode(lang, code);
        return it.map(x -> {
            if (args == null) {
                return x.getMessage();
            }
            var tpl = Mustache.compiler().compile(x.getMessage());
            return tpl.execute(args);
        }).orElse(String.format("%s.%s", lang, code));
    }

    public Locale detect(Locale l) {
        final var s = l.toLanguageTag();
        if (this.languages.contains(s)) {
            return l;
        }
        return Locale.US;
    }

    @PostConstruct
    void init() {
        this.languages = localeRepository.findDistinctLanguages();
    }

    @Resource
    LocaleRepository localeRepository;
    private List<String> languages;
}
