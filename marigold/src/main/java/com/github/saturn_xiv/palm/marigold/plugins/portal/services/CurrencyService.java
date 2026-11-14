package com.github.saturn_xiv.palm.marigold.plugins.portal.services;

import java.io.IOException;

import javax.xml.parsers.ParserConfigurationException;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import org.xml.sax.SAXException;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.Currency;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.CurrencyRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.utils.ISO4217;

import jakarta.annotation.PostConstruct;
import jakarta.annotation.Resource;

@Service("portal.s.currency")
public class CurrencyService {
    @PostConstruct
    void init() throws ParserConfigurationException, SAXException, IOException {
        if (currencyRepository.count() == 0) {
            logger.warn("currencies table is empty, load from resources");
            this.sync();
        }
    }

    @Transactional(rollbackFor = Throwable.class)
    public void sync() throws ParserConfigurationException, SAXException, IOException {
        var items = iso4217.load();
        for (var it : items) {
            logger.debug("found currency {}({}) {}", it.name(), it.code(), it.country());
            var currency = new Currency();
            currency.setCode(it.code());
            currency.setName(it.name());
            currency.setNumber(it.number());
            currency.setCountry(it.country());
            if (!it.units().equals("N.A.")) {
                currency.setUnits(Integer.parseInt(it.units()));
            }
            if (it.fund() != null) {
                currency.setFund(Boolean.parseBoolean(it.fund()));
            }
            currencyRepository.save(currency);
        }
        logger.info("totally found {} currency items", items.size());
    }

    @Resource
    ISO4217 iso4217;
    @Resource
    CurrencyRepository currencyRepository;

    private final static Logger logger = LoggerFactory.getLogger(CurrencyService.class);
}
