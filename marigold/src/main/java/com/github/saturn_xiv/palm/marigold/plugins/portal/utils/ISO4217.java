package com.github.saturn_xiv.palm.marigold.plugins.portal.utils;

import java.io.IOException;
import java.io.InputStream;
import java.util.ArrayList;
import java.util.List;

import javax.xml.XMLConstants;
import javax.xml.parsers.DocumentBuilderFactory;
import javax.xml.parsers.ParserConfigurationException;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.core.io.Resource;
import org.springframework.stereotype.Component;
import org.w3c.dom.Element;
import org.w3c.dom.NodeList;
import org.xml.sax.SAXException;

@Component("portal.h.iso4217")
public class ISO4217 {
    public record Item(String name, String country, String code, String number, String units, String fund) {
    }

    public List<Item> load() throws ParserConfigurationException, SAXException, IOException {
        final String isFund = "IsFund";

        var factory = DocumentBuilderFactory.newInstance();
        factory.setFeature(XMLConstants.FEATURE_SECURE_PROCESSING, true);

        var builder = factory.newDocumentBuilder();
        List<Item> items = new ArrayList<>();

        try (InputStream ins = listOneXml.getInputStream()) {
            var doc = builder.parse(ins);
            doc.getDocumentElement().normalize();
            NodeList tables = doc.getElementsByTagName("CcyTbl");
            for (var i = 0; i < tables.getLength(); i++) {
                var it = (Element) tables.item(i);
                var currencies = it.getElementsByTagName("CcyNtry");
                for (var j = 0; j < currencies.getLength(); j++) {
                    var jt = (Element) currencies.item(j);
                    var country = jt.getElementsByTagName("CtryNm").item(0).getTextContent();
                    var name = jt.getElementsByTagName("CcyNm").item(0).getTextContent();
                    var codes = jt.getElementsByTagName("Ccy");
                    if (codes.getLength() == 0) {
                        logger.warn("without code, ignore {} {}", name, country);
                        continue;
                    }
                    var code = codes.item(0).getTextContent();
                    var number = jt.getElementsByTagName("CcyNbr").item(0).getTextContent();
                    var units = jt.getElementsByTagName("CcyMnrUnts").item(0).getTextContent();
                    var fund = jt.hasAttribute(isFund) ? jt.getAttribute(isFund) : null;
                    items.add(new Item(name, country, code, number, units, fund));
                }
            }
        }
        return items;
    }

    @Value("classpath:iso4217/list-one.xml")
    private Resource listOneXml;
    private final static Logger logger = LoggerFactory.getLogger(ISO4217.class);
}
