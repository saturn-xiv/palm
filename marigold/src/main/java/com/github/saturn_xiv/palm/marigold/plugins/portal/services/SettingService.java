package com.github.saturn_xiv.palm.marigold.plugins.portal.services;

import java.lang.reflect.Type;
import java.nio.charset.StandardCharsets;
import java.security.GeneralSecurityException;
import java.util.List;

import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.Setting;
import com.github.saturn_xiv.palm.marigold.plugins.portal.models.user.Item;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.SettingRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.utils.RandomUtil;
import com.github.saturn_xiv.palm.marigold.plugins.portal.utils.tink.Aead;
import com.google.gson.Gson;
import com.google.gson.reflect.TypeToken;

import jakarta.annotation.PostConstruct;
import jakarta.annotation.Resource;

@Service("portal.s.setting")
public class SettingService {

    @Transactional(rollbackFor = Throwable.class)
    public <T> void set(String key, T value, boolean encrypt) throws GeneralSecurityException {
        this.set(null, key, value, encrypt);
    }

    @Transactional(rollbackFor = Throwable.class)
    public <T> void set(Item user, String key, T value, boolean encrypt) throws GeneralSecurityException {
        var val = this.toValue(value, encrypt);
        var it = settingRepository.findByKeyAndUser(key, user);
        if (it.isPresent()) {
            var cur = it.get();
            cur.setValue(val);
            settingRepository.save(cur);
        } else {
            var cur = new Setting();
            cur.setKey(key);
            cur.setUser(user);
            cur.setValue(val);
            settingRepository.save(cur);
        }
    }

    private <T> Setting.Value toValue(T value, boolean encrypt) throws GeneralSecurityException {
        final var buf = this.gson.toJson(value).getBytes(StandardCharsets.UTF_8);
        if (encrypt) {
            final var associated = randomUtil.bytes(16);
            return new Setting.Value(this.aead.encrypt(buf, associated), associated);
        }
        return new Setting.Value(buf, null);
    }

    @Transactional(readOnly = true)
    public <T> T get(String key, Class<T> clazz) throws GeneralSecurityException {
        return this.get(null, key, clazz);
    }

    @Transactional(readOnly = true)
    public <T> T get(Item user, String key, Class<T> clazz) throws GeneralSecurityException {
        var buf = this.get(user, key);
        return this.gson.fromJson(new String(buf, StandardCharsets.UTF_8), clazz);
    }

    @Transactional(readOnly = true)
    public <T> List<T> getList(Item user, String key, Class<T> clazz) throws GeneralSecurityException {
        Type type = TypeToken.getParameterized(List.class, clazz).getType();
        var buf = this.get(user, key);
        return this.gson.fromJson(new String(buf, StandardCharsets.UTF_8), type);
    }

    @Transactional(readOnly = true)
    public <T> List<T> getList(String key, Class<T> clazz) throws GeneralSecurityException {
        return this.getList(null, key, clazz);
    }

    @Transactional(readOnly = true)
    public byte[] get(Item user, String key) throws GeneralSecurityException {
        var it = settingRepository.findByKeyAndUser(key, user).get();
        if (it.getValue().associated_data() == null) {
            return it.getValue().payload();
        }
        return this.aead.decrypt(it.getValue().payload(), it.getValue().associated_data());
    }

    @PostConstruct
    void init() {
        this.gson = new Gson();
    }

    @Resource
    RandomUtil randomUtil;
    @Resource
    Aead aead;
    @Resource
    SettingRepository settingRepository;
    private Gson gson;
}
