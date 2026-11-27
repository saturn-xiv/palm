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

import jakarta.annotation.Resource;

@Service("portal.s.i18n")
public class I18nService {
    @Transactional(rollbackFor = Throwable.class)
    public void set(Locale lang, String code, String message) {
        final var lng = lang.toLanguageTag();
        var it = localeRepository.findByLangAndCode(lng, code);
        if (it.isPresent()) {
            var cur = it.get();
            cur.setMessage(message);
            localeRepository.save(cur);
        } else {
            var cur = new com.github.saturn_xiv.palm.marigold.plugins.portal.models.Locale();
            cur.setLang(lng);
            cur.setCode(code);
            cur.setMessage(message);
            localeRepository.save(cur);
        }
    }

    public String t(String lang, String code, Object args) {
        final var it = localeRepository.findByLangAndCode(lang, code);
        return it.map(x -> {
            if (args == null) {
                return x.getMessage();
            }
            var tpl = mustacheCompiler.compile(x.getMessage());
            return tpl.execute(args);
        }).orElse(String.format("%s.%s", lang, code));
    }

    public String t(String lang, String code) {
        return t(lang, code, null);
    }

    public String t(Locale locale, String code, Object args) {
        final var lang = locale.toLanguageTag();
        return this.t(lang, code, args);
    }

    public String t(Locale locale, String code) {
        return t(locale, code, null);
    }

    public Locale detect(Locale l) {
        final var s = l.toLanguageTag();
        if (localeRepository.findDistinctLanguages().contains(s)) {
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

    @Transactional(rollbackFor = Throwable.class)
    public void sync() throws IOException {
        logger.warn("locales table is empty, will load data from resources");
        final var items = localesSync.load();
        for (var it : items) {
            final var lang = it.getLeft().toLanguageTag();
            final var code = it.getMiddle();
            logger.debug("found locale {}.{}", lang, code);
            if (localeRepository.findByLangAndCode(lang, code).isEmpty()) {
                var jt = new com.github.saturn_xiv.palm.marigold.plugins.portal.models.Locale();
                jt.setLang(lang);
                jt.setCode(code);
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
    @Resource
    Mustache.Compiler mustacheCompiler;

    private final static Logger logger = LoggerFactory.getLogger(I18nService.class);
}
