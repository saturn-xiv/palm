package com.github.saturn_xiv.palm.marigold.plugins.portal.utils;

import java.util.Set;

import org.springframework.stereotype.Component;

import com.google.i18n.phonenumbers.NumberParseException;
import com.google.i18n.phonenumbers.PhoneNumberUtil;
import com.google.i18n.phonenumbers.PhoneNumberUtil.PhoneNumberFormat;
import com.google.i18n.phonenumbers.Phonenumber.PhoneNumber;

import jakarta.annotation.PostConstruct;

@Component("portal.u.phone-number")
public class PhoneNumberHelper {
    public int countryCodeForRegion(String region) {
        return this.instance.getCountryCodeForRegion(region);
    }

    public Set<String> supportedRegions() {
        return this.instance.getSupportedRegions();
    }

    public PhoneNumber parse(String number, String region) throws NumberParseException {
        final var it = this.instance.parse(number, region);
        if (!this.instance.isValidNumber(it)) {
            throw new IllegalArgumentException();
        }
        return it;
    }

    public String format(PhoneNumber number) {
        return this.format(number, PhoneNumberFormat.E164);
    }

    public String format(PhoneNumber number, PhoneNumberFormat format) {
        return this.instance.format(number, format);
    }

    @PostConstruct
    void init() {
        this.instance = PhoneNumberUtil.getInstance();
    }

    private PhoneNumberUtil instance;
}
