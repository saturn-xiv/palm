package com.github.saturn_xiv.palm.camellia;

import java.io.File;
import java.io.IOException;

import org.apache.commons.cli.CommandLine;
import org.apache.commons.cli.CommandLineParser;
import org.apache.commons.cli.DefaultParser;
import org.apache.commons.cli.Option;
import org.apache.commons.cli.Options;
import org.apache.commons.cli.ParseException;
import org.apache.commons.cli.help.HelpFormatter;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import tools.jackson.dataformat.toml.TomlMapper;

public class App {
    public static void main(String[] args) throws IOException {

        Option config_option = Option.builder("c").longOpt("config").hasArg().since("now")
                .desc("configuration file(toml)").get();
        Option port_option = Option.builder("p").longOpt("port").hasArg().since("now")
                .desc("http server's port").type(Integer.class).get();
        Option version_option = Option.builder("v").longOpt("version").since("now")
                .desc("print the version information").get();
        Option help_option = Option.builder("h").longOpt("help").since("now")
                .desc("print usage message").get();
        Options options = new Options().addOption(config_option).addOption(port_option).addOption(help_option)
                .addOption(version_option);

        CommandLineParser parser = new DefaultParser();
        HelpFormatter formatter = HelpFormatter.builder().get();
        CommandLine cmd = null;

        try {
            cmd = parser.parse(options, args);
        } catch (ParseException e) {
            System.out.println(e.getMessage());
            // TODO parse pom.properties
            formatter.printHelp("camellia", "header", options, "footer", true);
            System.exit(1);
        }
        if (cmd.hasOption("help")) {
            // TODO
            formatter.printHelp("camellia", "header", options, "footer", true);
            return;
        }
        if (cmd.hasOption("version")) {
            // TODO
            System.out.println("xxx");
            return;
        }

        String config_file = cmd.getOptionValue("config", "config.toml");
        String port = cmd.getOptionValue("port", "8080");

        logger.info("load configuration from file {}", config_file);
        TomlMapper mapper = new TomlMapper();
        final var config = mapper.readValue(new File(config_file), Config.class);

        var server = new Server(config);
        server.start(Integer.parseInt(port));
    }

    private final static Logger logger = LoggerFactory.getLogger(App.class);
}
