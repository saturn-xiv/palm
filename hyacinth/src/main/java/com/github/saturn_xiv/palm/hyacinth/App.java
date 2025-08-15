package com.github.saturn_xiv.palm.hyacinth;

import java.io.IOException;
import java.io.InputStream;
import java.util.Properties;

import javax.xml.parsers.DocumentBuilder;
import javax.xml.parsers.DocumentBuilderFactory;
import javax.xml.parsers.ParserConfigurationException;

import org.apache.commons.cli.CommandLine;
import org.apache.commons.cli.CommandLineParser;
import org.apache.commons.cli.DefaultParser;
import org.apache.commons.cli.Option;
import org.apache.commons.cli.Options;
import org.apache.commons.cli.ParseException;
import org.apache.commons.cli.help.HelpFormatter;
import org.w3c.dom.Document;
import org.w3c.dom.Element;
import org.w3c.dom.NodeList;
import org.xml.sax.SAXException;

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

        Properties props = new Properties();
        try (InputStream ins = App.class.getClassLoader()
                .getResourceAsStream("META-INF/maven/com.github.saturn_xiv.palm/hyacinth/pom.properties")) {
            props.load(ins);
        }

        String project_url = "";
        String project_name = "";
        try (InputStream ins = App.class.getClassLoader()
                .getResourceAsStream("META-INF/maven/com.github.saturn_xiv.palm/hyacinth/pom.xml")) {
            DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
            DocumentBuilder builder = factory.newDocumentBuilder();
            Document doc = builder.parse(ins);
            doc.getDocumentElement().normalize();
            {
                NodeList lst = doc.getElementsByTagName("project");
                if (lst.getLength() > 0) {
                    Element project = (Element) lst.item(0);
                    {
                        NodeList urls = project.getElementsByTagName("url");
                        if (urls.getLength() > 0) {
                            project_url = urls.item(0).getTextContent();
                        }
                    }
                    {
                        NodeList names = project.getElementsByTagName("name");
                        if (names.getLength() > 0) {
                            project_name = names.item(0).getTextContent();
                        }
                    }
                }
            }
        }

        if (line.hasOption(version)) {
            System.out.println(props.getProperty("version"));
            return;
        }

        if (line.hasOption(help)) {
            HelpFormatter formatter = HelpFormatter.builder().get();
            formatter.printHelp(props.getProperty("artifactId"), project_name, options, project_url, true);
            return;
        }

        Integer port_ = line.hasOption(port) ? line.getParsedOptionValue(port) : PORT;
        String config_ = line.hasOption(config) ? line.getParsedOptionValue(config) : CONFIG;
        Server server = new Server(config_);
        server.launch(port_);
    }

}
