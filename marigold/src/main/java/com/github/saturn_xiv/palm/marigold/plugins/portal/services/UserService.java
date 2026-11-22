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
import com.github.saturn_xiv.palm.marigold.plugins.portal.models.user.ByGoogleOauth2;
import com.github.saturn_xiv.palm.marigold.plugins.portal.models.user.ByWechatMiniProgram;
import com.github.saturn_xiv.palm.marigold.plugins.portal.models.user.ByWechatOauth2;
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
import com.github.saturn_xiv.palm.marigold.wechat.responses.AccessTokenByCodeResponse;
import com.github.saturn_xiv.palm.marigold.wechat.responses.JsCode2SessionResponse;
import com.github.saturn_xiv.palm.marigold.wechat.responses.UserInfoResponse;
import com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse;
import com.google.api.client.auth.oauth2.TokenResponse;
import com.google.api.services.oauth2.model.Userinfo;

import jakarta.annotation.Resource;

@Service("portal.s.user")
public class UserService {

    @Transactional(rollbackFor = Throwable.class)
    public void signIn(String clientId, TokenResponse tokenResponse, Userinfo userInfo, String ip, Locale locale,
            ZoneId timezone) {
        {
            var cur = googleOauth2UserRepository.findByClientIdAndSub(clientId, userInfo.getId());
            if (cur.isPresent()) {
                var it = cur.get();
                if (!it.isAvailable()) {
                    throw new IllegalArgumentException();
                }
                it.merge(tokenResponse, userInfo);
                googleOauth2UserRepository.save(it);
            } else {
                var sn = UUID.randomUUID().toString();
                this.create(sn, userInfo.getName(), locale, timezone);
                var user = userRepository.findBySn(sn).get();
                var it = new ByGoogleOauth2();
                it.setUser(user);
                it.setClientId(clientId);
                it.setSn(UUID.randomUUID().toString());
                it.merge(tokenResponse, userInfo);
                googleOauth2UserRepository.save(it);
            }
        }
        var it = googleOauth2UserRepository.findByClientIdAndSub(clientId, userInfo.getId()).get();
        signIn(it.getUser(), it.id(), ip, UserIndexResponse.Item.Type.GOOGLE_OAUTH2);
    }

    @Transactional(rollbackFor = Throwable.class)
    public void signIn(String appId, JsCode2SessionResponse jsCode2Session, String ip, Locale locale,
            ZoneId timezone) {
        {
            var cur = wechatMiniProgramUserRepository.findByAppIdAndOpenId(appId, jsCode2Session.getOpenId());
            if (cur.isPresent()) {
                var it = cur.get();
                if (!it.isAvailable()) {
                    throw new IllegalArgumentException();
                }
                it.merge(jsCode2Session);
                wechatMiniProgramUserRepository.save(it);
            } else {
                var sn = UUID.randomUUID().toString();
                this.create(sn, "Anonymous", locale, timezone);
                var user = userRepository.findBySn(sn).get();
                var it = new ByWechatMiniProgram();
                it.setUser(user);
                it.setAppId(appId);
                it.setSn(UUID.randomUUID().toString());
                it.merge(jsCode2Session);
                wechatMiniProgramUserRepository.save(it);
            }
        }
        var it = wechatMiniProgramUserRepository.findByAppIdAndOpenId(appId, jsCode2Session.getOpenId()).get();
        signIn(it.getUser(), it.id(), ip, UserIndexResponse.Item.Type.WECHAT_MINI_PROGRAM);
    }

    @Transactional(rollbackFor = Throwable.class)
    public void signIn(String appId, AccessTokenByCodeResponse accessTokenByCode, UserInfoResponse info,
            String ip, Locale locale, ZoneId timezone) {
        {
            var cur = wechatOauth2UserRepository.findByAppIdAndOpenId(appId, accessTokenByCode.getOpenId());
            if (cur.isPresent()) {
                var it = cur.get();
                if (!it.isAvailable()) {
                    throw new IllegalArgumentException();
                }
                it.merge(accessTokenByCode, info);
                wechatOauth2UserRepository.save(it);
            } else {
                var sn = UUID.randomUUID().toString();
                this.create(sn, info.getNickname(), locale, timezone);
                var user = userRepository.findBySn(sn).get();
                var it = new ByWechatOauth2();
                it.setUser(user);
                it.setAppId(appId);
                it.setSn(UUID.randomUUID().toString());
                it.merge(accessTokenByCode, info);
                wechatOauth2UserRepository.save(it);
            }
        }
        var it = wechatOauth2UserRepository.findByAppIdAndOpenId(appId, accessTokenByCode.getOpenId()).get();
        signIn(it.getUser(), it.id(), ip, UserIndexResponse.Item.Type.WECHAT_OAUTH2);
    }

    @Transactional(rollbackFor = Throwable.class)
    public void signIn(Item user, String email, String ip) {
        signIn(user, ip, email, UserIndexResponse.Item.Type.EMAIL);
    }

    private void signIn(Item user, String ip, String code, UserIndexResponse.Item.Type type) {
        logService.info(user, Plugin.class, ip, String.format("sign in by (%s, %s)", code, type));
        user.setLastSignInAt(user.getCurrentSignInAt());
        user.setLastSignInIp(user.getCurrentSignInIp());
        user.setCurrentSignInIp(ip);
        user.setCurrentSignInAt(Instant.now());
        user.setSignInCount(user.getSignInCount() + 1);
        userRepository.save(user);
    }

    @Transactional(rollbackFor = Throwable.class)
    public void setWechatOauth2UseNickname(String appId, String openId, String nickname) {
        var it = wechatOauth2UserRepository.findByAppIdAndOpenId(appId, openId).get();
        it.setNickname(nickname);
        wechatOauth2UserRepository.save(it);
    }

    @Transactional(rollbackFor = Throwable.class)
    public void destroyWechatOauth2User(String appId, String openId, String ip, String reason) {
        var it = wechatOauth2UserRepository.findByAppIdAndOpenId(appId, openId).get();
        wechatOauth2UserRepository.delete(it);
        logService.info(it.getUser(), Plugin.class, ip, String.format("Destroyed(%s)", reason));
    }

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
