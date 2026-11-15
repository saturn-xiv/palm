package com.github.saturn_xiv.palm.marigold.plugins.portal.services;

import java.security.NoSuchAlgorithmException;
import java.time.Instant;
import java.time.ZoneId;
import java.util.Locale;
import java.util.UUID;

import org.springframework.context.MessageSource;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import com.github.saturn_xiv.palm.marigold.plugins.portal.Plugin;
import com.github.saturn_xiv.palm.marigold.plugins.portal.models.user.ByEmail;
import com.github.saturn_xiv.palm.marigold.plugins.portal.models.user.Item;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.EmailUserRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.GoogleOauth2UserRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.SmsUserRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.UserBanRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.UserContactRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.UserRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.WechatMiniProgramUserRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.WechatOauth2UserRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.utils.CasbinEnforcer;
import com.github.saturn_xiv.palm.marigold.plugins.portal.utils.Gravatar;

import jakarta.annotation.Resource;

@Service("portal.s.user")
public class UserService {

    @Transactional(rollbackFor = Throwable.class)
    public void install(String name, String email, String rawPassword, Locale lang, ZoneId timezone, String ip)
            throws NoSuchAlgorithmException {
        signUp(name, email, rawPassword, lang, timezone, ip);
        confirm(email, ip);
        var user = emailUserRepository.findByEmail(email).get().getUser();
        enforcer.addRoleForUser(user.getId(), CasbinEnforcer.administrator());
        logService.info(user, Plugin.class, ip, "Apply administrator role");
        enforcer.addRoleForUser(user.getId(), CasbinEnforcer.root());
        logService.info(user, Plugin.class, ip, "Apply root role");
    }

    @Transactional(rollbackFor = Throwable.class)
    public void signUp(String name, String email, String rawPassword, Locale lang, ZoneId timezone, String ip)
            throws NoSuchAlgorithmException {
        if (emailUserRepository.findByEmail(email).isPresent()) {
            throw new IllegalArgumentException(String.format("User %s is already existed.", email));
        }

        final var sn = UUID.randomUUID().toString();
        create(sn, name, lang, timezone);
        var user = userRepository.findBySn(sn).get();
        var emailUser = new ByEmail();
        emailUser.setSn(UUID.randomUUID().toString());
        emailUser.setUser(user);
        emailUser.setEmail(email);
        emailUser.setName(name);
        emailUser.setPassword(passwordEncoder.encode(rawPassword));
        emailUser.setAvatar(gravatar.image(email));
        emailUserRepository.save(emailUser);
        logService.info(user, Plugin.class, ip, "Sign up.");
    }

    public boolean verify(String password, String rawPassword) {
        return passwordEncoder.matches(rawPassword, password);
    }

    @Transactional(rollbackFor = Throwable.class)
    public void confirm(String email, String ip) {
        final var now = Instant.now();
        var it = emailUserRepository.findByEmail(email).get();
        it.setConfirmedAt(now);
        emailUserRepository.save(it);
        logService.info(it.getUser(), Plugin.class, ip, "Confirmed.");
    }

    private void create(String sn, String name, Locale lang, ZoneId timezone) {
        var it = new Item();
        it.setSn(sn);
        it.setName(name);
        it.setLang(lang.toLanguageTag());
        it.setTimezone(timezone.getId());
        userRepository.save(it);
    }

    @Resource
    LogService logService;
    @Resource
    UserRepository userRepository;
    @Resource
    EmailUserRepository emailUserRepository;
    @Resource
    GoogleOauth2UserRepository googleOauth2UserRepository;
    @Resource
    SmsUserRepository smsUserRepository;
    @Resource
    WechatMiniProgramUserRepository wechatMiniProgramUserRepository;
    @Resource
    WechatOauth2UserRepository wechatOauth2UserRepository;
    @Resource
    UserBanRepository userBanRepository;
    @Resource
    UserContactRepository userContactRepository;
    @Resource
    MessageSource messageSource;
    @Resource
    PasswordEncoder passwordEncoder;
    @Resource
    Gravatar gravatar;
    @Resource
    CasbinEnforcer enforcer;
}
