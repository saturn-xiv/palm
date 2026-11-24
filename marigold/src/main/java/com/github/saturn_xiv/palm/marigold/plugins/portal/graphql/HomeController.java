package com.github.saturn_xiv.palm.marigold.plugins.portal.graphql;

import java.io.Serializable;
import java.time.OffsetDateTime;
import java.time.ZoneId;
import java.time.ZoneOffset;
import java.time.format.TextStyle;
import java.util.Arrays;
import java.util.Base64;
import java.util.Currency;
import java.util.List;
import java.util.Locale;
import java.util.Objects;
import java.util.Set;
import java.util.stream.Collectors;

import org.springframework.boot.info.BuildProperties;
import org.springframework.graphql.data.method.annotation.Argument;
import org.springframework.graphql.data.method.annotation.QueryMapping;
import org.springframework.graphql.data.method.annotation.SchemaMapping;
import org.springframework.stereotype.Controller;

import com.github.saturn_xiv.palm.marigold.plugins.portal.utils.PhoneNumberHelper;
import com.github.saturn_xiv.palm.plugins.portal.v1.PasswordBox;
import com.google.protobuf.InvalidProtocolBufferException;

import jakarta.annotation.Resource;
import jakarta.validation.constraints.NotEmpty;

@Controller("portal.g.home")
public class HomeController {
    @QueryMapping
    public String apiVersion() {
        return buildProperties.getVersion();
    }

    @QueryMapping
    public OffsetDateTime buildTime() {
        return buildProperties.getTime().atOffset(ZoneOffset.UTC);
    }

    @QueryMapping
    public Set<Timezone> timezones(@Argument @NotEmpty String lang) {
        final var lng = Locale.forLanguageTag(lang);
        return ZoneId.getAvailableZoneIds().stream().map((x) -> {
            var it = ZoneId.of(x);
            return new Timezone(x, it.getDisplayName(TextStyle.FULL_STANDALONE, lng));
        }).collect(Collectors.toSet());
    }

    public record Timezone(String code, String name) implements Serializable {
        @Override
        public int hashCode() {
            return Objects.hash(code);
        }
    }

    @QueryMapping
    public Language[] languages(@Argument @NotEmpty String lang) {
        final var lng = Locale.forLanguageTag(lang);
        return Arrays.stream(Locale.getAvailableLocales())
                .map((x) -> new Language(x.toLanguageTag(), x.getDisplayName(lng)))
                .toArray(Language[]::new);

    }

    public record Language(String tag, String name) implements Serializable {
        @Override
        public int hashCode() {
            return Objects.hash(tag);
        }
    }

    @QueryMapping
    public List<CountryDialCode> countryDialCodes(@Argument @NotEmpty String lang) {
        final var lng = Locale.forLanguageTag(lang);
        return phoneNumberHelper.supportedRegions().stream()
                .map((x) -> {
                    return new CountryDialCode(this.phoneNumberHelper.countryCodeForRegion(x),
                            new Locale.Builder()
                                    .setRegion(x)
                                    .build().getDisplayCountry(lng),
                            x);
                })
                .toList();
    }

    public record CountryDialCode(int code, String name, String region) implements Serializable {
        @Override
        public int hashCode() {
            return Objects.hash(region);
        }
    }

    @QueryMapping
    public Set<Currency> currencies() {
        return Currency.getAvailableCurrencies();
    }

    @SchemaMapping(typeName = "Currency", field = "code")
    public String getCode(Currency it) {
        return it.getCurrencyCode();
    }

    @SchemaMapping(typeName = "Currency", field = "name")
    public String getName(Currency it, @Argument @NotEmpty String lang) {
        return it.getDisplayName(Locale.forLanguageTag(lang));
    }

    @SchemaMapping(typeName = "Currency", field = "fractionDigits")
    public int getFractionDigits(Currency it) {
        return it.getDefaultFractionDigits();
    }

    public static String password(String raw) throws InvalidProtocolBufferException {
        final var buf = Base64.getUrlDecoder().decode(raw);
        final var box = PasswordBox.parseFrom(buf);
        return box.getPayload();
    }

    public static String email(String raw) {
        return raw.trim().toLowerCase();
    }

    @Resource
    BuildProperties buildProperties;
    @Resource
    PhoneNumberHelper phoneNumberHelper;

}
