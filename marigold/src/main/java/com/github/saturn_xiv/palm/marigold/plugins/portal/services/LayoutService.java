package com.github.saturn_xiv.palm.marigold.plugins.portal.services;

import java.util.Locale;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.info.BuildProperties;
import org.springframework.cache.annotation.CachePut;
import org.springframework.cache.annotation.Cacheable;
import org.springframework.data.redis.core.RedisOperations;
import org.springframework.stereotype.Service;

import com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.responses.PageLayout;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.LocaleRepository;

import jakarta.annotation.Resource;

@Service("portal.s.layout")
public class LayoutService {
    @Cacheable(value = LAYOUT_KEY, key = "#result.locale")
    public PageLayout fetch(Locale lang) {
        return get(lang);
    }

    @CachePut(value = LAYOUT_KEY, key = "#result.locale")
    public PageLayout put(Locale lang) {
        return get(lang);
    }

    private PageLayout get(Locale lang) {
        return new PageLayout(lang, buildProperties, i18nService, settingService, localeRepository);
    }

    @Autowired
    RedisOperations<String, String> redisOperations;
    @Resource
    BuildProperties buildProperties;
    @Resource
    I18nService i18nService;
    @Resource
    SettingService settingService;
    @Resource
    LocaleRepository localeRepository;

    private final static String LAYOUT_KEY = "site.layout";
}
