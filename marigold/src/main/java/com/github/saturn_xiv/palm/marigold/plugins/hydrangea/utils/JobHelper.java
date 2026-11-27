package com.github.saturn_xiv.palm.marigold.plugins.hydrangea.utils;

import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.HashMap;
import java.util.Map;
import java.util.UUID;
import java.util.stream.Stream;

import org.apache.commons.io.FilenameUtils;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Component;

import com.github.saturn_xiv.palm.marigold.plugins.hydrangea.models.JobConfig;
import com.github.saturn_xiv.palm.marigold.plugins.hydrangea.services.JobLogService;
import com.github.saturn_xiv.palm.marigold.plugins.portal.queues.MessageSender;
import com.github.saturn_xiv.palm.plugins.hydrangea.v1.Task;
import com.samskivert.mustache.Mustache;

import jakarta.annotation.PostConstruct;
import jakarta.annotation.Resource;
import tools.jackson.dataformat.toml.TomlMapper;

@Component("hydrangea.u.job-handler")
public class JobHelper {

    public Map<String, JobConfig> all() throws IOException {
        Map<String, JobConfig> items = new HashMap<>();
        try (Stream<Path> stream = Files.list(Path.of(this.jobsDir))) {
            final var files = stream.filter(Files::isRegularFile)
                    .filter((x) -> EXT.equals(FilenameUtils.getExtension(x.toString()))).toList();
            for (var x : files) {
                items.put(FilenameUtils.getBaseName(x.getFileName().toString()), this.load(x));
            }
        }
        return items;
    }

    public void execute(String name, String ip, String... args) throws IOException {

        logger.info("receive job request {}({})", name, String.join(",", args));
        var file = Path.of(jobsDir, String.format("%s.%s", name, EXT));
        final var job = this.load(file);
        Map<String, Object> context = new HashMap<>();
        if (args == null && job.args().size() == 0) {

        } else if (args != null && args.length == job.args().size()) {
            for (var i = 0; i < args.length; i++) {
                context.put(job.args().get(i).name(), args[i]);
            }
        } else {
            throw new IllegalArgumentException();
        }

        var template = Mustache.compiler().compile(job.script());
        final var script = template.execute(context);
        final var sn = UUID.randomUUID().toString();

        messageSender.produce(jobQueue, emailQueue,
                Task.newBuilder().setSn(sn).setName(name).setVersion(job.version())
                        .setSummary(job.summary()).setScript(script).build());
        jobLogService.create(sn, ip, name, job.version(), job.summary(), script);

    }

    private JobConfig load(Path file) throws IOException {
        logger.debug("load job from {}", file.toUri());
        try (InputStream input = Files.newInputStream(file)) {
            return tomlMapper.readValue(input, JobConfig.class);
        }
    }

    @PostConstruct
    void init() {
        this.tomlMapper = new TomlMapper();
    }

    @Resource
    MessageSender messageSender;
    @Value("${palm.hydrangea.jobs-dir}")
    String jobsDir;
    @Value("${palm.queues.hydrangea.shell-run}")
    String jobQueue;
    @Value("${palm.queues.portal.email-send}")
    String emailQueue;
    @Resource
    JobLogService jobLogService;

    TomlMapper tomlMapper;

    private final String EXT = "toml";
    private final static Logger logger = LoggerFactory.getLogger(JobHelper.class);
}
