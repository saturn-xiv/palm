package com.github.saturn_xiv.palm.plugins.musa.helpers;

import jakarta.validation.constraints.NotNull;
import org.apache.thrift.TException;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Component;

import java.time.LocalDate;
import java.time.temporal.ChronoUnit;
import java.util.concurrent.TimeUnit;

@Component("palm.musa.helper.jwt")
public class JwtHelper {

    public String verify(@NotNull final String token, @NotNull final String issuer, @NotNull final String audience) throws TException {
        var jwt = loquat.jwt();
        final var it = jwt.verify(token, issuer, audience);
        return it.payload;
    }

    public String sign(@NotNull final String issuer, @NotNull final String subject, @NotNull final String audience, final int years, @NotNull final String payload) throws TException {
        var jwt = loquat.jwt();

        LocalDate today = LocalDate.now();
        LocalDate exp = today.plusYears(years);
        return jwt.sign(
                issuer, subject, audience,
                TimeUnit.SECONDS.convert(ChronoUnit.DAYS.between(today, exp), TimeUnit.DAYS),
                payload
        );
    }


    @Autowired
    LoquatClient loquat;


    private final static Logger logger = LoggerFactory.getLogger(JwtHelper.class);
}
