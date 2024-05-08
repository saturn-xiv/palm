package com.github.saturn_xiv.palm.tutorial;

import org.apache.thrift.TException;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.FileInputStream;
import java.io.IOException;
import java.util.Properties;

public class Main {
    public static void main(String[] args) throws IOException, TException {
        if (args.length != 1) {
            logger.error("usage: java -jar tutorial-1.0-SNAPSHOT-all.jar config.properties");
            return;
        }
        logger.info("load configuration from {}", args[0]);
        var config = new Properties();
        try (var fis = new FileInputStream(args[0])) {
            config.load(fis);

            var morus = new MorusClient(config.getProperty("morus.host"), Integer.parseInt(config.getProperty("morus.port")));
            final var md = "- [Google](https://www.google.com)";
            var htm = morus.markdownToHtml(md, true);
            logger.info("{} => {}", md, htm);
        }
    }

    private final static Logger logger = LoggerFactory.getLogger(Main.class);
}