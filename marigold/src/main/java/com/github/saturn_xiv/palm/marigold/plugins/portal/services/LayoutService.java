package com.github.saturn_xiv.palm.marigold.plugins.portal.services;

import java.util.Locale;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.cache.annotation.CachePut;
import org.springframework.cache.annotation.Cacheable;
import org.springframework.data.redis.core.RedisOperations;
import org.springframework.stereotype.Service;

import com.github.saturn_xiv.palm.marigold.plugins.portal.controllers.responses.Layout;

@Service("portal.s.layout")
public class LayoutService {
    @Cacheable(value = LAYOUT_KEY, key = "#result.locale")
    public Layout fetch(Locale lang) {

        return get(lang);
    }

    @CachePut(value = LAYOUT_KEY, key = "#result.locale")
    public Layout put(Locale lang) {
        return get(lang);

    }

    private Layout get(Locale lang) {
        // TODO
        return new Layout(lang, null, null, null);
    }

    @Autowired
    RedisOperations<String, String> redisOperations;

    private final static String LAYOUT_KEY = "site.layout";
}
