package com.github.saturn_xiv.palm.hyacinth.models;

import java.io.IOException;
import java.io.InputStream;
import java.util.Properties;

import javax.xml.parsers.DocumentBuilder;
import javax.xml.parsers.DocumentBuilderFactory;
import javax.xml.parsers.ParserConfigurationException;

import org.apache.commons.cli.ParseException;
import org.w3c.dom.Document;
import org.w3c.dom.Element;
import org.w3c.dom.NodeList;
import org.xml.sax.SAXException;

public class MetaInfo {

    public MetaInfo() throws IOException, ParseException, SAXException, ParserConfigurationException {
        Properties props = new Properties();
        try (InputStream ins = MetaInfo.class.getClassLoader()
                .getResourceAsStream("META-INF/maven/com.github.saturn_xiv.palm/hyacinth/pom.properties")) {
            props.load(ins);
        }
        this.artifactId = props.getProperty("artifactId");
        this.version = props.getProperty("version");

        try (InputStream ins = MetaInfo.class.getClassLoader()
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
                        NodeList items = project.getElementsByTagName("url");
                        if (items.getLength() > 0) {
                            this.url = items.item(0).getTextContent();
                        }
                    }
                    {
                        NodeList items = project.getElementsByTagName("name");
                        if (items.getLength() > 0) {
                            this.name = items.item(0).getTextContent();
                        }
                    }
                    {
                        NodeList items = project.getElementsByTagName("description");
                        if (items.getLength() > 0) {
                            this.description = items.item(0).getTextContent();
                        }
                    }
                }
            }
        }
    }

    private String name;
    private String url;
    private String description;
    private String version;
    private String artifactId;

    public String getVersion() {
        return this.version;
    }

    public String getName() {
        return this.name;
    }

    public String getDescription() {
        return this.description;
    }

    public String getUrl() {
        return this.url;
    }

    public String getArtifactId() {
        return this.artifactId;
    }
}
