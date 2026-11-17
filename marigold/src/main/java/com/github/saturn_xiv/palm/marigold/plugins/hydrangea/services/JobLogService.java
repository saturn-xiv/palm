package com.github.saturn_xiv.palm.marigold.plugins.hydrangea.services;

import java.util.ArrayList;
import java.util.List;

import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import com.github.saturn_xiv.palm.marigold.plugins.hydrangea.models.JobLog;
import com.github.saturn_xiv.palm.marigold.plugins.hydrangea.repositories.JobLogRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.models.user.Item;

import jakarta.annotation.Resource;

@Service("hydrangea.s.job-log")
public class JobLogService {
    @Transactional(rollbackFor = Throwable.class)
    public void create(String sn, String ip, String name, String version, String summary, String script) {
        create(null, sn, ip, name, version, summary, script, new ArrayList<>());
    }

    @Transactional(rollbackFor = Throwable.class)
    public void create(Item user, String sn, String ip, String name, String version, String summary, String script,
            List<String> args) {
        var it = new JobLog();
        it.setUser(user);
        it.setSn(sn);
        it.setIp(ip);
        it.setName(name);
        it.setScript(script);
        it.setVersion(version);
        it.setSummary(summary);
        it.setArgs(args);
        jobLogRepository.save(it);
    }

    @Resource
    JobLogRepository jobLogRepository;
}
