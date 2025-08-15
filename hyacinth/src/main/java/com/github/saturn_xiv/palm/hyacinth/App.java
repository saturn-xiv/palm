package com.github.saturn_xiv.palm.hyacinth;

import java.io.IOException;

import javax.xml.parsers.ParserConfigurationException;

import org.apache.commons.cli.CommandLine;
import org.apache.commons.cli.CommandLineParser;
import org.apache.commons.cli.DefaultParser;
import org.apache.commons.cli.Option;
import org.apache.commons.cli.Options;
import org.apache.commons.cli.ParseException;
import org.apache.commons.cli.help.HelpFormatter;
import org.xml.sax.SAXException;

import com.github.saturn_xiv.palm.hyacinth.models.MetaInfo;

public class App {
    public static void main(String[] args)
            throws IOException, ParseException, SAXException, ParserConfigurationException {
        final int PORT = 8080;
        final String CONFIG = "config.toml";
        Options options = new Options();

        Option version = Option.builder("v")
                .longOpt("version")
                .desc("print the version information and exit")
                .get();
        Option help = Option.builder("h")
                .longOpt("help")
                .desc("print this message")
                .get();
        Option port = Option.builder("p")
                .hasArg()
                .type(Integer.class)
                .longOpt("port")
                .desc(String.format("port to listen(%d)", PORT))
                .get();
        Option config = Option.builder("c")
                .hasArg()
                .type(String.class)
                .longOpt("config")
                .desc(String.format("configuration file to load(%s)", CONFIG))
                .get();
        options.addOption(port);
        options.addOption(version);
        options.addOption(help);
        options.addOption(config);
        CommandLineParser parser = new DefaultParser();
        CommandLine line = parser.parse(options, args);

        final var metaInfo = new MetaInfo();

        if (line.hasOption(version)) {
            System.out.println(metaInfo.getVersion());
            return;
        }

        if (line.hasOption(help)) {
            HelpFormatter formatter = HelpFormatter.builder().get();
            formatter.printHelp(metaInfo.getArtifactId(),
                    String.format("%s - %s", metaInfo.getName(), metaInfo.getDescription()), options,
                    metaInfo.getUrl(), true);
            return;
        }

        Integer port_ = line.hasOption(port) ? line.getParsedOptionValue(port) : PORT;
        String config_ = line.hasOption(config) ? line.getParsedOptionValue(config) : CONFIG;
        Server server = new Server(config_);
        server.launch(port_);
    }

}
