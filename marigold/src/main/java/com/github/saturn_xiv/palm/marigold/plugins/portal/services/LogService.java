package com.github.saturn_xiv.palm.marigold.plugins.portal.services;

import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.Log;
import com.github.saturn_xiv.palm.marigold.plugins.portal.models.Log.Level;
import com.github.saturn_xiv.palm.marigold.plugins.portal.models.user.Item;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.LogRepository;

import jakarta.annotation.Resource;

@Service("portal.s.log")
public class LogService {
    @Transactional(rollbackFor = Throwable.class)
    public <T> void info(Item user, Class<T> plugin, String ip, String message) {
        create(user, plugin, ip, Level.Info, message);
    }

    private <T> void create(Item user, Class<T> plugin, String ip, Log.Level level, String message) {
        var it = new Log();
        it.setPlugin(plugin.getTypeName());
        it.setIp(ip);
        it.setLevel(level);
        it.setUser(user);
        it.setMessage(message);
        logRepository.save(it);
    }

    @Resource
    LogRepository logRepository;
}
