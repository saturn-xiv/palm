package com.github.saturn_xiv.palm.marigold.plugins.portal.services;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Locale;
import java.util.Map;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Service;

import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.LocaleRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.utils.LocalesSync;
import org.springframework.transaction.annotation.Transactional;
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

    public List<Map.Entry<String, String>> messages(Locale l) {
        List<Map.Entry<String, String>> items = new ArrayList<>();
        for (var it : localeRepository.findByLang(l.toLanguageTag())) {
            items.add(Map.entry(it.getCode(), it.getMessage()));
        }
        return items;
    }

    @PostConstruct
    void init() throws IOException {
        if (localeRepository.count() == 0) {
            this.sync();
        }
        this.languages = localeRepository.findDistinctLanguages();
    }

    @Transactional(rollbackFor = Throwable.class)
    public void sync() throws IOException {
        logger.warn("locales table is empty, will load data from resources");
        final var items = localesSync.load();
        for (var it : items) {
            var lang = it.getLeft().toLanguageTag();
            logger.debug("found locale {}.{}", lang, it.getMiddle());
            if (localeRepository.findByLangAndCode(lang, lang).isEmpty()) {
                var jt = new com.github.saturn_xiv.palm.marigold.plugins.portal.models.Locale();
                jt.setLang(lang);
                jt.setCode(it.getMiddle());
                jt.setMessage(it.getRight());
                localeRepository.save(jt);
            }
        }
        logger.info("totally found {} locale items", items.size());
    }

    @Resource
    LocaleRepository localeRepository;
    @Resource
    LocalesSync localesSync;

    private List<String> languages;
    private final static Logger logger = LoggerFactory.getLogger(I18nService.class);
}
